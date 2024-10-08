use std::collections::HashMap;

use serde::Serialize;

use crate::{prices::PriceValueTrait, MarketItem};

#[derive(Clone, PartialEq, Serialize)]
pub enum GlobalMetricType {
    TotalSold,
    TotalVolume,
    AveragePrice,
}

impl ToString for GlobalMetricType {
    fn to_string(&self) -> String {
        match self {
            GlobalMetricType::TotalSold => "TotalSold".to_string(),
            GlobalMetricType::TotalVolume => "TotalVolume".to_string(),
            GlobalMetricType::AveragePrice => "AveragePrice".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct GlobalMetricResult {
    pub kind: GlobalMetricType,
    pub result: GlobalMetricValue,
    pub duration_micros: u128,
}

#[derive(Serialize)]
pub enum GlobalMetricValue {
    TotalVolume(f64),
    TotalSold(u64),
    AveragePrice(f64),
}

pub struct TotalSold;
pub struct TotalVolume;
pub struct AveragePrice;

pub trait MetricCalculation {
    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue;
}

impl MetricCalculation for TotalSold {
    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let total_sold: u64 = items
            .iter()
            .map(|(_, item)| item.analyzes_result.as_ref().map_or(0, |r| r.total_sold))
            .sum();

        GlobalMetricValue::TotalSold(total_sold)
    }
}

impl MetricCalculation for AveragePrice {
    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let total_items = items.len() as f64;
        let total_price: f64 = items.iter().map(|(_, item)| item.price.to_usd()).sum();
        let avg_price = if total_items > 0.0 {
            total_price / total_items
        } else {
            0.0
        };

        GlobalMetricValue::AveragePrice(avg_price)
    }
}

impl MetricCalculation for TotalVolume {
    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let total_volume: f64 = items
            .iter()
            .map(|(_, item)| {
                item.analyzes_result
                    .as_ref()
                    .map_or(0.0, |r| r.total_volume)
            })
            .sum();

        GlobalMetricValue::TotalVolume(total_volume)
    }
}
