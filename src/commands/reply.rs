use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command] 
pub async fn reply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult { 
    msg.reply(ctx, &msg.content).await?;
    Ok(())
}