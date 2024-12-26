use crate::compute::item::metrics::{self};

use super::metrics::ItemMetricCalculation;

pub fn get_metrics() -> Vec<Box<dyn ItemMetricCalculation>> {
    vec![
        Box::new(metrics::ItemTotalSold),
        Box::new(metrics::ItemTotalVolume),
        Box::new(metrics::ItemSteamEstimatedFee),
        Box::new(metrics::ItemGameEstimatedFee),
        Box::new(metrics::ItemValveEstimatedFee),
        Box::new(metrics::ItemPopularityScore),
    ]
}
