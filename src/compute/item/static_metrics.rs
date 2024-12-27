use serde::Serialize;

use crate::{
    consts::VALVE_GAME_IDS, prices::{PriceValue, PriceValueTrait}, MarketItem
};

#[derive(Clone, Serialize)]
pub struct StaticMetrics {
    pub total_sold: u64,
    pub total_volume: f64,
    pub steam_estimated_fee: f64,
    pub game_estimated_fee: f64,
    pub valve_estimated_fee: f64,
}

impl StaticMetrics {
    pub fn new() -> Self {
        Self {
            total_sold: 0,
            total_volume: 0.0,
            steam_estimated_fee: 0.0,
            game_estimated_fee: 0.0,
            valve_estimated_fee: 0.0,
        }
    }
}

pub fn compute_item_static_metrics(item: &mut MarketItem) {
    let total_sold: u64 = item
        .history
        .iter()
        .map(|(_, _, amount)| *amount as u64)
        .sum();

    let total_volume: PriceValue = item
        .history
        .iter()
        .map(|(_, avg_price, amount)| (*avg_price as u64) * (*amount as u64))
        .sum();

    let steam_fee: PriceValue = item
        .history
        .iter()
        .map(|(_, avg_price, amount)| (*avg_price as u64) * (*amount as u64) / 10) // divide by 10 to get 10% fee
        .sum();

    let game_fee: PriceValue = item
        .history
        .iter()
        .map(|(_, avg_price, amount)| (*avg_price as u64) * (*amount as u64) / 20) // divide by 20 to get 5% fee
        .sum();

    let valve_game_fee: PriceValue = if VALVE_GAME_IDS.contains(&item.app_id) {
        game_fee
    } else {
        0
    };

    let valve_fee = steam_fee + valve_game_fee;

    item.static_metrics.total_sold = total_sold;
    item.static_metrics.total_volume = total_volume.to_usd(); // Assuming price is converted
    item.static_metrics.steam_estimated_fee = steam_fee.to_usd();
    item.static_metrics.game_estimated_fee = game_fee.to_usd();
    item.static_metrics.valve_estimated_fee = valve_fee.to_usd();
}
