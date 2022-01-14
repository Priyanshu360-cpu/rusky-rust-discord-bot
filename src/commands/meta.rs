use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let ping = std::time::SystemTime::now().duration_since(msg.timestamp.into())?.as_millis();

    msg.channel_id.send_message(ctx, |m| m.embed(|e| e.title("Ping").description(format!("Rust Framework Ping - {}ms",ping)).field(
        "Discord Api ",
        format!("Ping - {}ms",ping),
        false,
    ).colour(0xff0000))).await?;

    
    Ok(())
}