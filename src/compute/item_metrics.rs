use serde::Serialize;

use crate::{
    consts::VALVE_GAME_IDS,
    prices::{PriceValue, PriceValueTrait},
    MarketItem,
};

#[derive(Clone, PartialEq, Serialize, Hash, Eq)]
pub enum ItemMetricType {
    ItemTotalSold,
    SteamEstimatedFee,
    GameEstimatedFee,
    ValveEstimatedFee,
}

impl ToString for ItemMetricType {
    fn to_string(&self) -> String {
        match self {
            ItemMetricType::ItemTotalSold => "ItemTotalSold".to_string(),
            ItemMetricType::SteamEstimatedFee => "SteamEstimatedFee".to_string(),
            ItemMetricType::GameEstimatedFee => "GameEstimatedFee".to_string(),
            ItemMetricType::ValveEstimatedFee => "ValveEstimatedFee".to_string(),
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
    SteamEstimatedFee(f64),
    GameEstimatedFee(f64),
    ValveEstimatedFee(f64),
}

// #[derive(Serialize, Clone)]
// pub enum CachedItemMetricValue {
//     NotComputed,
//     Computed(ItemMetricValue),
// }

pub struct ItemTotalSold;
pub struct SteamEstimatedFee;
pub struct GameEstimatedFee;
pub struct ValveEstimatedFee;

pub trait ItemMetricCalculation {
    fn calculate(&self, item: &MarketItem) -> ItemMetricValue;
}

impl ItemMetricCalculation for ItemTotalSold {
    fn calculate(&self, item: &MarketItem) -> ItemMetricValue {
        let total_sold: u64 = item
            .history
            .iter()
            .map(|(_, _, amount)| amount.clone() as u64)
            .sum();

        ItemMetricValue::TotalSold(total_sold)
    }
}

impl ItemMetricCalculation for SteamEstimatedFee {
    fn calculate(&self, item: &MarketItem) -> ItemMetricValue {
        let steam_fee: PriceValue = item
            .history
            .iter()
            .map(|(_, avg_price, amount)| avg_price * (*amount as u64) / 10) // divide by 10 to get 10% fee
            .sum();

        ItemMetricValue::SteamEstimatedFee(steam_fee.to_usd())
    }
}

impl ItemMetricCalculation for GameEstimatedFee {
    fn calculate(&self, item: &MarketItem) -> ItemMetricValue {
        let game_fee: PriceValue = item
            .history
            .iter()
            .map(|(_, avg_price, amount)| avg_price * (*amount as u64) / 20) // divide by 20 to get 5% fee
            .sum();

        ItemMetricValue::GameEstimatedFee(game_fee.to_usd())
    }
}

impl ItemMetricCalculation for ValveEstimatedFee {
    fn calculate(&self, item: &MarketItem) -> ItemMetricValue {
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
    }
}
