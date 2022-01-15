mod commands;

use std::{collections::HashSet, env, sync::Arc};

use commands::{math::*, meta::*, owner::*,reply::*,display::*,table::*,status::*,chatbot::*};
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use tracing::{error, info};
use sqlx::postgres::PgPoolOptions;
use lavalink_rs::{gateway::*, model::*, LavalinkClient};
use songbird::SerenityInit;
struct Bot {
    database: sqlx::SqlitePool,
}
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
struct LavalinkHandler;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self,ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        use serenity::model::gateway::Activity;
        use serenity::model::user::OnlineStatus;

        let activity = Activity::playing("Rust Coz why not");
        let status = OnlineStatus::Idle;
        ctx.set_presence(Some(activity), status).await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

}
#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {
    async fn track_start(&self, _client: LavalinkClient, event: TrackStart) {
        info!("Track started!\nGuild: {}", event.guild_id);
    }
    async fn track_finish(&self, _client: LavalinkClient, event: TrackFinish) {
        info!("Track finished!\nGuild: {}", event.guild_id);
    }
}

#[group]
#[commands(math, ping, quit, reply, display, table,status,chatbot)]
struct General;

#[tokio::main]
async fn main() {

    dotenv::dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);
   
   
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix("~")).group(&GENERAL_GROUP);
        let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");
        sqlx::migrate!("./migrations").run(&database).await.expect("Couldn't run database migrations");
  

    let bot = Bot {
        database,
    };
    let bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let lava_client = LavalinkClient::builder(bot_id.0)
    .set_host("localhost:2333")
    .set_password(
        env::var("LAVALINK_PASSWORD").unwrap_or_else(|_| "youshallnotpass".to_string()),
    )
    .build(LavalinkHandler)
    .await;


    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client"); 
        client.start().await.expect("err");


    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}