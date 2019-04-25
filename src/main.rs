extern crate reqwest;
extern crate scraper;

use std::cmp::Ordering;
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

#[derive(PartialEq, Eq, PartialOrd)]
struct ItemInfo {
    name: Option<String>,
    price: Option<i32>,
    amount: Option<i32>,
    refine: Option<i8>,
    properties: Option<String>,
}

impl Ord for ItemInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        let price_ord = self.price.cmp(&other.price);
        if price_ord == Ordering::Equal {
            let amount_ord = self.amount.cmp(&other.amount);
            if amount_ord == Ordering::Less {
                return Ordering::Greater;
            } else if amount_ord == Ordering::Greater {
                return Ordering::Less;
            } else {
                return Ordering::Equal;
            };
        }
        return price_ord;
    }
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

    let items_iterator = document.select(&rows_selector);
    let item_name = document.select(&name_selector).next().map(|name_el: ElementRef| { name_el.inner_html() });

    let items: Vec<ItemInfo> = items_iterator.map(|item_element: scraper::ElementRef| {
        return ItemInfo{
            name: item_name,
            price: item_element.select(&price_selector).nth(0).map(get_number_from_table),
            amount: item_element.select(&amount_selector).nth(0).map(get_number_from_table),
            refine: None,
            properties: None,
        };
    }).collect();

    items.sort();

    let cheapest: scraper::ElementRef = document.select(&rows_selector).last().unwrap();

    let maybe_name: Option<ElementRef> = document.select(&name_selector).next();

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
