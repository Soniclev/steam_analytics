use crate::compute::item::metrics::{self, ItemMetricType};

use super::metrics::ItemMetricCalculation;

pub fn get_metrics() -> Vec<(ItemMetricType, Box<dyn ItemMetricCalculation>)> {
    vec![
                (
                    ItemMetricType::ItemTotalSold,
                    Box::new(metrics::ItemTotalSold),
                ),
                (
                    ItemMetricType::ItemTotalVolume,
                    Box::new(metrics::ItemTotalVolume),
                ),
                (
                    ItemMetricType::ItemSteamEstimatedFee,
                    Box::new(metrics::ItemSteamEstimatedFee),
                ),
                (
                    ItemMetricType::ItemGameEstimatedFee,
                    Box::new(metrics::ItemGameEstimatedFee),
                ),
                (
                    ItemMetricType::ItemValveEstimatedFee,
                    Box::new(metrics::ItemValveEstimatedFee),
                ),
                (
                    ItemMetricType::ItemPopularityScore,
                    Box::new(metrics::ItemPopularityScore),
                ),
            ]
}
