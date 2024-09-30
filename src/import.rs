use chrono::{DateTime, Utc};
use regex::Regex;

use crate::{consts::DESIRED_PERCENTILE, prices::{PriceValue, PriceValueTrait}, steam_analyzer::analyze_steam_sell_history, MarketItem};



pub fn import_item(page: &String, current_datetime: DateTime<Utc>) -> Option<MarketItem> {
    let re = Regex::new(r"<title>Steam Community Market :: Listings for (.+?)</title>").unwrap();

    let app_id_re = Regex::new(r#""appid":(\d+)"#).unwrap();

    // Apply the regex to find matches
    let app_id: Option<u64>;

    if let Some(caps) = app_id_re.captures(page) {
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
    if let Some(caps) = re.captures(page) {
        // Capture the market name from the first capturing group
        if let Some(market_name) = caps.get(1) {
            // let current_datetime = Utc::now();
            let analysis_result = analyze_steam_sell_history(page, current_datetime);

            return Some(MarketItem {
                app_id: app_id.unwrap(),
                name: market_name.as_str().replace("&amp;", "&").to_string(),
                updated_at: current_datetime,
                analyzes_result: analysis_result.clone(),
                price: {
                    if analysis_result.is_some() {
                        analysis_result
                            .as_ref()
                            .unwrap()
                            .get_price_by_percentile(DESIRED_PERCENTILE)
                            .unwrap()
                    } else {
                        PriceValue::from_usd_f64(0.0)
                    }
                },
            });
        }
    }
    return None;
}