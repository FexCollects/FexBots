use db::mutation::Mutation;
use eyre::WrapErr;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use std::sync::mpsc::channel;
use tokio::sync::Mutex;
use twitch_api::{
    HelixClient,
    client::ClientDefault,
    eventsub::{self, Event, Message, Payload},
};
use twitch_oauth2::TwitchToken as _;

use crate::commands::*;
use crate::websocket;

pub struct FexBotConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub bot_user_id: String,
    pub conn: DatabaseConnection,
}

pub struct FexBot {
    pub client: HelixClient<'static, reqwest::Client>,
    pub token: Arc<Mutex<twitch_oauth2::UserToken>>,
    pub bot_id: twitch_api::types::UserId,
    pub conn: DatabaseConnection,
}

struct SendMessageData {
    broadcaster_id: twitch_api::types::UserId,
    chatter_id: twitch_api::types::UserId,
    message_id: twitch_api::types::MsgId,
    message: String,
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
            client_secret,
        )
        .await?;
        let token = Arc::new(Mutex::new(token));

        Ok(FexBot {
            client,
            token,
            bot_id: twitch_api::types::UserId::new(config.bot_user_id),
            conn: config.conn,
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
            chats: vec![
                twitch_api::types::UserId::new("68411561".into()), // FexCollects
                twitch_api::types::UserId::new("176723607".into()), // SBCoop
                twitch_api::types::UserId::new("106239207".into()), // Magnemite
                twitch_api::types::UserId::new("861073341".into()), // Yarnity
                twitch_api::types::UserId::new("29002848".into()), // Kwikpanik
                twitch_api::types::UserId::new("103539171".into()), // BigWiggins
                twitch_api::types::UserId::new("214333642".into()), // LegendLinke
                twitch_api::types::UserId::new("83193553".into()), // skyfishjack
            ],
        };

        // Set up 3 tasks
        //   - refresh_token which periodically generates a new token as needed
        //   - send_message which consumes queued messages and posts them to twitch chat
        //   - ws which listens on the ws connection, runs commands and queues responses

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

        // message queue
        let (tx, rx) = channel::<SendMessageData>();

        let send_message = async move {
            let token = self.token.clone();
            let client = self.client.clone();
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            loop {
                interval.tick().await;
                loop {
                    match rx.try_recv() {
                        Err(_) => break,
                        Ok(msg) => {
                            let token = token.lock().await.clone();
                            let _ = client
                                .send_chat_message_reply(
                                    msg.broadcaster_id,
                                    msg.chatter_id,
                                    msg.message_id,
                                    &*msg.message,
                                    &token,
                                )
                                .await
                                .inspect_err(|e| tracing::warn!("Failed to send message: {e}"));
                        }
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok(())
        };

        let ws = websocket.run(|e, ts| async {
            //let token = self.token.clone();
            //let client = self.client.clone();
            self.handle_event(e, ts, tx.clone()).await
        });

        futures::future::try_join3(ws, refresh_token, send_message).await?;
        Ok(())
    }
    async fn handle_event(
        &self,
        event: Event,
        timestamp: twitch_api::types::Timestamp,
        msg_queue: std::sync::mpsc::Sender<SendMessageData>,
    ) -> Result<(), eyre::Report> {
        match event {
            Event::ChannelChatMessageV1(Payload {
                message: Message::Notification(payload),
                subscription,
                ..
            }) => {
                let conn = self.conn.clone();
                tokio::spawn(async move {
                    tracing::debug!(
                        "[{}] {}: {}",
                        timestamp,
                        payload.chatter_user_name,
                        payload.message.text
                    );

                    let _ = FexBot::command(conn, &payload, &subscription, msg_queue)
                        .await
                        .inspect_err(|e| tracing::warn!("Failed to run command: {}", e));
                });
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
        conn: DatabaseConnection,
        payload: &eventsub::channel::ChannelChatMessageV1Payload,
        subscription: &eventsub::EventSubscriptionInformation<
            eventsub::channel::ChannelChatMessageV1,
        >,
        msg_queue: std::sync::mpsc::Sender<SendMessageData>,
    ) -> Result<(), eyre::Report> {
        let mut body = payload.message.text.clone();
        let chatter_id = payload.chatter_user_id.as_str();
        let int_chatter_id = chatter_id.parse::<i64>()?;

        // Never respond to ourselves (1215874701 -> FexBots user id)
        if chatter_id == "1215874701" {
            return Ok(());
        }

        // Special case the commands that look for a specific substring in the
        // message body
        if body.contains("he'll") {
            body = "!he'll".into();
        }

        if body.contains("honk") {
            body = "!honk".into();
        }

        // Handle the normal !command commands
        if !body.starts_with("!") {
            return Ok(());
        }

        let Some(command) = Command::find(&body) else {
            return Ok(());
        };

        let _ = Mutation::get_or_create_chatter(
            &conn,
            int_chatter_id,
            payload.chatter_user_name.as_str().into(),
        )
        .await?;

        // Get the response
        let Some(message) = command
            .run(
                body.clone(),
                int_chatter_id,
                payload.broadcaster_user_id.as_str(),
                conn,
            )
            .await
        else {
            return Ok(());
        };

        let msg = SendMessageData {
            broadcaster_id: subscription.condition.broadcaster_user_id.clone(),
            chatter_id: subscription.condition.user_id.clone(),
            message_id: payload.message_id.clone(),
            message,
        };

        msg_queue
            .send(msg)
            .wrap_err_with(|| format!("Failed to send reponse for command {}", body))
    }
}
