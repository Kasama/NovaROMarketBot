extern crate reqwest;
extern crate scraper;

use std::cmp::Ordering;
use std::error::Error;
use std::string::String;
use scraper::{Html,Selector,ElementRef};

fn main() -> Result<(), Box<dyn Error>> {
    // let info = get_lowest_price(7135)?;
    let info = get_market_entries(6380)?;
    let i = info.first().unwrap();

    println!("Item: {:?}", i.name);
    println!("Price: {:?}", i.price);
    println!("Amount: {:?}", i.amount);

    return Ok(())
}

#[derive(PartialEq, Eq, Ord, Debug)]
struct ItemInfo {
    name: Option<String>,
    item_type: ItemType,
    price: Option<i32>,
    amount: Option<i32>,
    refine: Option<i8>,
    properties: Option<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ItemType {
    Item,
    Equip,
}

impl PartialOrd for ItemInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let price_ord = self.price.cmp(&other.price);
        if price_ord == Ordering::Equal {
            let amount_ord = self.amount.cmp(&other.amount);
            return match amount_ord {
                Ordering::Greater => Some(Ordering::Less),
                Ordering::Equal => Some(Ordering::Equal),
                Ordering::Less => Some(Ordering::Greater),
            }
        }
        return Some(price_ord);
    }
}

fn get_market_entries(item_id: i32) -> Vec<ItemInfo> {
    let url = format!("https://www.novaragnarok.com/?module=vending&action=item&id={}", item_id);
    let maybe_body = reqwest::get(&url)
        .and_then(|response| {
            response.text()
        });
    if let Err(_) = maybe_body {
        return vec![];
    }

    let body = maybe_body.unwrap();

    return get_items(body);

}

fn get_items(body: String) -> Vec<ItemInfo> {
    let document = Html::parse_document(&body);

    let rows_selector = Selector::parse("#itemtable > tbody > tr").unwrap();

    let differentiation_selector = Selector::parse("th.sorting:nth-child(2)").unwrap();
    let name_selector = Selector::parse(".tooltip > a:nth-child(1)").unwrap();

    let price_selector = Selector::parse("td:nth-child(1)").unwrap();
    let amount_selector = Selector::parse("td:nth-child(2)").unwrap();
    let refine_selector = Selector::parse("td:nth-child(2)").unwrap();
    let properties_selector = Selector::parse("td:nth-child(3)").unwrap();

    fn get_number_from_table(el: ElementRef) -> i32 {
        return el.value().attr("data-order").unwrap_or("0").parse::<i32>().unwrap_or(0);
    }

    let items_iterator = document.select(&rows_selector);
    let item_name = document.select(&name_selector).next().map(|name_el: ElementRef| { name_el.inner_html() });
    let is_item = document.select(&differentiation_selector).next().map(|label: ElementRef| { label.inner_html().to_lowercase() != "qty" });

    let mut items: Vec<ItemInfo> = items_iterator.map(|item_element: scraper::ElementRef| {
        let name = item_name.clone();
        match is_item {
            Some(true) => ItemInfo{
                name: name,
                item_type: ItemType::Item,
                price: item_element.select(&price_selector).nth(0).map(get_number_from_table),
                amount: item_element.select(&amount_selector).nth(0).map(get_number_from_table),
                refine: None,
                properties: None,
            },
            Some(false) => ItemInfo{
                name: name,
                item_type: ItemType::Equip,
                price: item_element.select(&price_selector).nth(0).map(get_number_from_table),
                refine: item_element.select(&refine_selector).nth(0).map(get_number_from_table),
                properties: item_element.select(&properties_selector).nth(0),
                amount: None,
            },
            None => ItemInfo { None }
        }
    }).collect();

    items.sort();

    return items;

}
