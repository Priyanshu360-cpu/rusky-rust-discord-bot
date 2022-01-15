use serenity::client::Context;


use serenity::{
    framework::{
        standard::{
             CommandResult,
            macros::{command},
        },
    },
    model::{channel::Message},
    Result as SerenityResult,
};
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}
