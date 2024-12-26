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
    fn to_string(&self) -> String {
        "TotalSold".to_string()
    }

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
    fn to_string(&self) -> String {
        "AveragePrice".to_string()
    }

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
    fn to_string(&self) -> String {
        "TotalVolume".to_string()
    }

    fn is_huge(&self) -> bool {
        false
    }

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

impl MetricCalculation for SteamEstimatedFee {
    fn to_string(&self) -> String {
        "SteamEstimatedFee".to_string()
    }

    fn is_huge(&self) -> bool {
        false
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let steam_fee: f64 = items
            .iter()
            .map(|(_, item)| {
                // const KIND: metrics::ItemMetricType =
                //     metrics::ItemMetricType::ItemSteamEstimatedFee;
                const KIND: &str = stringify!(ItemSteamEstimatedFee);
                item.metrics
                    .get(KIND)
                    .and_then(|computed| match computed {
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
    fn to_string(&self) -> String {
        "GameEstimatedFee".to_string()
    }

    fn is_huge(&self) -> bool {
        false
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let game_fee: f64 = items
            .iter()
            .map(|(_, item)| {
                // const KIND: metrics::ItemMetricType =
                //     metrics::ItemMetricType::ItemGameEstimatedFee;
                const KIND: &str = stringify!(ItemGameEstimatedFee);
                item.metrics
                    .get(KIND)
                    .and_then(|computed| match computed {
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
    fn to_string(&self) -> String {
        "ValveEstimatedFee".to_string()
    }

    fn is_huge(&self) -> bool {
        false
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let valve_fee: f64 = items
            .iter()
            .map(|(_, item)| {
                // const KIND: metrics::ItemMetricType =
                //     metrics::ItemMetricType::ItemValveEstimatedFee;
                const KIND: &str = stringify!(ItemValveEstimatedFee);
                item.metrics
                    .get(KIND)
                    .and_then(|computed| match computed {
                        ItemMetricValue::ValveEstimatedFee(fee) => Some(fee.clone()),
                        _ => None,
                    })
                    .unwrap_or(0.0)
            })
            .sum();

        GlobalMetricValue::ValveEstimatedFee(valve_fee)
    }
}

