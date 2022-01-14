use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::model::Permissions;

struct Handler;

#[command]
pub async fn display(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    msg.channel_id.send_message(ctx, |m| m.embed(|e| e.title("User Avatar").description(&msg.author).colour(0xff0000).image(&msg.author.face()))).await?;

    Ok(())
}