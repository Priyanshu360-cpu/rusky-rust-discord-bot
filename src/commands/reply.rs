use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command] 
pub async fn reply(ctx: &Context, msg: &Message) -> CommandResult { 
    msg.reply(ctx, &msg.content).await?;
    Ok(())
}