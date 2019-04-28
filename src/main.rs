mod item;
mod web;

use std::error::Error;
use web::{get_ids_by_name};

fn main() -> Result<(), Box<dyn Error>> {

    // let info = get_market_entries(2992); // pendant
    // let info = get_market_entries(7135); // bottle grenade
    // let info = get_market_entries(6380); // mora coin
    let ids = get_ids_by_name(String::from("heroic backpack"));
    // let i = info.first();

    println!("ids: {:?}", ids);

    return Ok(());
}
