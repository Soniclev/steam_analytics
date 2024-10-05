use serde::Serialize;

use crate::MarketItem;


#[derive(Clone, PartialEq, Serialize)]
pub enum ItemMetricType {
    ItemTotalSold,
}

impl ToString for ItemMetricType {
    fn to_string(&self) -> String {
        match self {
            ItemMetricType::ItemTotalSold => "ItemTotalSold".to_string(),
        }
    }
}


#[derive(Serialize)]
pub struct ItemMetricResult {
    pub kind: ItemMetricType,
    pub result: ItemMetricValue,
    pub duration_micros: u128, 
}

#[derive(Serialize)]
pub enum ItemMetricValue {
    TotalSold(u64),
}


pub struct ItemTotalSold;


pub trait ItemMetricCalculation {
    fn calculate(&self, item: &MarketItem) -> ItemMetricValue;
}


impl ItemMetricCalculation for ItemTotalSold {
    fn calculate(&self, item: &MarketItem) -> ItemMetricValue {
        let total_sold: u64 = item.history.iter()
            .map(|(_, _, amount)| amount.clone() as u64)
            .sum();

        ItemMetricValue::TotalSold(total_sold)
    }
}
