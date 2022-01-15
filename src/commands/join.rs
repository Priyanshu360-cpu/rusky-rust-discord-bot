
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
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };
   
    let manager = songbird::get(ctx).await
    .expect("Songbird Voice client placed in at initialisation.").clone();

let _handler = manager.join(guild_id, connect_to).await;
   

    Ok(())
}
