use serde::Serialize;

use crate::{prices::PriceValueTrait, MarketItem};

#[derive(Clone, Serialize)]
pub struct StaticMetrics {
    pub total_volume: f64,
}

impl StaticMetrics {
    pub fn new() -> Self {
        Self { total_volume: 0.0 }
    }
}


pub fn compute_item_static_metrics(item: &mut MarketItem) {
    let total_volume: u64 = item
        .history
        .iter()
        .map(|(_, avg_price, amount)| {
            avg_price * (*amount as u64)
        }
        )
        .sum();
    item.static_metrics.total_volume = total_volume.to_usd(); // Assuming price is converted

}
