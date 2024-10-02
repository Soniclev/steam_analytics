use std::collections::HashMap;

use crate::{prices::PriceValueTrait, MarketItem};


pub enum MetricType {
    TotalSold,
    TotalVolume,
    AveragePrice,
}


pub struct MetricResult {
    pub result: MetricValue,
    pub duration_micros: u128, 
}

pub enum MetricValue {
    TotalVolume(f64),
    TotalSold(u64),
    AveragePrice(f64),
}


impl MetricResult {
    pub fn to_html(&self) -> String {
        let result_str = match &self.result {
            MetricValue::TotalSold(sold) => format!("Total Sold: {} pcs.", sold),
            MetricValue::TotalVolume(volume) => format!("Total Volume: ${:.2}", volume),
            MetricValue::AveragePrice(price) => format!("Average Price: ${:.2}", price),
        };
        
        format!("{} (processed in {} µs)", result_str, self.duration_micros)
    }
}
pub struct TotalSold;
pub struct TotalVolume;
pub struct AveragePrice;


pub trait MetricCalculation {
    fn calculate(&self, items: &HashMap<String, MarketItem>) -> MetricValue;
}


impl MetricCalculation for TotalSold {
    fn calculate(&self, items: &HashMap<String, MarketItem>) -> MetricValue {
        let total_sold: u64 = items.iter()
            .map(|(_, item)| item.analyzes_result.as_ref().map_or(0, |r| r.total_sold))
            .sum();

        MetricValue::TotalSold(total_sold)
    }
}


impl MetricCalculation for AveragePrice {
    fn calculate(&self, items: &HashMap<String, MarketItem>) -> MetricValue {
        let total_items = items.len() as f64;
        let total_price: f64 = items.iter().map(|(_, item)| item.price.to_usd()).sum();
        let avg_price = if total_items > 0.0 { total_price / total_items } else { 0.0 };

        MetricValue::AveragePrice(avg_price)
    }
}


impl MetricCalculation for TotalVolume {
    fn calculate(&self, items: &HashMap<String, MarketItem>) -> MetricValue {
        let total_volume: f64 = items.iter().map(|(_, item)| item.analyzes_result.as_ref().map_or(0.0, |r| r.total_volume)).sum();

        MetricValue::TotalVolume(total_volume)
    }
}
