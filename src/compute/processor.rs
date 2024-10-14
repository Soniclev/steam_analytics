use std::{collections::HashMap, time::Instant};

use crate::MarketItem;

use super::{
    global_metrics::{self, GlobalMetricResult, GlobalMetricType, MetricCalculation},
    item_metrics::{self, ItemMetricCalculation, ItemMetricResult, ItemMetricType},
};

pub struct MetricProcessor {
    global_metrics: Vec<(GlobalMetricType, Box<dyn MetricCalculation>)>,
    item_metrics: Vec<(ItemMetricType, Box<dyn ItemMetricCalculation>)>,
}

impl MetricProcessor {
    pub fn new() -> Self {
        MetricProcessor {
            global_metrics: vec![
                (
                    GlobalMetricType::TotalSold,
                    Box::new(global_metrics::TotalSold),
                ),
                (
                    GlobalMetricType::AveragePrice,
                    Box::new(global_metrics::AveragePrice),
                ),
                (
                    GlobalMetricType::TotalVolume,
                    Box::new(global_metrics::TotalVolume),
                ),
                (
                    GlobalMetricType::SteamEstimatedFee,
                    Box::new(global_metrics::SteamEstimatedFee),
                ),
                (
                    GlobalMetricType::GameEstimatedFee,
                    Box::new(global_metrics::GameEstimatedFee),
                ),
                (
                    GlobalMetricType::ValveEstimatedFee,
                    Box::new(global_metrics::ValveEstimatedFee),
                ),
                (
                    GlobalMetricType::Test,
                    Box::new(global_metrics::Test),
                )
            ],
            item_metrics: vec![
                (
                    ItemMetricType::ItemTotalSold,
                    Box::new(item_metrics::ItemTotalSold),
                ),
                (
                    ItemMetricType::ItemTotalVolume,
                    Box::new(item_metrics::ItemTotalVolume),
                ),
                (
                    ItemMetricType::ItemSteamEstimatedFee,
                    Box::new(item_metrics::ItemSteamEstimatedFee),
                ),
                (
                    ItemMetricType::ItemGameEstimatedFee,
                    Box::new(item_metrics::ItemGameEstimatedFee),
                ),
                (
                    ItemMetricType::ItemValveEstimatedFee,
                    Box::new(item_metrics::ItemValveEstimatedFee),
                ),
                (
                    ItemMetricType::ItemPopularityScore,
                    Box::new(item_metrics::ItemPopularityScore),
                ),
            ],
        }
    }

    pub fn process_global(
        &self,
        items: &HashMap<String, MarketItem>,
    ) -> Vec<GlobalMetricResult> {
        // first pass to calculate all item metrics
        // for (_, item) in items.iter_mut() {
        //     self.process_item(item);
        // }

        self.global_metrics
            .iter()
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

    pub fn process_item(&self, item: &mut MarketItem) -> Vec<ItemMetricResult> {
        self.item_metrics
            .iter()
            .map(|(kind, metric)| {
                if item.metrics.contains_key(kind) {
                    return ItemMetricResult {
                        kind: kind.clone(),
                        result: item.metrics.get(kind).unwrap().clone(),
                        duration_micros: 0,
                    };
                } else {
                    let start_time = Instant::now();
                    let value = metric.calculate(item);
                    let duration_micros = start_time.elapsed().as_micros();
                    let item_metric_result = ItemMetricResult {
                        kind: kind.clone(),
                        result: value,
                        duration_micros,
                    };

                    item.metrics
                        .insert(kind.clone(), item_metric_result.result.clone());

                    item_metric_result
                }
            })
            .collect()
    }
}
