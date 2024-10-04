use std::{collections::HashMap, time::Instant};

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
            .map(|(kind, metric)| {
                let start_time = Instant::now();
                let value = metric.calculate(items);
                let duration_micros = start_time.elapsed().as_micros();
                MetricResult {
                    kind: kind.clone(),
                    result: value,
                    duration_micros,
                }
    })
            .collect()
    }
}
