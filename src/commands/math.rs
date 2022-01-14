use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn math(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>()?;
    let car=args.single::<char>()?;
    let two = args.single::<f64>()?;
let finalo = match car{
    '+'=>one+two,
    '*'=>one*two,
    '-'=>one-two,
    '/'=>one/two,
    _=>0.0,
};
    msg.channel_id.say(&ctx.http, finalo).await?;

    Ok(())
}