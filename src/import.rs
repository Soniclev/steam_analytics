use chrono::{DateTime, NaiveDateTime, NaiveTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    compute::item::static_metrics::StaticMetrics, consts::DESIRED_PERCENTILE, game::cs2::determine_item_category, prices::{PriceValue, PriceValueTrait}, steam_analyzer::{analyze_steam_sell_history, extract_sell_history}, MarketItem, MarketItemState
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
            let extracted_history = extract_sell_history(&page);
            let analysis_result = analyze_steam_sell_history(&extracted_history, current_datetime);

            let market_name_clean = market_name.as_str().replace("&amp;", "&").replace("%20", "").to_string();

            let updated_at = if let Some(last) = extracted_history.last() {
                DateTime::from_naive_utc_and_offset(NaiveDateTime::new(last.0, NaiveTime::MIN), Utc)
            } else {
                current_datetime
            };

            return Some(MarketItem {
                app_id: app_id.unwrap(),
                name: market_name_clean.clone(),
                updated_at: updated_at,
                history: extracted_history
                    .into_iter()
                    .map(|(date, price, amount)| (date, PriceValue::from_usd_f64(price) as u32, amount as u32))
                    .collect(),
                analyzes_result: analysis_result.clone(),
                static_metrics: StaticMetrics::new(),
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

                determine_item_category: determine_item_category(&market_name_clean),

                metrics: Vec::new(),
                state: MarketItemState::NotAnalyzed,
            });
        }
    }
    return None;
}
