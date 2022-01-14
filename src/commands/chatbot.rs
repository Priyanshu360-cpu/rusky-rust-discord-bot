use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use reqwest::Url;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct Reply{
    message: String,
}
#[command]
async fn chatbot(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(act) = msg.content.strip_prefix("~chatbot"){
        let act=act.trim();
        
        let url = format!(
            "https://api.affiliateplus.xyz/api/chatbot?message={}&botname=Rusky&ownername=Priyanshu",
            act
        );
        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<Reply>().await?;
        msg.reply(ctx,res.message).await?;
    }
    else{
     msg.reply(ctx,"Some error occured").await?;   
    }

    
    Ok(())
}