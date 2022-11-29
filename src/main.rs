use rocket::config::Config;
use std;
use std::env;
use std::net::IpAddr;
use std::net::Ipv4Addr;

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

#[post("/", data = "<msg>")]
async fn discord(msg: &str) -> String {
    let client = reqwest::Client::new();
    let params = [("username", "Alertisc"), ("content", msg)];
    let args = env::args().collect::<Vec<String>>();
    let url = args[1].as_str();
    let res = client.post(url).form(&params).send().await;

    format!("👋 Msg to push inside discord is : |{}|", msg)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    if env::args().len() < 2 {
        println!("STOP! Webhooks url missing, please use `./binary https://example.com`");
        std::process::exit(-1);
    }
    let mut config = Config::default();
    let any_network = Ipv4Addr::new(0, 0, 0, 0);
    config.address = IpAddr::V4(any_network);

    rocket::custom(config)
        .mount("/discord", routes![discord])
        .launch()
        .await;

    Ok(())
}
