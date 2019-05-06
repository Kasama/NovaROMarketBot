#![allow(dead_code)]
mod telegram;

use std::error::Error;
use std::env;

use telegram::run_bot;

fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not found");
    run_bot(token)
}
