use serenity::model::gateway::Activity;
use serenity::model::user::OnlineStatus;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
        
#[command] 
pub async fn status(ctx: &Context, msg: &Message) -> CommandResult { 
    let activity = Activity::playing(&msg.content);
        let status = OnlineStatus::Idle;
        ctx.set_presence(Some(activity), status).await;
    Ok(())
}