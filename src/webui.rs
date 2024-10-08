use std::collections::HashMap;
use std::time::Instant;

use actix_web::Responder;
use chrono::Utc;
use serde::Serialize;

use crate::compute::global_metrics::GlobalMetricResult;
use crate::compute::item_metrics::ItemMetricResult;
use crate::compute::processor::MetricProcessor;
use crate::consts::EVENTS;
use crate::import::import_item;
use crate::MarketItem;

use super::AppStateWithCounter;

use actix_web::HttpResponse;

use actix_web::{web, Result};

pub async fn import_handler(
    req_body: web::Bytes,
    data: web::Data<AppStateWithCounter>,
) -> Result<HttpResponse> {
    let req_body_str = String::from_utf8(req_body.to_vec()).unwrap();
    let item: Option<MarketItem> = import_item(&req_body_str, Utc::now());

    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    if let Some(item) = item {
        let mut items = data.items.lock().unwrap();
        items.insert(item.name.clone(), item);
    }

    Ok(HttpResponse::Ok().body("ok"))
}

pub async fn items_api_handler(data: web::Data<AppStateWithCounter>) -> Result<impl Responder> {
    let resp_gen_started = Instant::now();
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    // list all items
    let mut items = data.items.lock().unwrap();
    let total_items = items.len();

    let processor = MetricProcessor::new();
    let results = processor.process_global(&mut items);

    let obj = ItemsApiResponse {
        total_items: (total_items as u64),
        global_stats: GlobalStats {
            total_items: (total_items as u64),
            metrics: results
                .into_iter()
                .map(|r| (r.kind.to_string(), r))
                .collect(),
        },
        items: items.clone().into_values().collect(),
        response_generation_duration: resp_gen_started.elapsed().as_micros(),
    };
    Ok(web::Json(obj))
}

#[derive(Serialize)]
struct GlobalStats {
    metrics: HashMap<String, GlobalMetricResult>,
    total_items: u64,
}

#[derive(Serialize)]
struct ItemsApiResponse {
    total_items: u64,
    response_generation_duration: u128,
    global_stats: GlobalStats,
    items: Vec<MarketItem>,
}

pub async fn static_handler(path: web::Path<String>) -> HttpResponse {
    let path = path.into_inner();
    let content = std::fs::read_to_string(format!("./src/static/{}", path)).unwrap();
    HttpResponse::Ok().body(content)
}

#[derive(Serialize)]
struct ItemApiResponse {
    item: MarketItem,
    events: Vec<(String, String, String)>,
    item_metrics: HashMap<String, ItemMetricResult>,
    response_generation_duration: u128,
}

pub async fn item_detail_api_handler(
    data: web::Data<AppStateWithCounter>,
    params: web::Path<(u64, String)>,
) -> Result<impl Responder> {
    let resp_gen_started = Instant::now();
    let (app_id, market_name) = params.into_inner();
    let mut items = data.items.lock().unwrap();

    if !items.contains_key(&market_name) {
        return Err(actix_web::error::ErrorNotFound(format!(
            "Item {app_id} - {market_name} not found"
        )));
    };

    let mut item = items.get_mut(&market_name).unwrap();

    let processor = MetricProcessor::new();
    let results = processor.process_item(&mut item);

    let obj = ItemApiResponse {
        item: item.clone(),
        events: EVENTS
            .iter()
            .map(|(start, end, name)| (start.to_string(), end.to_string(), name.to_string()))
            .collect(),
        item_metrics: results
            .into_iter()
            .map(|r| (r.kind.to_string(), r))
            .collect(),
        response_generation_duration: resp_gen_started.elapsed().as_micros(),
    };

    Ok(web::Json(obj))
}

#[derive(Serialize)]
struct EventsApiResponse {
    events: Vec<(String, String, String)>,
}

pub async fn events_api_handler() -> Result<impl Responder> {
    return Ok(web::Json(EventsApiResponse {
        events: EVENTS
            .iter()
            .map(|(start, end, name)| (start.to_string(), end.to_string(), name.to_string()))
            .collect(),
    }));
}
