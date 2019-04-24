extern crate reqwest;
extern crate scraper;

use std::error::Error;
use std::string::String;
use scraper::{Html,Selector,ElementRef};

fn main() -> Result<(), Box<dyn Error>> {
    /*
    let body = reqwest::get("https://www.novaragnarok.com/?module=vending&action=item&id=7135")?
        .text()?;

    let document = Html::parse_document(&body);

    let selector = Selector::parse(".tooltip > a:nth-child(1)").unwrap();
    let element = document.select(&selector).next().unwrap();
    let item_name = element.inner_html();
    println!("{:?}", item_name);
    */

    // let info = get_lowest_price(7135)?;
    let info = get_lowest_price(6380)?;

    println!("Item: {:?}", info.name);
    println!("Price: {:?}", info.price);
    println!("Amount: {:?}", info.amount);

    return Ok(())
}

struct ItemInfo {
    name: String,
    price: i32,
    amount: i32,
}

fn get_lowest_price(item_id: i32) -> Result<ItemInfo, Box<dyn Error>> {
    let url = format!("https://www.novaragnarok.com/?module=vending&action=item&id={}", item_id);
    let body = reqwest::get(&url)?.text()?;

    let document = Html::parse_document(&body);

    let rows_selector = Selector::parse("#itemtable > tbody > tr").unwrap();

    let name_selector = Selector::parse(".tooltip > a:nth-child(1)").unwrap();
    let price_selector = Selector::parse("td:nth-child(1)").unwrap();
    let amount_selector = Selector::parse("td:nth-child(2)").unwrap();

    fn get_number_from_table(el: ElementRef) -> i32 {
        return el.value().attr("data-order").unwrap_or("0").parse::<i32>().unwrap_or(0);
    }

    let cheapest: scraper::ElementRef = document.select(&rows_selector).last().unwrap();

    let maybe_name: Option<ElementRef> = document.select(&name_selector).next();
    let name = maybe_name.map(|name_el: ElementRef| { name_el.inner_html() }).unwrap_or("<Couldnt find item name>".to_string());

    let maybe_price: Option<ElementRef> = cheapest.select(&price_selector).nth(0);
    let price = maybe_price.map(get_number_from_table).unwrap_or(0);

    let maybe_amount: Option<ElementRef> = cheapest.select(&amount_selector).nth(0);
    let amount = maybe_amount.map(get_number_from_table).unwrap_or(0);

    return Ok(ItemInfo{
        name: name,
        price: price,
        amount: amount,
    })
}
