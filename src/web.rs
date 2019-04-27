extern crate reqwest;
extern crate scraper;

use crate::item::{ItemInfo,ItemType};
use std::string::String;
use scraper::{Html,Selector,ElementRef};

pub fn get_market_entries(item_id: i32) -> Vec<ItemInfo> {
    let url = format!("https://www.novaragnarok.com/?module=vending&action=item&id={}", item_id);
    let maybe_body = reqwest::get(&url)
        .and_then(|mut response| {
            response.text()
        });

    return match maybe_body {
        Ok(body) => get_items(body),
        Err(_) => vec![],
    };

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
    if let None = is_item { return vec![] };

    let mut items: Vec<ItemInfo> = items_iterator.map(|item_element: scraper::ElementRef| {
        let name = item_name.clone();
        match is_item.unwrap() {
            true => ItemInfo{
                name: name,
                item_type: ItemType::Item,
                price: item_element.select(&price_selector).nth(0).map(get_number_from_table),
                amount: item_element.select(&amount_selector).nth(0).map(get_number_from_table),
                refine: None,
                properties: None,
            },
            false => ItemInfo{
                name: name,
                item_type: ItemType::Equip,
                price: item_element.select(&price_selector).nth(0).map(get_number_from_table),
                refine: item_element.select(&refine_selector).nth(0).map(get_number_from_table),
                properties: item_element.select(&properties_selector).nth(0).map(|el| el.inner_html()),
                amount: None,
            },
        }
    }).collect();

    items.sort();

    return items;

}
