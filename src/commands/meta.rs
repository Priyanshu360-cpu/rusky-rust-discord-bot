use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::model::channel::Embed;
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let embed = Embed::fake(|e| {
        e.title("Ping").description("Bot Ping - 23ms").field(
            "Discord Api Ping ",
            "- 12ms",
            false,
        )
    });
        
    msg.channel_id.say(&ctx.http, embed).await?;
    Ok(())
}