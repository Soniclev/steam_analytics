use actix_web::{get, Error, http::{StatusCode}, post, web, App, HttpResponse, HttpServer, Responder, Result};
use chrono::{DateTime, TimeZone, Utc};
use consts::DESIRED_PERCENTILE;
use prices::{PriceValue, PriceValueTrait};
use regex::Regex;
use std::{collections::HashMap, fs, sync::Mutex, time::{Instant, UNIX_EPOCH}};
use steam_analyzer::analyze_steam_sell_history;

mod consts;
mod prices;
mod steam_analyzer;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn user_detail(data: web::Data<AppStateWithCounter>, params: web::Path<(u32,String)>) -> Result<HttpResponse, Error> {
    let (app_id, market_name) = params.into_inner();
    let items = data.items.lock().unwrap();
    if !items.contains_key(&market_name) {
        return Result::Ok(HttpResponse::NotFound().body(format!("Item not found")));
    }
    let item = items.get(&market_name).unwrap();
    let content = format!("
     Link: https://steamcommunity.com/market/listings/{app_id}/{market_name}\n
     App id: {app_id}\n
     Market name: {market_name}\n
     Price: ${}\n
     Updated at: {}\n
     Analyzes result: {:?}\n
     ", item.price.to_usd(), item.updated_at, item.analyzes_result.as_ref().unwrap());
    Result::Ok(HttpResponse::Ok().body(content))
}


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

fn import_item(page: &String, current_datetime: DateTime<Utc>) -> Option<MarketItem> {
    let re = Regex::new(r"<title>Steam Community Market :: Listings for (.+?)</title>").unwrap();

    let app_id_re = Regex::new(r#""appid":(\d+)"#).unwrap();

    // Apply the regex to find matches
    let app_id: Option<u64>;

    if let Some(caps) = app_id_re.captures(page) {
        // Capture the appid from the first capturing group
        if let Some(p_app_id) = caps.get(1) {
            app_id = Some(p_app_id.as_str().parse::<u64>().unwrap());
        }
        else {
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

markup::define! {
    Home<'a>(title: &'a str, content: &'a str) {
        @markup::doctype()
        html {
            head {
                title { @title }
                style {
                    "body { background: #fafbfc; }"
                    "#main { padding: 1rem; }"
                }
            }
            body {
                @Header { title }
                #main {
                    pre {
                        @content
                    }
                }
                @Footer { year: 2020 }
            }
        }
    }

    Header<'a>(title: &'a str) {
        header {
            h1 { @title }
        }
    }

    Footer(year: u32) {
        footer {
            "(c) " @year
        }
    }
}


async fn index(data: web::Data<AppStateWithCounter>) -> Result<HttpResponse> {
    let req_start = Instant::now();
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
        item_list.push_str(&format!(
            "{} {}: ${} (total {} pcs., volume ${:.2})  | processing duration: {} µs, imported {} ago\n",
            item.app_id,
            name,
            item.price.to_usd(),
            total_sold,
            total_volume,
            duration,
            now.signed_duration_since(item.updated_at).to_string()
        ));
    }

    let global_stats = format!(
        "Total sold: {} pcs., total volume: ${:.2}",
        total_sold, total_volume
    );

    let content = format!("Request number: {counter}\nTotal items: {total_items}\n\nGlobal stats:\n{global_stats}\n\nItems:\n{item_list}\n\nResponse generating duration: {} µs\n", req_start.elapsed().as_micros());


    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(
            format!(
                "{}",
                Home {
                    title: "Example Domain",
                    content: &content,
                }
            )
        )
    )

    // format!("Request number: {counter}\nTotal items: {total_items}\n\nGlobal stats:\n{global_stats}\n\nItems:\n{item_list}\n\nResponse generating duration: {} µs\n", req_start.elapsed().as_micros())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
        items: Mutex::new(HashMap::new()),
    });

    // load mocked data from folder ./mocked
    // list all files in the folder
    let mut files = fs::read_dir("./src/mocked").unwrap();
    while let Some(entry) = files.next() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".html") {
                let file = fs::read_to_string(path.clone()).unwrap();
                let file_saved_at = path.clone().metadata().unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis();
                let date = Utc.timestamp_millis_opt(file_saved_at.try_into().unwrap()).unwrap();
                let item = import_item(&file, date);
                if let Some(item) = item {
                    let mut items = counter.items.lock().unwrap();
                    items.insert(item.name.clone(), item);
                }
            }
        }
    }

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
            .service(
                web::resource("/item/{app_id}/{market_name}")
                    .route(web::get().to(user_detail)),
            )
            // ;    
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
