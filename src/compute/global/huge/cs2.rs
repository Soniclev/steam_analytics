use std::collections::HashMap;

use chrono::{Datelike, Timelike};

use crate::{
    compute::{
        global::base::{GlobalMetricValue, MetricCalculation},
        item::metrics::ItemMetricValue,
    },
    game::cs2::ItemCategory,
    webui::ItemCategoryStatsFull,
    MarketItem, MarketItemState,
};
pub struct CS2TotalItemsByCategory;

impl MetricCalculation for CS2TotalItemsByCategory {
    fn to_string(&self) -> String {
        "CS2TotalItemsByCategory".to_string()
    }

    fn is_huge(&self) -> bool {
        true
    }

    fn calculate(&self, items: &HashMap<String, MarketItem>) -> GlobalMetricValue {
        let mut result: HashMap<ItemCategory, ItemCategoryStatsFull> = HashMap::new();

        for (_, item) in items.iter() {
            if item.app_id != 730 {
                continue;
            }

            let category = item.determine_item_category.clone();

            if !result.contains_key(&category) {
                result.insert(category.clone(), ItemCategoryStatsFull::new());
            }

            let value = result.get_mut(&category).unwrap();
            value.total_items += 1;

            match item.state {
                MarketItemState::Analyzed => {
                    value.total_analyzed_items += 1;

                    item.metrics.iter().for_each(|(_, metric)| match metric {
                        ItemMetricValue::TotalSold(sold) => value.total_sold += sold.clone(),
                        ItemMetricValue::TotalVolume(volume) => {
                            value.total_volume += volume.clone()
                        }
                        _ => {}
                    });

                    // iterate over history and count sold per month
                    let mut sold_per_day: HashMap<&chrono::DateTime<chrono::Utc>, u64> =
                        HashMap::new();

                    for (date, _price, amount) in item.history.iter() {
                        if let Some(count) = sold_per_day.get(&date) {
                            sold_per_day.insert(date, count + *amount as u64);
                        } else {
                            sold_per_day.insert(date, *amount as u64);
                        }
                    }

                    // sum up all sold per month
                    for (dt, amount) in sold_per_day.into_iter() {
                        if value.sold_per_day.contains_key(dt) {
                            value
                                .sold_per_day
                                .insert(dt.clone(), value.sold_per_day.get(dt).unwrap() + amount);
                        } else {
                            value.sold_per_day.insert(dt.clone(), amount);
                        }

                        let date_trunc_by_month = dt
                            .with_day(1)
                            .unwrap()
                            .with_hour(0)
                            .unwrap()
                            .with_minute(0)
                            .unwrap()
                            .with_second(0)
                            .unwrap();
                        if let Some(count) = value.sold_per_month.get(&date_trunc_by_month) {
                            value
                                .sold_per_month
                                .insert(date_trunc_by_month, count + amount);
                        } else {
                            value.sold_per_month.insert(date_trunc_by_month, amount);
                        }
                    }
                }
                _ => {}
            };
        }
        GlobalMetricValue::CS2TotalItemsByCategory(
            result
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        )
    }
}