use actix_web::{web, App, HttpServer};
use chrono::{DateTime, Utc};
use compute::item_metrics::{ItemMetricType, ItemMetricValue};
use prices::PriceValue;
use serde::Serialize;
use std::{collections::HashMap, sync::Mutex};

mod compute;
mod consts;
mod import;
mod mocked;
mod prices;
mod steam_analyzer;
mod webui;

#[derive(Serialize, Clone)]
struct MarketItem {
    app_id: u64,
    name: String,
    price: PriceValue,

    updated_at: DateTime<Utc>,

    history: Vec<(DateTime<Utc>, PriceValue, i32)>,
    analyzes_result: Option<steam_analyzer::AnalysisResult>,

    // metrics
    metrics: HashMap<ItemMetricType, ItemMetricValue>,
}

#[derive(Serialize, Clone)]
struct MarketItemShort {
    app_id: u64,
    name: String,
    price: PriceValue,

    updated_at: DateTime<Utc>,

    // history: Vec<(DateTime<Utc>, PriceValue, i32)>,
    // analyzes_result: Option<steam_analyzer::AnalysisResult>,

    // metrics
    metrics: HashMap<ItemMetricType, ItemMetricValue>,
}

struct AppStateWithCounter {
    cccccccc
        cc
        dsdfdsfds


    items: Mutex<HashMap<String, MarketItem>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
        items: Mutex::new(HashMap::new()),
    });

    mocked::import_items_from_folder(&counter, "./src/mocked");
    println!("Imported {} items", counter.items.lock().unwrap().len());

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .service(web::redirect("/", "/static/index.html"))
            .route("/api/items", web::get().to(webui::items_api_handler))
            .route(
                "/api/item/{app_id}/{market_name}",
                web::get().to(webui::item_detail_api_handler),
            )
            .route("/api/events", web::get().to(webui::events_api_handler))
            .route("/api/import", web::post().to(webui::import_handler))
            .service(
                web::resource("/static/{filename}").route(web::get().to(webui::static_handler)),
            ) // serve static files
              // ;
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
