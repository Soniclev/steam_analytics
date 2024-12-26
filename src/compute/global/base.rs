use std::collections::HashMap;

use serde::Serialize;

use crate::{
    compute::global::{huge, metrics},
    webui::ItemCategoryStatsFull,
    MarketItem,
};

#[derive(Serialize, Clone)]
pub struct GlobalMetricResult {
    pub result: GlobalMetricValue,
    pub duration_micros: u128,
}

impl GlobalMetricResult {
    pub fn should_include_in_ws(&self) -> bool {
        match self.result {
            GlobalMetricValue::TotalVolume(_) => true,
            GlobalMetricValue::TotalSold(_) => true,
            GlobalMetricValue::AveragePrice(_) => true,
            GlobalMetricValue::SteamEstimatedFee(_) => true,
            GlobalMetricValue::GameEstimatedFee(_) => true,
            GlobalMetricValue::ValveEstimatedFee(_) => true,
            GlobalMetricValue::CS2TotalItemsByCategory(_) => false,
        }
    }
}

#[derive(Serialize, Clone, PartialEq)]
pub enum GlobalMetricValue {
    TotalVolume(f64),
    TotalSold(u64),
    AveragePrice(f64),
    SteamEstimatedFee(f64),
    GameEstimatedFee(f64),
    ValveEstimatedFee(f64),
    CS2TotalItemsByCategory(HashMap<String, ItemCategoryStatsFull>),
}

pub trait MetricCalculation {
    fn to_string(&self) -> String;
    fn is_huge(&self) -> bool;
    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue;
}

pub fn get_metrics() -> Vec<Box<dyn MetricCalculation>> {
    vec![
        Box::new(metrics::TotalSold),
        Box::new(metrics::AveragePrice),
        Box::new(metrics::TotalVolume),
        Box::new(metrics::SteamEstimatedFee),
        Box::new(metrics::GameEstimatedFee),
        Box::new(metrics::ValveEstimatedFee),
        Box::new(huge::cs2::CS2TotalItemsByCategory),
    ]
}
