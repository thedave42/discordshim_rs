mod embedbuilder;
mod healthcheck;
mod messages;
mod server;
mod test;

use async_std::sync::RwLock;
use log::error;
//use serenity::client::EventHandler;
// use serenity::Client;
use std::env;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::server::Server;
// use serenity::async_trait;

use crate::healthcheck::healthcheck;
// use serenity::model::channel::Message;
// use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::GatewayIntents;
// use tokio::task;

use poise::serenity_prelude as serenity;

struct Handler {
    healthcheckchannel: ChannelId,
    server: Arc<RwLock<Server>>,
}
// Custom user data passed to all command functions
pub struct Data {
    poise_mentions: AtomicU32,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

// #[async_trait]
// impl EventHandler for Handler {
//     async fn message(&self, ctx: Context, new_message: Message) {
//         // Check for statistics messages
//         if new_message.channel_id == self.healthcheckchannel {
//             if new_message.content == "/stats" {
//                 self.server
//                     .read()
//                     .await
//                     .send_stats(new_message.channel_id, ctx.clone())
//                     .await;
//             }
//         }

//         // Check for health check message.
//         if new_message.is_own(ctx.cache) {
//             if new_message.channel_id == self.healthcheckchannel {
//                 if new_message.embeds.len() != 1 {
//                     return;
//                 }
//                 let embed1 = new_message.embeds.get(0).unwrap();
//                 if embed1.title.is_none() {
//                     return;
//                 }
//                 let flag = embed1.title.as_ref().unwrap().clone();
//                 self.server
//                     .read()
//                     .await
//                     .send_command(new_message.channel_id, new_message.author.id, flag)
//                     .await;
//                 return;
//             }
//             return;
//         }

//         if new_message.is_private() {
//             return;
//         }
//         // Process all other messages as normal.
//         self.server
//             .read()
//             .await
//             .send_command(
//                 new_message.channel_id,
//                 new_message.author.id,
//                 new_message.content,
//             )
//             .await;
//         for attachment in new_message.attachments {
//             let filedata = attachment.download().await.unwrap();
//             self.server
//                 .read()
//                 .await
//                 .send_file(
//                     new_message.channel_id,
//                     new_message.author.id,
//                     attachment.filename,
//                     filedata,
//                 )
//                 .await;
//         }
//     }

//     async fn ready(&self, _ctx: Context, _ready: Ready) {
//         let ctx = Arc::new(_ctx);
//         task::spawn(run_server(ctx, self.server.clone()));
//     }
// }

// async fn run_server(_ctx: Arc<Context>, server: Arc<RwLock<Server>>) {
//     server.read().await.run(_ctx).await
// }

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.content.to_lowercase().contains("poise")
                && new_message.author.id != ctx.cache.current_user().id
            {
                let old_mentions = data.poise_mentions.fetch_add(1, Ordering::SeqCst);
                new_message
                    .reply(
                        ctx,
                        format!("Poise has been mentioned {} times", old_mentions + 1),
                    )
                    .await?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();

    for argument in env::args() {
        match argument.to_lowercase().as_str() {
            "serve" => {
                exit(serve().await);
            }
            "healthcheck" => {
                exit(healthcheck().await);
            }
            &_ => {}
        }
    }
    error!("Usage: TODO");
}

async fn serve() -> i32 {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    // let framework = StandardFramework::new().configure(|c| c.prefix("~"));
    let framework = poise::Framework::builder()
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    poise_mentions: AtomicU32::new(0),
                })
            })
        })
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .build();

    // let channel_id: u64 = env::var("HEALTH_CHECK_CHANNEL_ID")
    //     .expect("channel id")
    //     .parse()
    //     .unwrap();

    // let handler = Handler {
    //     healthcheckchannel: ChannelId::new(channel_id),
    //     server: Arc::new(RwLock::new(Server::new())),
    // };



    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();

    return 0;
}
