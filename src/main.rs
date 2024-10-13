use actix_web::{rt::{spawn, time::sleep}, web, App, HttpServer};
use chrono::{DateTime, Utc};
use compute::{item_metrics::{ItemMetricType, ItemMetricValue}, processor::MetricProcessor};
use prices::PriceValue;
use serde::Serialize;
use webui::GlobalStats;
use std::{collections::HashMap, sync::{Arc, Mutex}, time::Duration};

mod compute;
mod consts;
mod import;
mod mocked;
mod prices;
mod steam_analyzer;
mod webui;

#[derive(Serialize, Clone, Copy, PartialEq)]
enum MarketItemState {
    NotAnalyzed,
    Analyzed,
}

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
    state: MarketItemState,
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
    metrics: Vec<ItemMetricValue>,
    // metrics: HashMap<ItemMetricType, ItemMetricValue>,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
    items: Mutex<HashMap<String, MarketItem>>,
    global_stats: Mutex<GlobalStats>,
}

// Background task that periodically increments the counter
async fn run_background_task(data: Arc<AppStateWithCounter>) {
    loop {
        {           
            // println!("Background task processing items...");
            let mut items = data.items.lock().unwrap();
            // println!("Background task found {} items", items.len());
            let processor = MetricProcessor::new();
            let mut processed = 0;
            for (_, item) in items.iter_mut() {
                if processed > 100 {
                    break;
                }
                if item.state == MarketItemState::NotAnalyzed {
                    processor.process_item(item);
                    item.state = MarketItemState::Analyzed;
                    processed += 1;
                }
            }
            let global_metrics = processor.process_global(&items);
            let mut global_stats = data.global_stats.lock().unwrap();
            global_stats.metrics = global_metrics.into_iter().collect();
            global_stats.total_items = items.len() as u64;
            global_stats.total_analyzed_items = (items.iter().filter(|(_, item)| item.state == MarketItemState::Analyzed).count()) as u64;
        }
        sleep(Duration::from_millis(1)).await;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
        items: Mutex::new(HashMap::new()),
        global_stats: Mutex::new(GlobalStats::new()),
    });

    // Clone the Arc to move into the background task
    let bg_data = Arc::clone(&counter);

    // Spawn a background task
    spawn(async move {
        run_background_task(bg_data).await;
    });

    let import_bg_data = Arc::clone(&counter);
    // Spawn a background task
    spawn(async move {
        mocked::import_items_from_folder(&import_bg_data, "./src/mocked").await;
        println!("Imported {} items", import_bg_data.items.lock().unwrap().len());
    });


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
