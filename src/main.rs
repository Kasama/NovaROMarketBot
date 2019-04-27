mod item;
mod web;

use std::error::Error;
use web::get_market_entries;

fn main() -> Result<(), Box<dyn Error>> {
    // let info = get_lowest_price(7135);
    let info = get_market_entries(6380);
    let i = info.first();

    println!("Item: {:?}", i);

    return Ok(())
}

