use crate::compute::item::metrics::{self};

use super::metrics::ItemMetricCalculation;

pub fn get_metrics() -> Vec<Box<dyn ItemMetricCalculation>> {
    vec![
        Box::new(metrics::ItemPopularityScore),
    ]
}
