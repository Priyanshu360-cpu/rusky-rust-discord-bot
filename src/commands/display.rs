use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;




#[command]
pub async fn display(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
let member:User;
if args.is_empty(){
    member=msg.author.id.to_user(&ctx).await.unwrap();
}
else{
    member = args.single::<id::UserId>().unwrap().to_user(&ctx).await.unwrap();
}
    msg.channel_id.send_message(ctx, |m| m.embed(|e| e.title("User Avatar").description("Avatar below").colour(0xff0000).image(member.face()))).await?;

    Ok(())
}