#![allow(dead_code)]

use std::error::Error;
use novaro_bot::web::{get_ids_by_name};

use tokio_core::reactor::Core;
use std::env;
use telegram_bot::{Api, UpdateKind, MessageKind};
use futures::prelude::*;
use telegram_bot::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {

    let mut core = Core::new().expect("Failed to create Core");
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not found");
    let api = Api::configure(token).build(core.handle()).expect("Could not build Api type");

    let streams = api.stream().for_each(|update: telegram_bot::types::Update| {
        match update.kind {
            UpdateKind::Message(msg) => {
                match &msg.kind {
                    MessageKind::Text { data, entities: _entities } => {
                        let ids = get_ids_by_name(data);
                        // IdInfo implementa Display
                        println!("{:?}", ids);
                        api.spawn(msg.text_reply(format!("{:?}", ids)));
                    }
                    _ => {}
                }
            },
            _ => {}
        }

        Ok(())
    });

    core.run(streams).unwrap();
    Ok(())
}

