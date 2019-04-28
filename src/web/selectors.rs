extern crate scraper;

struct Selector {
    Rows: scraper::Selector,
    Differentiation: scraper::Selector,
    Name: scraper::Selector,
    Price: scraper::Selector,
    Amount: scraper::Selector,
    Refine: scraper::Selector,
    Properties: scraper::Selector,
}

pub selector = Selector{
    Rows: scraper::Selector::parse("#itemtable > tbody > tr").unwrap(),

    Differentiation: scraper::Selector::parse("th.sorting:nth-child(2)").unwrap(),
    Name: scraper::Selector::parse(".tooltip > a:nth-child(1)").unwrap(),

    Price: scraper::Selector::parse("td:nth-child(1)").unwrap(),
    Amount: scraper::Selector::parse("td:nth-child(2)").unwrap(),
    Refine: scraper::Selector::parse("td:nth-child(2)").unwrap(),
    Properties: scraper::Selector::parse("td:nth-child(3)").unwrap(),
};
