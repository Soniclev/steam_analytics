use chrono::{Duration, Utc};
use serde::Serialize;

use crate::{
    consts::VALVE_GAME_IDS,
    prices::{PriceValue, PriceValueTrait},
    MarketItem,
};

#[derive(Clone, PartialEq, Serialize, Hash, Eq)]
pub enum ItemMetricType {
    ItemTotalSold,
    ItemTotalVolume,
    ItemSteamEstimatedFee,
    ItemGameEstimatedFee,
    ItemValveEstimatedFee,
    ItemPopularityScore,
}

impl ToString for ItemMetricType {
    fn to_string(&self) -> String {
        match self {
            ItemMetricType::ItemTotalSold => "TotalSold".to_string(),
            ItemMetricType::ItemTotalVolume => "TotalVolume".to_string(),
            ItemMetricType::ItemSteamEstimatedFee => "SteamEstimatedFee".to_string(),
            ItemMetricType::ItemGameEstimatedFee => "GameEstimatedFee".to_string(),
            ItemMetricType::ItemValveEstimatedFee => "ValveEstimatedFee".to_string(),
            ItemMetricType::ItemPopularityScore => "PopularityScore".to_string(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct ItemMetricResult {
    pub kind: ItemMetricType,
    pub result: ItemMetricValue,
    pub duration_micros: u128,
}

#[derive(Serialize, Clone)]
pub enum ItemMetricValue {
    TotalSold(u64),
    TotalVolume(f64),
    SteamEstimatedFee(f64),
    GameEstimatedFee(f64),
    ValveEstimatedFee(f64),
}

macro_rules! define_metric {
    ($metric_name:ident, $calc_body:expr, $result_type:expr) => {
        pub struct $metric_name;

        impl ItemMetricCalculation for $metric_name {
            fn calculate(&self, item: &MarketItem) -> ItemMetricValue {
                $calc_body(item)
            }
        }

        // impl ToString for $metric_name {
        //     fn to_string(&self) -> String {
        //         stringify!($metric_name).to_string()
        //     }
        // }
    };
}

pub trait ItemMetricCalculation {
    fn calculate(&self, item: &MarketItem) -> ItemMetricValue;
}

define_metric!(
    ItemTotalSold,
    |item: &MarketItem| {
        let total_sold: u64 = item
            .history
            .iter()
            .map(|(_, _, amount)| *amount as u64)
            .sum();
        ItemMetricValue::TotalSold(total_sold)
    },
    ItemMetricType::ItemTotalSold
);

define_metric!(
    ItemTotalVolume,
    |item: &MarketItem| {
        let total_volume: u64 = item
            .history
            .iter()
            .map(|(_, avg_price, amount)| avg_price * (*amount as u64))
            .sum();
        ItemMetricValue::TotalVolume(total_volume as f64) // Assuming price is converted
    },
    ItemMetricType::ItemTotalVolume
);

define_metric!(
    ItemSteamEstimatedFee,
    |item: &MarketItem| {
        let steam_fee: PriceValue = item
            .history
            .iter()
            .map(|(_, avg_price, amount)| avg_price * (*amount as u64) / 10) // divide by 10 to get 10% fee
            .sum();

        ItemMetricValue::SteamEstimatedFee(steam_fee.to_usd())
    },
    ItemMetricType::ItemSteamEstimatedFee
);

define_metric!(
    ItemGameEstimatedFee,
    |item: &MarketItem| {
        let game_fee: PriceValue = item
            .history
            .iter()
            .map(|(_, avg_price, amount)| avg_price * (*amount as u64) / 20) // divide by 20 to get 5% fee
            .sum();

        ItemMetricValue::GameEstimatedFee(game_fee.to_usd())
    },
    ItemMetricType::ItemGameEstimatedFee
);

define_metric!(
    ItemValveEstimatedFee,
    |item: &MarketItem| {
        let steam_fee: PriceValue = item
            .history
            .iter()
            .map(|(_, avg_price, amount)| avg_price * (*amount as u64) / 10) // divide by 10 to get 10% fee
            .sum();

        let game_fee = if VALVE_GAME_IDS.contains(&item.app_id) {
            item.history
                .iter()
                .map(|(_, avg_price, amount)| avg_price * (*amount as u64) / 20) // divide by 20 to get 5% fee
                .sum()
        } else {
            0
        };

        let valve_fee = steam_fee + game_fee;

        ItemMetricValue::ValveEstimatedFee(valve_fee.to_usd())
    },
    ItemMetricType::ItemValveEstimatedFee
);

define_metric!(
    ItemPopularityScore,
    |item: &MarketItem| {
        let total_sold: u64 = item
            .history
            .iter()
            // filter only items that are sold last 365 days
            .filter(|(date, _, _)| {
                *date >= Utc::now().checked_sub_signed(Duration::days(365)).unwrap()
            })
            .map(|(_, _, amount)| *amount as u64)
            .sum();

        let popularity_score = (total_sold as f64).sqrt();
        ItemMetricValue::TotalVolume(popularity_score)
    },
    ItemMetricType::ItemPopularityScore
);
