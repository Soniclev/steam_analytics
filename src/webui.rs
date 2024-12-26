use std::collections::HashMap;
use std::time::Instant;

use actix_web::Responder;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::compute::global_metrics::{GlobalMetricResult, GlobalMetricType};
use crate::compute::item_metrics::ItemMetricResult;
use crate::compute::processor::MetricProcessor;
use crate::consts::EVENTS;
use crate::import::import_item;
use crate::{MarketItem, MarketItemShort};

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

    // list all items
    let items = data.items.lock().unwrap();
    let obj = ItemsApiResponse {
        global_stats: data.global_stats.lock().unwrap().clone(),
        items: items.values().map(|item| MarketItemShort {
            app_id: item.app_id,
            name: item.name.clone(),
            price: item.price.clone(),
            updated_at: item.updated_at.clone(),
            determine_item_category: item.determine_item_category.clone(),
            metrics: item.metrics.iter().map(|(_, value)| value.clone()).collect(),
        }).collect(),
        response_generation_duration: resp_gen_started.elapsed().as_micros(),
    };
    Ok(web::Json(obj))
}

#[derive(Serialize, Clone)]
pub struct GlobalStats {
    pub metrics: Vec<GlobalMetricResult>,
    pub huge_metrics: Vec<GlobalMetricResult>,
    pub total_items: u64,
    pub total_analyzed_items: u64,
}

impl GlobalStats {
    pub(crate) fn new() -> Self {
        Self { metrics: Vec::new(), huge_metrics: Vec::new(), total_items: 0, total_analyzed_items: 0 }
    }

    pub fn to_lite(&self) -> GlobalStats {
        // filter out metrics that we don't need
        let metrics = self.metrics.iter().filter(|f| f.kind != GlobalMetricType::CS2TotalItemsByCategory).map(|f| f.clone()).collect();
        GlobalStats { metrics, huge_metrics: self.huge_metrics.clone(), total_items: self.total_items, total_analyzed_items: self.total_analyzed_items }
    }
}


#[derive(Serialize, Clone, PartialEq)]
pub struct ItemCategoryStatsFull {
    pub total_items: u64,
    pub total_analyzed_items: u64,
    pub total_sold: u64,
    pub total_volume: f64,

    pub sold_per_day: HashMap<DateTime<Utc>, u64>,
    pub sold_per_month: HashMap<DateTime<Utc>, u64>,
}

impl ItemCategoryStatsFull {
    pub fn new() -> Self {
        Self { total_items: 0, total_analyzed_items: 0, total_sold: 0, total_volume: 0.0, sold_per_day: HashMap::new(), sold_per_month: HashMap::new() }
    }
}


#[derive(Serialize)]
struct ItemsApiResponse {
    response_generation_duration: u128,
    global_stats: GlobalStats,
    // cs2_categories: HashMap<ItemCategory, ItemCategoryStats>,
    items: Vec<MarketItemShort>,
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

#[derive(Serialize, Clone)]
pub struct ChartsApiResponse {
    events: Vec<(String, String, String)>,
    global_stats: GlobalStats,
}

pub async fn charts_handler(data: web::Data<AppStateWithCounter>) -> Result<impl Responder> {
    let global_stats = data.global_stats.lock().unwrap();


    Ok(web::Json(
        ChartsApiResponse {
            events: EVENTS
            .iter()
            .map(|(start, end, name)| (start.to_string(), end.to_string(), name.to_string()))
            .collect(),
            global_stats: global_stats.clone()
        }
)
)
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
