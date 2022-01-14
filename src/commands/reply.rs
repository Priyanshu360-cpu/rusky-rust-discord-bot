use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command] 
pub async fn reply(ctx: &Context, msg: &Message) -> CommandResult { 
    if let Some(act) = msg.content.strip_prefix("~reply"){
        let act=act.trim();
    msg.reply(ctx, act).await?;
    }else{
        msg.reply(ctx,format!("Some error occured")).await?;
    }
    Ok(())
}