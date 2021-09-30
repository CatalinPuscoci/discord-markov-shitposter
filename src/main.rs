use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;
use markov_strings::*;
use std::env;
use std::fs::File;
use rand::prelude::*;

struct GeneratedMessage;
struct AttachmentVec;

impl TypeMapKey for GeneratedMessage {
    type Value = Arc<RwLock<Markov>>;
}
impl TypeMapKey for AttachmentVec {
    type Value = Arc<RwLock<Vec<String>>>;
}
#[group]
#[commands(poza,coinflip,cοinflip)]
struct General;
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx:Context, msg:Message){
        if  msg.content.contains("<@!887794426011873361>") || msg.content.contains("<@887794426011873361>"){
            let data_read = &ctx.data.read().await;
            let markov = data_read.get::<GeneratedMessage>().unwrap().read().await;
            let gen = markov.generate().unwrap();
            println!("{:?}",gen);
            msg.reply(&ctx, gen.text).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let file = File::open("markov_saved.json").unwrap();
    let attachmentfile = File::open("attachments.json").unwrap();
    let importdata:ImportExport = serde_json::from_reader(file).unwrap();
    let attachmentvec:Vec<String> = serde_json::from_reader(attachmentfile).unwrap();
    //let mut markov = Markov::from_export(data);
    //    markov
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<GeneratedMessage>(Arc::new(RwLock::new(Markov::from_export(importdata))));
        data.insert::<AttachmentVec>(Arc::new(RwLock::new(attachmentvec)));
        let mut a = data.get::<GeneratedMessage>().unwrap().write().await;
        a.set_filter(|r| {
            // A minimal relative score and number of references
            // The thresholds are relative to your input
            r.score <=50 && r.score >=5 && r.refs.len() > 3
                && r.text.len() <= 600
        })
        .set_max_tries(10000);
        }
    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn poza(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;
    //let markov = data_read.get::<GeneratedMessage>().unwrap().read().await;
    let attachvec = data_read.get::<AttachmentVec>().unwrap().read().await;
    let mut attach:Vec<&String> = attachvec.choose_multiple(&mut rand::thread_rng(), 1).collect();
    let pic = attach.pop().unwrap();
    println!("{:?}",pic);
    msg.reply(ctx, pic).await?;
    Ok(())
}

#[command]
async fn coinflip(ctx: &Context, msg: &Message) -> CommandResult {
    let outcomes = vec!["cap".to_string(),"pajura".to_string()];
    let mut outcome:Vec<&String> = outcomes.choose_multiple(&mut rand::thread_rng(), 1).collect();
    let answer = outcome.pop().unwrap();
    println!("{}",answer);
    msg.reply(ctx,answer).await?;
    Ok(())
}

#[command]
async fn cοinflip(ctx: &Context, msg: &Message) -> CommandResult {
    println!("{}","cap");
    msg.reply(ctx,"cap".to_string()).await?;
    Ok(())
}

