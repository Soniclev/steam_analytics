use std::collections::HashMap;

use crate::{compute::item::metrics::ItemMetricValue, prices::PriceValueTrait, MarketItem};

use super::base::{GlobalMetricValue, MetricCalculation};

pub struct TotalSold;
pub struct TotalVolume;
pub struct AveragePrice;
pub struct SteamEstimatedFee;
pub struct GameEstimatedFee;
pub struct ValveEstimatedFee;

impl MetricCalculation for TotalSold {
    fn is_huge(&self) -> bool {
        false
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let total_sold: u64 = items
            .iter()
            .map(|(_, item)| item.analyzes_result.as_ref().map_or(0, |r| r.total_sold))
            .sum();

        GlobalMetricValue::TotalSold(total_sold)
    }
}

impl MetricCalculation for AveragePrice {
    fn is_huge(&self) -> bool {
        false
    }

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
    fn is_huge(&self) -> bool {
        false
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let total_volume: f64 = items
            .iter()
            .map(|(_, item)| {
                item.static_metrics.total_volume
                // item.analyzes_result
                //     .as_ref()
                //     .map_or(0.0, |r| r.total_volume)
            })
            .sum();

        GlobalMetricValue::TotalVolume(total_volume)
    }
}

impl MetricCalculation for SteamEstimatedFee {
    fn is_huge(&self) -> bool {
        false
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let steam_fee: f64 = items
            .iter()
            .map(|(_, item)| {
                item.metrics
                    .iter()
                    .find(|m| matches!(m.result, ItemMetricValue::SteamEstimatedFee(_)))
                    .and_then(|computed| match computed.result {
                        ItemMetricValue::SteamEstimatedFee(fee) => Some(fee.clone()),
                        _ => None,
                    })
                    .unwrap_or(0.0)
            })
            .sum();

        GlobalMetricValue::SteamEstimatedFee(steam_fee)
    }
}

impl MetricCalculation for GameEstimatedFee {
    fn is_huge(&self) -> bool {
        false
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let game_fee: f64 = items
            .iter()
            .map(|(_, item)| {
                item.metrics
                    // .get(KIND)
                    .iter()
                    .find(|m| matches!(m.result, ItemMetricValue::GameEstimatedFee(_)))
                    .and_then(|computed| match computed.result {
                        ItemMetricValue::GameEstimatedFee(fee) => Some(fee.clone()),
                        _ => None,
                    })
                    .unwrap_or(0.0)
            })
            .sum();

        GlobalMetricValue::GameEstimatedFee(game_fee)
    }
}

impl MetricCalculation for ValveEstimatedFee {
    fn is_huge(&self) -> bool {
        false
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let valve_fee: f64 = items
            .iter()
            .map(|(_, item)| {
                item.metrics
                    // .get(KIND)
                    .iter()
                    .find(|m| matches!(m.result, ItemMetricValue::ValveEstimatedFee(_)))
                    .and_then(|computed| match computed.result {
                        ItemMetricValue::ValveEstimatedFee(fee) => Some(fee.clone()),
                        _ => None,
                    })
                    .unwrap_or(0.0)
            })
            .sum();

        GlobalMetricValue::ValveEstimatedFee(valve_fee)
    }
}
