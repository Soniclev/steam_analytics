use std::collections::HashMap;

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    consts::DESIRED_PERCENTILE,
    prices::{PriceValue, PriceValueTrait},
    steam_analyzer::{analyze_steam_sell_history, extract_sell_history},
    MarketItem,
};

lazy_static! {
    static ref MARKET_NAME_REGEX: Regex =
        Regex::new(r#"<title>Steam Community Market :: Listings for (.+?)</title>"#).unwrap();
    static ref APP_ID_REGEX: Regex = Regex::new(r#""appid":(\d+)"#).unwrap();
}

pub fn import_item(page: &String, current_datetime: DateTime<Utc>) -> Option<MarketItem> {
    // Apply the regex to find matches
    let app_id: Option<u64>;

    if let Some(caps) = APP_ID_REGEX.captures(page) {
        // Capture the appid from the first capturing group
        if let Some(p_app_id) = caps.get(1) {
            app_id = Some(p_app_id.as_str().parse::<u64>().unwrap());
        } else {
            return None;
        }
    } else {
        return None;
    }

    // Apply the regex to find matches
    if let Some(caps) = MARKET_NAME_REGEX.captures(page) {
        // Capture the market name from the first capturing group
        if let Some(market_name) = caps.get(1) {
            // let current_datetime = Utc::now();
            let analysis_result = analyze_steam_sell_history(page, current_datetime);

            return Some(MarketItem {
                app_id: app_id.unwrap(),
                name: market_name.as_str().replace("&amp;", "&").to_string(),
                updated_at: current_datetime,
                history: extract_sell_history(&page)
                    .into_iter()
                    .map(|(date, price, amount)| (date, PriceValue::from_usd_f64(price), amount))
                    .collect(),
                analyzes_result: analysis_result.clone(),
                price: {
                    if analysis_result.is_some() {
                        analysis_result
                            .as_ref()
                            .unwrap()
                            .get_price_by_percentile(DESIRED_PERCENTILE)
                            .unwrap_or_else(|| PriceValue::from_usd_f64(0.0))
                    } else {
                        PriceValue::from_usd_f64(0.0)
                    }
                },

                metrics: HashMap::new(),
            });
        }
    }
    return None;
}
