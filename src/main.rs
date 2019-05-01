#![allow(dead_code)]

use novaro_bot::web::{get_ids_by_name, get_market_entries};
use std::error::Error;

use futures::prelude::*;
use std::env;
use telegram_bot::prelude::*;
use telegram_bot::{Api, InlineKeyboardButton, InlineKeyboardMarkup, Message, MessageKind, UpdateKind, SendMessage};
use tokio_core::reactor::Core;

fn main() -> Result<(), Box<dyn Error>> {
    let mut core = Core::new().expect("Failed to create Core");
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not found");
    let api = Api::configure(token)
        .build(core.handle())
        .expect("Could not build Api type");

    let streams = api
        .stream()
        .for_each(|update: telegram_bot::types::Update| {
            dbg!(&update);
            match update.kind {
                UpdateKind::Message(msg) => {
                    match &msg.kind {
                        MessageKind::Text {
                            data,
                            entities: _entities,
                        } => {
                            let ids = get_ids_by_name(data);
                            // https://docs.rs/telegram-bot/0.6.1/telegram_bot/macro.reply_markup.html
                            let mut message = SendMessage::new(msg.chat, "Ola querido amiguinho");

                            // TODO: Fix this API design, why must this be mutable? How could we build it in a better way?
                            let mut inline_keyboard_markup = InlineKeyboardMarkup::new();
                            ids.into_iter().for_each(|idinfo| {
                                let inline_keyboard_button = InlineKeyboardButton::callback(
                                    &idinfo.name,
                                    &idinfo.id.to_string(),
                                );
                                inline_keyboard_markup.add_row(vec![inline_keyboard_button]);
                            });

                            message.reply_markup(inline_keyboard_markup);
                            api.spawn(message);
                        }
                        _ => {}
                    }
                }
                UpdateKind::CallbackQuery(query) => {
                    let idinfo_id = query.data.parse::<i32>().ok();
                    match idinfo_id {
                        None => {
                            //TODO: todo!("Send message to user")
                        }
                        Some(id) => {
                            let market_entries = get_market_entries(id);
                            market_entries.first().map(|entry| {
                                let ans = query.answer(format!("{:?}", entry));
                                api.spawn(ans);
                            });
                        }
                    }
                }
                _ => {}
            }

            Ok(())
        });

    core.run(streams).unwrap();
    Ok(())
}
