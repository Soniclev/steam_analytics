use std::collections::HashMap;

use serde::Serialize;

use crate::{compute::global::{huge, metrics}, webui::ItemCategoryStatsFull, MarketItem};

#[derive(Clone, PartialEq, Serialize)]
pub enum GlobalMetricType {
    TotalSold,
    TotalVolume,
    AveragePrice,
    SteamEstimatedFee,
    GameEstimatedFee,
    ValveEstimatedFee,
    CS2TotalItemsByCategory,
}


#[derive(Serialize, Clone)]
pub struct GlobalMetricResult {
    pub kind: GlobalMetricType,
    pub result: GlobalMetricValue,
    pub duration_micros: u128,
}

impl GlobalMetricResult {
    pub fn should_include_in_ws(&self) -> bool {
        self.kind != GlobalMetricType::CS2TotalItemsByCategory
    }
}

#[derive(Serialize, Clone)]
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

pub fn get_metrics() -> Vec<(GlobalMetricType, Box<dyn MetricCalculation>)> {
    vec![
                (
                    GlobalMetricType::TotalSold,
                    Box::new(metrics::TotalSold),
                ),
                (
                    GlobalMetricType::AveragePrice,
                    Box::new(metrics::AveragePrice),
                ),
                (
                    GlobalMetricType::TotalVolume,
                    Box::new(metrics::TotalVolume),
                ),
                (
                    GlobalMetricType::SteamEstimatedFee,
                    Box::new(metrics::SteamEstimatedFee),
                ),
                (
                    GlobalMetricType::GameEstimatedFee,
                    Box::new(metrics::GameEstimatedFee),
                ),
                (
                    GlobalMetricType::ValveEstimatedFee,
                    Box::new(metrics::ValveEstimatedFee),
                ),
                (
                    GlobalMetricType::CS2TotalItemsByCategory,
                    Box::new(huge::cs2::CS2TotalItemsByCategory),
                )
            ]
}
