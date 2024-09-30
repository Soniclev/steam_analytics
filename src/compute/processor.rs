use std::collections::HashMap;

use crate::MarketItem;

use super::traits::{MetricCalculation, MetricResult, MetricType};

pub struct MetricProcessor {
    metrics: Vec<(MetricType, Box<dyn MetricCalculation>)>,
}

impl MetricProcessor {
    pub fn new() -> Self {
        MetricProcessor {
            metrics: Vec::new(),
        }
    }

    pub fn add_metric(&mut self, metric_type: MetricType, metric: Box<dyn MetricCalculation>) {
        self.metrics.push((metric_type, metric));
    }

    pub fn process(&self, items: &HashMap<String, MarketItem>) -> Vec<MetricResult> {
        self.metrics.iter()
            .map(|(_, metric)| metric.calculate(items))
            .collect()
    }
}
