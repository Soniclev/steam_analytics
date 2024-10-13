use actix_web::{
    rt::{spawn, time::sleep},
    web, App, HttpRequest, HttpServer, Responder,
};
use actix_ws::Message;
use chrono::{DateTime, Utc};
use compute::{
    item_metrics::{ItemMetricType, ItemMetricValue},
    processor::MetricProcessor,
};
use futures::StreamExt as _;
use prices::PriceValue;
use serde::Serialize;
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
    time::Duration,
};
use webui::GlobalStats;

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

    // metrics
    metrics: Vec<ItemMetricValue>,}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
    items: Mutex<HashMap<String, MarketItem>>,
    items_to_process: Mutex<VecDeque<String>>, // Queue of market names to process
    global_stats: Mutex<GlobalStats>,
}

// Background task that periodically increments the counter
async fn run_background_task(data: Arc<AppStateWithCounter>) {
    loop {
        {
            let mut items_to_process = data.items_to_process.lock().unwrap();

            if !items_to_process.is_empty() {
                println!("Handling {} items", items_to_process.len());
                let mut items = data.items.lock().unwrap();
                let processor = MetricProcessor::new();
                let mut processed = 0;

                while let Some(market_name) = items_to_process.pop_front() {
                    if processed > 1000 {
                        break;
                    }
                    let item = items.get_mut(&market_name).unwrap();
                    if item.state == MarketItemState::NotAnalyzed {
                        processor.process_item(item);
                        item.state = MarketItemState::Analyzed;
                        processed += 1;
                    }
                }
                if processed > 0 {
                    let global_metrics = processor.process_global(&items);
                    let mut global_stats = data.global_stats.lock().unwrap();
                    global_stats.metrics = global_metrics.into_iter().collect();
                    global_stats.total_items = items.len() as u64;
                    global_stats.total_analyzed_items = (items
                        .iter()
                        .filter(|(_, item)| item.state == MarketItemState::Analyzed)
                        .count()) as u64;
                }
            }
        }
        sleep(Duration::from_millis(1)).await;
    }
}

async fn ws(
    req: HttpRequest,
    body: web::Payload,
    data: web::Data<AppStateWithCounter>,
) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    println!("Connected");

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(_) => {
                    // println!("Got text: {msg}");
                    let global_metrics = data.global_stats.lock().unwrap().clone();
                    // serialize global metrics to json
                    let json = serde_json::to_string(&global_metrics).unwrap();
                    let _ = session.text(json).await;
                }
                Message::Binary(bytes) => println!("Got binary: {bytes:?}"),
                Message::Continuation(item) => println!("Got continuation: {item:?}"),
                Message::Pong(bytes) => println!("Got pong: {bytes:?}"),
                Message::Close(close_reason) => println!("Got close: {close_reason:?}"),
                Message::Nop => (),
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
        items: Mutex::new(HashMap::new()),
        items_to_process: Mutex::new(VecDeque::new()),
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
        println!(
            "Imported {} items",
            import_bg_data.items.lock().unwrap().len()
        );
    });

    // tx.

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
            .route("/ws", web::get().to(ws))
            .service(
                web::resource("/static/{filename}").route(web::get().to(webui::static_handler)),
            ) // serve static files
              // ;
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
