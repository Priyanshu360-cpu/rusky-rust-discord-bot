

use serenity::{
    client::{ Context},
    framework::{
        standard::{
            macros::{command},
            Args, CommandResult,
        },
    },
    model::{channel::Message},
    Result as SerenityResult,
};

use lavalink_rs::{LavalinkClient};
use serenity::prelude::*;


struct Lavalink;

impl TypeMapKey for Lavalink {
    type Value = LavalinkClient;
}

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.message().to_string();

    let guild_id = match ctx.cache.guild_channel(msg.channel_id).await {
        Some(channel) => channel.guild_id,
        None => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Error finding channel info")
                    .await,
            );

            return Ok(());
        }
    };

    let lava_client = {
        let data = ctx.data.read().await;
        data.get::<Lavalink>().unwrap().clone()
    };

    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(_handler) = manager.get(guild_id) {
        let query_information = lava_client.auto_search_tracks(&query).await?;

        if query_information.tracks.is_empty() {
            check_msg(
                msg.channel_id
                    .say(&ctx, "Could not find any video of the search query.")
                    .await,
            );
            return Ok(());
        }

        if let Err(why) = &lava_client
            .play(guild_id, query_information.tracks[0].clone())
            .queue()
            .await
        {
            msg.reply(ctx,format!("{}", why));
            return Ok(());
        };
        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    format!(
                        "Added to queue: {}",
                        query_information.tracks[0].info.as_ref().unwrap().title
                    ),
                )
                .await,
        );
    } else {
        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    "Use `~join` first, to connect the bot to your current voice channel.",
                )
                .await,
        );
    }

    Ok(())
}
