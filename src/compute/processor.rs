use std::{collections::HashMap, time::Instant};

use crate::MarketItem;

use super::{item_metrics::{ItemMetricCalculation, ItemMetricResult, ItemMetricType}, traits::{GlobalMetricResult, GlobalMetricType, MetricCalculation}};

pub struct MetricProcessor {
    global_metrics: Vec<(GlobalMetricType, Box<dyn MetricCalculation>)>,
    item_metrics: Vec<(ItemMetricType, Box<dyn ItemMetricCalculation>)>,
}

impl MetricProcessor {
    pub fn new() -> Self {
        MetricProcessor {
            global_metrics: Vec::new(),
            item_metrics: Vec::new(),
        }
    }

    pub fn add_global_metric(&mut self, metric_type: GlobalMetricType, metric: Box<dyn MetricCalculation>) {
        self.global_metrics.push((metric_type, metric));
    }

    pub fn add_item_metric(&mut self, metric_type: ItemMetricType, metric: Box<dyn ItemMetricCalculation>) {
        self.item_metrics.push((metric_type, metric));
    }

    pub fn process_global(&self, items: &HashMap<String, MarketItem>) -> Vec<GlobalMetricResult> {
        self.global_metrics.iter()
            .map(|(kind, metric)| {
                let start_time = Instant::now();
                let value = metric.calculate(items);
                let duration_micros = start_time.elapsed().as_micros();
                GlobalMetricResult {
                    kind: kind.clone(),
                    result: value,
                    duration_micros,
                }
    })
            .collect()
    }

    pub fn process_item(&self, item: &MarketItem) -> Vec<ItemMetricResult> {
        self.item_metrics.iter()
            .map(|(kind, metric)| {
                let start_time = Instant::now();
                let value = metric.calculate(item);
                let duration_micros = start_time.elapsed().as_micros();
                ItemMetricResult {
                    kind: kind.clone(),
                    result: value,
                    duration_micros,
                }
    })
            .collect()
    }
}
