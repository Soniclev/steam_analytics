use chrono::{Duration, Utc};
use serde::Serialize;

use crate::MarketItem;

#[derive(Serialize, Clone)]
pub struct ItemMetricResult {
    pub result: ItemMetricValue,
    pub duration_micros: u128,
}

#[derive(Serialize, Clone)]
pub enum ItemMetricValue {
    PopularityScore(f64),
}

macro_rules! define_metric {
    ($metric_name:ident, $calc_body:expr, $result_type:expr) => {
        pub struct $metric_name;

        impl ItemMetricCalculation for $metric_name {
            fn calculate(&self, item: &MarketItem) -> ItemMetricValue {
                $calc_body(item)
            }
        }
    };
}

pub trait ItemMetricCalculation {
    fn calculate(&self, item: &MarketItem) -> ItemMetricValue;
}

define_metric!(
    ItemPopularityScore,
    |item: &MarketItem| {
        let begin_from = Utc::now().checked_sub_signed(Duration::days(365)).unwrap().date_naive();
        let total_sold: u64 = item
            .history
            .iter()
            // filter only items that are sold last 365 days
            .filter(|(date, _, _)| {
                *date >= begin_from
            })
            .map(|(_, _, amount)| *amount as u64)
            .sum();

        let popularity_score = (total_sold as f64).sqrt();
        ItemMetricValue::PopularityScore(popularity_score)
    },
    ItemMetricType::ItemPopularityScore
);
