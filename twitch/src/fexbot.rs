use std::sync::Arc;
use tokio::sync::Mutex;
use twitch_api::{
    client::ClientDefault,
    eventsub::{self, Event, Message, Payload},
    HelixClient,
};
use twitch_oauth2::{TwitchToken as _, UserToken};
use eyre::WrapErr;

use crate::websocket;

pub struct FexBotConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub bot_user_id: String,
}

pub struct FexBot {
    pub client: HelixClient<'static, reqwest::Client>,
    pub token: Arc<Mutex<twitch_oauth2::UserToken>>,
    pub bot_id: twitch_api::types::UserId,
}

impl FexBot {
    pub async fn new(config: FexBotConfig) -> Result<Self, eyre::Report> {
        let client: HelixClient<reqwest::Client> = twitch_api::HelixClient::with_client(
            ClientDefault::default_client_with_name(Some("FexBots".parse()?))?,
        );
        let client_id = twitch_oauth2::ClientId::new(config.client_id);
        let client_secret = twitch_oauth2::ClientSecret::new(config.client_secret);
        let refresh_token = twitch_oauth2::RefreshToken::new(config.refresh_token);

        let token = twitch_oauth2::UserToken::from_refresh_token(
            &client,
            refresh_token,
            client_id,
            client_secret
        ).await?;
        let token = Arc::new(Mutex::new(token));

        Ok(FexBot {
            client,
            token,
            bot_id: twitch_api::types::UserId::new(config.bot_user_id),
        })
    }

    pub async fn start(&self) -> Result<(), eyre::Report> {
        // To make a connection to the chat we need to use a websocket connection.
        // This is a wrapper for the websocket connection that handles the reconnects and handles all messages from eventsub.
        let websocket = websocket::ChatWebsocketClient {
            session_id: None,
            token: self.token.clone(),
            client: self.client.clone(),
            connect_url: twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
            chats: vec![twitch_api::types::UserId::new("68411561".into())],
        };
        let refresh_token = async move {
            let token = self.token.clone();
            let client = self.client.clone();
            // We check constantly if the token is valid.
            // We also need to refresh the token if it's about to be expired.
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            loop {
                interval.tick().await;
                let mut token = token.lock().await;
                if token.expires_in() < std::time::Duration::from_secs(60) {
                    token
                        .refresh_token(&self.client)
                        .await
                        .wrap_err("couldn't refresh token")?;
                }
                token
                    .validate_token(&client)
                    .await
                    .wrap_err("couldn't validate token")?;
            }
            #[allow(unreachable_code)]
            Ok(())
        };
        let ws = websocket.run(|e, ts| async { self.handle_event(e, ts).await });
        futures::future::try_join(ws, refresh_token).await?;
        Ok(())
    }
    async fn handle_event(
        &self,
        event: Event,
        timestamp: twitch_api::types::Timestamp,
    ) -> Result<(), eyre::Report> {
        let token = self.token.lock().await;
        match event {
            Event::ChannelChatMessageV1(Payload {
                message: Message::Notification(payload),
                subscription,
                ..
            }) => {
                tracing::debug!(
                    "[{}] {}: {}",
                    timestamp, payload.chatter_user_name, payload.message.text
                );
                if let Some(command) = payload.message.text.strip_prefix("!") {
                    let mut split_whitespace = command.split_whitespace();
                    let command = split_whitespace.next().unwrap();
                    let rest = split_whitespace.next();

                    self.command(&payload, &subscription, command, rest, &token)
                        .await?;
                }
            }
            Event::ChannelChatNotificationV1(Payload {
                message: Message::Notification(payload),
                ..
            }) => {
                tracing::debug!(
                    "[{}] {}: {}",
                    timestamp,
                    match &payload.chatter {
                        eventsub::channel::chat::notification::Chatter::Chatter {
                            chatter_user_name: user,
                            ..
                        } => user.as_str(),
                        _ => "anonymous",
                    },
                    payload.message.text
                );
            }
            _ => {}
        }
        Ok(())
    }

    async fn command(
        &self,
        payload: &eventsub::channel::ChannelChatMessageV1Payload,
        subscription: &eventsub::EventSubscriptionInformation<
            eventsub::channel::ChannelChatMessageV1,
        >,
        command: &str,
        _rest: Option<&str>,
        token: &UserToken,
    ) -> Result<(), eyre::Report> {
        tracing::info!("Command: {}", command);
        self.client
            .send_chat_message_reply(
                &subscription.condition.broadcaster_user_id,
                &subscription.condition.user_id,
                &payload.message_id,
                command,
                token,
            )
            .await?;
        Ok(())
    }
}
