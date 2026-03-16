use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::env;
use twitch::fexbot::{FexBot, FexBotConfig};

#[tokio::main]
async fn main() -> Result<(), eyre::Report> {
    // Setup terminal display of tracing logs
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    // Enable tracing to display eyre errors
    color_eyre::install()?;

    // Setup .env processing and extract values
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID is not set in .env file");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET is not set in .env file");
    let refresh_token = env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN is not set in .env file");
    let bot_user_id = env::var("BOT_USER_ID").expect("BOT_USER_ID is not set in .env file");

    // Create the database connection and run migrations if necessary
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    // Create the bot
    let fexbot = FexBot::new(FexBotConfig {
            client_id,
            client_secret,
            refresh_token,
            bot_user_id,
    }).await?;

    // Run the bot in its own future
    let bot_task = async move {
        fexbot.start().await
    };

    // Run the application
    let app_task = async move {
        api::start(api::AppConfig {
            conn,
            host,
            port,
        }).await
    };


    futures::future::try_join(bot_task, app_task).await?;
    Ok(())
}
