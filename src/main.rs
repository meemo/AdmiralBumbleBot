#![warn(clippy::all, clippy::needless_pass_by_value)]
#![feature(drain_filter)]

use handler::Handler;

use dotenv::dotenv;
use serenity::Client;

#[macro_use]
mod macros;
mod commands;
mod handler;
mod logging;
mod storage;

const CACHE_SIZE: usize = 100;
const STORAGE_PATH: &str = "storage";

fn main() {
    dotenv().ok();

    let mut client = Client::new(
        get_env!("ABB_TOKEN"),
        Handler {
            storage: sled::open(STORAGE_PATH).expect("Error opening storage database"),
        },
    )
    .expect("Error creating client");

    {
        let mut cache = client.cache_and_http.cache.write();
        cache.settings_mut().max_messages(CACHE_SIZE);
    }

    if let Err(e) = client.start() {
        eprintln!("Error starting client: {}", e);
    }
}
