use std::env;
use std::net::IpAddr;
use std::net::Ipv4Addr;

use rocket::config::Config;
use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("msg => {:?}", msg.channel_id);
        if msg.content == "!ping" {
            println!("Shard {}", ctx.shard_id);

            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[macro_use]
extern crate rocket;

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
    #[field(value = "ru")]
    #[field(value = "ру")]
    Russian,
}

#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}

// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

// Try visiting:
//   http://127.0.0.1:8000/hello/мир
#[get("/мир")]
fn mir() -> &'static str {
    "Привет, мир!"
}

// Try visiting:
//   http://127.0.0.1:8000/wave/Rocketeer/100
#[get("/<name>/<age>")]
fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

#[post("/", data = "<msg>")]
async fn discord(msg: &str) -> String {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // The total number of shards to use. The "current shard number" of a
    // shard - that is, the shard it is assigned to - is indexed at 0,
    // while the total shard count is indexed at 1.
    //
    // This means if you have 5 shards, your total shard count will be 5, while
    // each shard will be assigned numbers 0 through 4.
    // if let Err(why) = client.start_shards(2).await {
    //     println!("Client error: {:?}", why);
    // }
    let chan = ChannelId(1035305332186030120);

    let _ = chan
        .send_message(client.cache_and_http.http.clone(), |m| {
            m.content(msg).embed(|e| {
                e.title("Alert_name").description(format!(
                    "
                    🚨 Alert from Rust API (POST)\nMessage: |`{}`|",
                    msg
                ))
            })
        })
        .await;

    format!("👋 Msg to push inside discord is : |{}|", msg)
}

#[get("/?<lang>&<opt..>")]
fn hello(lang: Option<Lang>, opt: Options<'_>) -> String {
    let mut greeting = String::new();
    if opt.emoji {
        greeting.push_str("👋 ");
    }

    match lang {
        Some(Lang::Russian) => greeting.push_str("Привет"),
        Some(Lang::English) => greeting.push_str("Hello"),
        None => greeting.push_str("Hi"),
    }

    if let Some(name) = opt.name {
        greeting.push_str(", ");
        greeting.push_str(name);
    }

    greeting.push('!');
    greeting
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment variable : DISCORD_TOKEN use export=token");
    let mut config = Config::default();
    let any_network = Ipv4Addr::new(0, 0, 0, 0);
    config.address = IpAddr::V4(any_network);

    rocket::custom(config)
        .mount("/discord", routes![discord])
        .launch()
        .await;

    // let _rocket = rocket::build()
    //     .mount("/discord", routes![discord])
    //     .launch()
    //     .await?;

    Ok(())
}
