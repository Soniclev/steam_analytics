use std::{collections::HashMap, time::Instant};

use crate::{MarketItem, MarketItemState};

use super::{
    global::{self, base::{GlobalMetricResult, MetricCalculation}},
    item::{self, metrics::{ItemMetricCalculation, ItemMetricResult}, static_metrics::compute_item_static_metrics},
};

pub struct MetricProcessor {
    global_metrics: Vec<Box<dyn MetricCalculation>>,
    item_metrics: Vec<Box<dyn ItemMetricCalculation>>,
}

impl MetricProcessor {
    pub fn new() -> Self {
        MetricProcessor {
            global_metrics: global::base::get_metrics(),
            item_metrics: item::base::get_metrics(),
        }
    }

    pub fn process_global(
        &self,
        items: &HashMap<String, MarketItem>,
    ) -> Vec<GlobalMetricResult> {
        self.global_metrics
            .iter()
            .filter(| metric| !metric.is_huge())
            .map(|metric| {
                let start_time = Instant::now();
                let value = metric.calculate(items);
                let duration_micros = start_time.elapsed().as_micros();
                GlobalMetricResult {
                    result: value,
                    duration_micros,
                }
            })
            .collect()
    }

    pub fn process_global_huge(&self, items: &HashMap<String, MarketItem>,) -> Vec<GlobalMetricResult> {
        self.global_metrics
            .iter()
            .filter(| metric| metric.is_huge())
            .map(| metric| {
                let start_time = Instant::now();
                let value = metric.calculate(items);
                let duration_micros = start_time.elapsed().as_micros();
                GlobalMetricResult {
                    result: value,
                    duration_micros,
                }
            })
            .collect()
    }

    pub fn process_item(&self, item: &mut MarketItem) -> Vec<ItemMetricResult> {
        if item.state == MarketItemState::Analyzed {
            return item.metrics.clone();
        }

        compute_item_static_metrics(item);

        self.item_metrics
            .iter()
            .map(|metric| {
                let start_time = Instant::now();
                let value = metric.calculate(item);
                let duration_micros = start_time.elapsed().as_micros();
                let item_metric_result = ItemMetricResult {
                    result: value,
                    duration_micros,
                };

                item.metrics.push(item_metric_result.clone());

                item_metric_result
            })
            .collect()
    }
}
