use novaro_bot::web::{get_ids_by_name, get_market_entries};

use std::error::Error;
use futures::prelude::*;
use telegram_bot::{
    Api,
    EditMessageText,
    InlineKeyboardButton,
    InlineKeyboardMarkup,
    Message,
    MessageKind,
    UpdateKind,
    SendMessage
};
use tokio_core::reactor::Core;

pub fn run_bot(token: String) -> Result<(), Box<dyn Error>> {
    let mut core = Core::new().expect("Failed to create Core");
    let api = Api::configure(token)
        .build(core.handle())
        .expect("Could not build Api type");

    let streams = api
        .stream()
        .for_each(|update: telegram_bot::types::Update| {
            match update.kind {
                UpdateKind::Message(msg) => handle_market_search(msg, &api),
                UpdateKind::CallbackQuery(query) => handle_market_entry(query, &api),
                _ => {},
            }

            Ok(())
        });

    core.run(streams).unwrap();
    Ok(())
}

fn handle_market_search(message: Message, api: &Api) {
    match &message.kind {
        MessageKind::Text {
            data,
            entities: _entities,
        } => {
            let ids = get_ids_by_name(data);
            // https://docs.rs/telegram-bot/0.6.1/telegram_bot/macro.reply_markup.html
            let mut message = SendMessage::new(message.chat, "I've found these items on the market. Select one");

            // TODO: Fix this API design, why must this be mutable? How could we build it in a better way?
            let mut inline_keyboard_markup = InlineKeyboardMarkup::new();
            ids.into_iter().for_each(|idinfo| {
                let inline_keyboard_button = InlineKeyboardButton::callback(
                    format!("{} - {}", &idinfo.id, &idinfo.name),
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

fn handle_market_entry(query: telegram_bot::types::CallbackQuery, api: &Api) {
    let idinfo_id = query.data.parse::<i32>().ok();
    let chat = query.message.chat;
    let message_id = query.message.id;
    let button_text = match query.message.kind {
        MessageKind::Text { data: msg, entities: _ } => msg,
        _ => String::from("Unsupported Type")
    };
    match idinfo_id {
        None => {
            //TODO: todo!("Send message to user")
        }
        Some(id) => {
            let market_entries = get_market_entries(id);
            market_entries.first().map(|entry: &novaro_bot::item::ItemInfo| {
                let header = format!("Found the best price on '{}':\n", button_text);
                let _price = entry.price.unwrap_or(0);
                let body = format!("    {:?}", entry);
                let text = format!("{}{}", header, body);
                let edit = EditMessageText::new(chat, message_id, text);
                api.spawn(edit);
            });
        }
    }
}
