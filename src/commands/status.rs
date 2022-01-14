use serenity::model::gateway::Activity;
use serenity::model::user::OnlineStatus;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
        
#[command] 
pub async fn status(ctx: &Context, msg: &Message) -> CommandResult { 
    if let Some(act) = msg.content.strip_prefix("~status"){
    let act=act.trim();
    let activity = Activity::playing(act);
    
        let status = OnlineStatus::Idle;
        ctx.set_presence(Some(activity), status).await;
        msg.channel_id.say(&ctx.http, format!("Set Status to {}",act)).await?;
    }
else {
        msg.channel_id.say(&ctx.http, format!("Failed to change Status")).await?;
    }
    Ok(())
}