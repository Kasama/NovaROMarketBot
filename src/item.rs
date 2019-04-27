use std::cmp::Ordering;

#[derive(PartialEq, Eq, Ord, Debug)]
pub struct ItemInfo {
    pub name: Option<String>,
    pub item_type: ItemType,
    pub price: Option<i32>,
    pub amount: Option<i32>,
    pub refine: Option<i32>,
    pub properties: Option<String>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemType {
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
