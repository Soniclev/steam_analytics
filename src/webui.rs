use std::time::Instant;

use actix_web::http::StatusCode;
use actix_web::Error;
use chrono::Utc;

use crate::prices;

use super::AppStateWithCounter;

use actix_web::HttpResponse;

use actix_web::Responder;
use actix_web::{Result, web};
use prices::{PriceValue, PriceValueTrait};


pub async fn index(data: web::Data<AppStateWithCounter>) -> Result<HttpResponse> {
    let resp_gen_started = Instant::now();
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    // list all items
    let items = data.items.lock().unwrap();
    let total_items = items.len();
    let total_sold = items
        .iter()
        .map(|x| x.1.analyzes_result.as_ref().unwrap().total_sold)
        .sum::<u64>();
    let total_volume = items
        .iter()
        .map(|x| x.1.analyzes_result.as_ref().unwrap().total_volume)
        .sum::<f64>();
    let mut item_list = String::new();

    let now = Utc::now();

    for (name, item) in items.iter() {
        let total_sold = {
            if item.analyzes_result.is_some() {
                item.analyzes_result.as_ref().unwrap().total_sold
            } else {
                0
            }
        };
        let total_volume = {
            if item.analyzes_result.is_some() {
                item.analyzes_result.as_ref().unwrap().total_volume
            } else {
                0.0
            }
        };
        let duration = {
            if item.analyzes_result.is_some() {
                item.analyzes_result.as_ref().unwrap().duration_micros
            } else {
                0
            }
        };

        let link_with_name = format!(
            "<a href=\"/item/{}/{}\">{} {}</a>",
            item.app_id, name, item.app_id, name
        );
        let brief_info = format!(
            "${} (total {} pcs., volume ${:.2})",
            item.price.to_usd(),
            total_sold,
            total_volume
        );
        let processing_stats = format!(
            "processing duration: {} µs, imported {} ago",
            duration,
            now.signed_duration_since(item.updated_at).to_string()
        );

        item_list.push_str(&format!(
            "<p>{link_with_name}: {brief_info}   | {processing_stats}</p>\n",
        ));
    }

    let global_stats = format!(
        "Total sold: {} pcs., total volume: ${:.2}",
        total_sold, total_volume
    );

    let content = format!("<p>Request number: {counter}</p>\n<p>Total items: {total_items}</p>\n\n<p>Global stats:</p>\n<p>{global_stats}</p>\n<hr>\n<p>Items:</p>\n{item_list}\n\n<p>Response generating duration: {} µs</p>\n", resp_gen_started.elapsed().as_micros());

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(content))
}

pub async fn user_detail(
    data: web::Data<AppStateWithCounter>,
    params: web::Path<(u32, String)>,
) -> Result<HttpResponse, Error> {
    let (app_id, market_name) = params.into_inner();
    let items = data.items.lock().unwrap();
    if !items.contains_key(&market_name) {
        return Result::Ok(HttpResponse::NotFound().body(format!("Item not found")));
    }
    let item = items.get(&market_name).unwrap();
    let content = format!(
        "
     Link: https://steamcommunity.com/market/listings/{app_id}/{market_name}\n
     App id: {app_id}\n
     Market name: {market_name}\n
     Price: ${}\n
     Updated at: {}\n
     Analyzes result: {:?}\n
     ",
        item.price.to_usd(),
        item.updated_at,
        item.analyzes_result.as_ref().unwrap()
    );
    Result::Ok(HttpResponse::Ok().body(content))
}
