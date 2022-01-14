use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {

    msg.channel_id.send_message(ctx, |m| m.embed(|e| e.title("Ping").description("Rust Framework Ping - 23ms").field(
        "Discord Api ",
        "Ping - 12ms",
        false,
    ).colour(0xff0000))).await?;

    
    Ok(())
}