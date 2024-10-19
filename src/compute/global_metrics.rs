use std::collections::HashMap;

use serde::Serialize;

use crate::{compute::item_metrics::ItemMetricValue, game::cs2::ItemCategory, prices::PriceValueTrait, webui::ItemCategoryStats, MarketItem, MarketItemState};

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
    CS2TotalItemsByCategory(HashMap<String, ItemCategoryStats>),
}

pub struct TotalSold;
pub struct TotalVolume;
pub struct AveragePrice;
pub struct SteamEstimatedFee;
pub struct GameEstimatedFee;
pub struct ValveEstimatedFee;

pub struct CS2TotalItemsByCategory;

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

impl MetricCalculation for SteamEstimatedFee {
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
    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let mut result: HashMap<ItemCategory, ItemCategoryStats> = HashMap::new();

        for (_, item) in items.iter() {
            let category = item.determine_item_category.clone();

            if !result.contains_key(&category) {
                result.insert(category.clone(), ItemCategoryStats::new());
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
                    let mut sold_per_month = HashMap::new();
                    for (date, price, amount) in item.history.iter() {
                        // let month = date.date().month();
                        // let year = date.date().year();
                        // let key = DateTime::from_utc(NaiveDate::from_ymd(year, month, 1).and_hms(0, 0, 0), Utc);
                        if let Some(count) = sold_per_month.get(&date) {
                            sold_per_month.insert(date, count + amount);
                        }
                        else {
                            sold_per_month.insert(date, *amount);
                        }
                    }

                    // sum up all sold per month
                    for (dt, count) in sold_per_month.iter() {
                        // upsert
                        if value.sold_per_month.contains_key(dt) {
                            value.sold_per_month.insert(*dt.clone(), value.sold_per_month.get(dt).unwrap() + (*count as u64));
                        }
                        else {
                            value.sold_per_month.insert(*dt.clone(), *count as u64);
                        }
                    }
                },
                _ => {},
            };
        }
        GlobalMetricValue::CS2TotalItemsByCategory(result.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }
}
