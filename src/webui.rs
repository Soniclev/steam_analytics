use std::time::Instant;

use actix_web::http::StatusCode;
use actix_web::Error;
use chrono::Utc;

use crate::compute::processor::MetricProcessor;
use crate::compute::traits::AveragePrice;
use crate::compute::traits::MetricType;
use crate::compute::traits::TotalSold;
use crate::compute::traits::TotalVolume;
use crate::prices;

use super::AppStateWithCounter;

use actix_web::HttpResponse;

use actix_web::{Result, web};
use prices::PriceValueTrait;


pub async fn chart_handler() -> Result<HttpResponse> {
    let data = serde_json::json!({
        "labels": ["January", "February", "March", "April", "May", "June"],
        "datasets": [
            {
                "label": "My First Dataset",
                "data": [65, 59, 80, 81, 56, 55],
                "borderColor": "rgba(75, 192, 192, 1)",
                "fill": false
            },
            {
                "label": "My Second Dataset",
                "data": [28, 48, 40, 19, 86, 27],
                "borderColor": "rgba(153, 102, 255, 1)",
                "fill": false
            }
        ]
    });

    let html = format!(r#"
    <html>
        <head>
            <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
            <script src="https://cdn.jsdelivr.net/npm/hammerjs@2.0.8"></script>
            <script src="https://cdn.jsdelivr.net/npm/chartjs-plugin-zoom@2.0.1"></script>
        </head>
        <body>
            <canvas id="myChart" width="400" height="200"></canvas>
            <script>
                const ctx = document.getElementById('myChart').getContext('2d');
                const data = {data};  // Here is the chart data from Rust
                const config = {{
                    type: 'line',
                    data: data,
                    options: {{
                      plugins: {{
                        zoom: {{
                          pan: {{
                              enabled: true,
                              mode: 'xy',
                            }},
                          zoom: {{
                            
                            wheel: {{
                              enabled: true,
                            }},
                            pinch: {{
                              enabled: true,
                            }},
                            mode: 'xy',
                          }}
                        }}
                      }}
                    }}  
                }};
                new Chart(ctx, config);
            </script>
        </body>
    </html>
    "#, data = data);

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}



pub async fn index(data: web::Data<AppStateWithCounter>) -> Result<HttpResponse> {
    let resp_gen_started = Instant::now();
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    // list all items
    let items = data.items.lock().unwrap();
    let total_items = items.len();

    let mut processor = MetricProcessor::new();
    processor.add_metric(MetricType::TotalSold, Box::new(TotalSold));
    processor.add_metric(MetricType::AveragePrice, Box::new(AveragePrice));
    processor.add_metric(MetricType::TotalVolume, Box::new(TotalVolume));

    let results = processor.process(&items);

    let mut metrics_html = String::new();
    for result in results {
        metrics_html.push_str(&format!("<p>{}</p>\n", result.to_html()));
    }

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

    let global_stats = format!("{metrics_html}");

    let content = format!("<p>Request number: {counter}</p>\n<p>Total items: {total_items}</p>\n\n<p>Global stats:</p>\n<p>{global_stats}</p>\n<hr>\n<p>Items:</p>\n{item_list}\n\n<p>Response generating duration: {} µs</p>\n", resp_gen_started.elapsed().as_micros());

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(content))
}

pub async fn market_item_detail(
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
