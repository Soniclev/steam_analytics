use actix_web::{web, App, HttpServer};
use chrono::{DateTime, Utc};
use prices::PriceValue;
use std::{
    collections::HashMap,
    sync::Mutex,
};

mod consts;
mod import;
mod prices;
mod mocked;
mod steam_analyzer;
mod webui;

struct MarketItem {
    app_id: u64,
    name: String,
    price: PriceValue,

    updated_at: DateTime<Utc>,

    analyzes_result: Option<steam_analyzer::AnalysisResult>,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads

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

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(webui::index))
            .service(
                web::resource("/item/{app_id}/{market_name}")
                    .route(web::get().to(webui::user_detail)),
            )
        // ;
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
