use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command] 
pub async fn table(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult { 
    let one = args.single::<i64>()?;
    let two = args.single::<i64>()?;
   
    for x in 1..two+1 {
        let display = format!("{}X{}={}", one,x,one*x);
        msg.channel_id.say(&ctx.http, display).await?;
    }    
    Ok(())
}