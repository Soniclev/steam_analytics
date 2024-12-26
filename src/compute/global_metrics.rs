use std::collections::HashMap;

use chrono::{Datelike, Timelike};
use serde::Serialize;

use crate::{compute::item_metrics::ItemMetricValue, game::cs2::ItemCategory, prices::PriceValueTrait, webui::ItemCategoryStatsFull, MarketItem, MarketItemState};

use super::item_metrics::{self};

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

impl ToString for GlobalMetricType {
    fn to_string(&self) -> String {
        match self {
            GlobalMetricType::TotalSold => "TotalSold".to_string(),
            GlobalMetricType::TotalVolume => "TotalVolume".to_string(),
            GlobalMetricType::AveragePrice => "AveragePrice".to_string(),
            GlobalMetricType::SteamEstimatedFee => "SteamEstimatedFee".to_string(),
            GlobalMetricType::GameEstimatedFee => "GameEstimatedFee".to_string(),
            GlobalMetricType::ValveEstimatedFee => "ValveEstimatedFee".to_string(),
            GlobalMetricType::CS2TotalItemsByCategory => "CS2TotalItemsByCategory".to_string(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct GlobalMetricResult {
    pub kind: GlobalMetricType,
    pub result: GlobalMetricValue,
    pub duration_micros: u128,
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

pub struct TotalSold;
pub struct TotalVolume;
pub struct AveragePrice;
pub struct SteamEstimatedFee;
pub struct GameEstimatedFee;
pub struct ValveEstimatedFee;

pub struct CS2TotalItemsByCategory;

pub trait MetricCalculation {
    fn is_huge(&self) -> bool;
    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue;
}

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
                item.analyzes_result
                    .as_ref()
                    .map_or(0.0, |r| r.total_volume)
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
                const KIND: item_metrics::ItemMetricType =
                    item_metrics::ItemMetricType::ItemSteamEstimatedFee;
                item.metrics
                    .get(&KIND)
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
    fn is_huge(&self) -> bool {
        false
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let game_fee: f64 = items
            .iter()
            .map(|(_, item)| {
                const KIND: item_metrics::ItemMetricType =
                    item_metrics::ItemMetricType::ItemGameEstimatedFee;
                item.metrics
                    .get(&KIND)
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
    fn is_huge(&self) -> bool {
        false
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let valve_fee: f64 = items
            .iter()
            .map(|(_, item)| {
                const KIND: item_metrics::ItemMetricType =
                    item_metrics::ItemMetricType::ItemValveEstimatedFee;
                item.metrics
                    .get(&KIND)
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

impl MetricCalculation for CS2TotalItemsByCategory {
    fn is_huge(&self) -> bool {
        true
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let mut result: HashMap<ItemCategory, ItemCategoryStatsFull> = HashMap::new();

        for (_, item) in items.iter() {
            if item.app_id != 730 {
                continue;
            }

            let category = item.determine_item_category.clone();

            if !result.contains_key(&category) {
                result.insert(category.clone(), ItemCategoryStatsFull::new());
            }

            let value = result.get_mut(&category).unwrap();
            value.total_items += 1;

            match item.state {
                MarketItemState::Analyzed => {
                    value.total_analyzed_items += 1;
    
                    item.metrics.iter()
                        .for_each(|(_, metric)| match metric {
                            ItemMetricValue::TotalSold(sold) => value.total_sold += sold.clone(),
                            ItemMetricValue::TotalVolume(volume) => value.total_volume += volume.clone(),
                            _ => {},
                        });

                    // iterate over history and count sold per month
                    let mut sold_per_day: HashMap<&chrono::DateTime<chrono::Utc>, u64> = HashMap::new();

                    for (date, _price, amount) in item.history.iter() {
                        if let Some(count) = sold_per_day.get(&date) {
                            sold_per_day.insert(date, count + *amount as u64);
                        }
                        else {
                            sold_per_day.insert(date, *amount as u64);
                        }
                    }

                    // sum up all sold per month
                    for (dt, amount) in sold_per_day.into_iter() {
                        if value.sold_per_day.contains_key(dt) {
                            value.sold_per_day.insert(dt.clone(), value.sold_per_day.get(dt).unwrap() + amount);
                        }
                        else {
                            value.sold_per_day.insert(dt.clone(), amount);
                        }

                        let date_trunc_by_month = dt.with_day(1).unwrap().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap();
                        if let Some(count) = value.sold_per_month.get(&date_trunc_by_month) {
                            value.sold_per_month.insert(date_trunc_by_month, count + amount);
                        }
                        else {
                            value.sold_per_month.insert(date_trunc_by_month, amount);
                        }
                    }
                },
                _ => {},
            };
        }
        GlobalMetricValue::CS2TotalItemsByCategory(result.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }
}
