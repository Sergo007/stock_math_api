#![allow(dead_code, unused_imports, unused_variables)]

use actix_files as fs;
use dotenv::dotenv;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::*;
use tracing_json2::JsonTracing;
mod basket;
mod bfs_alg;
mod branch_and_bound;
mod config;
mod distance_matrix_calculate;
mod logging;
mod middleware;
mod naive_solution_min_path;
mod transform_route;
mod travelling_salesman;
use actix_web::{error, middleware::Logger, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonTracing, Clone)]
struct PostCalculateOptimalPathRequest {
    request_id: Option<String>,
    geometry: Vec<Vec<char>>,
    points: Vec<(usize, usize)>,
}

#[derive(Serialize, Deserialize, JsonTracing, Clone)]
struct PostCalculateOptimalPathResponce {
    algorithm: String,
    distance_matrix_time: String,
    traveling_salesman_time: String,
    distance: f64,
    path: Vec<(usize, usize)>,
}

#[post("/calculate_optimal_path")]
async fn post_calculate_optimal_path(
    req: web::Json<PostCalculateOptimalPathRequest>,
) -> web::Json<PostCalculateOptimalPathResponce> {
    let mut resp = PostCalculateOptimalPathResponce {
        algorithm: "simulated_annealing".to_string(),
        distance_matrix_time: "".to_string(),
        traveling_salesman_time: "".to_string(),
        distance: -1.0,
        path: Vec::new(),
    };
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    resp.distance_matrix_time = format!("{:?}", now1);

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    resp.path = naive_solution_min_path::solve(&req.geometry, &req.points);
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    resp.traveling_salesman_time = format!("{:?}", now1);
    resp.distance = 0.0;
    info!(request = ?req.clone(), responce = ?resp.clone(), "handle request");
    actix_web::web::Json(resp)
}

#[ctor::ctor]
fn init() {
    dotenv().ok();
    logging::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("app start");
    info!("app start");
    let app_cfg = config::Application::from_env();
    info!("starting application {}:{}", app_cfg.url(), app_cfg.port());
    info!("geometry builder {}:{}/ui", app_cfg.url(), app_cfg.port());
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(1024 * 1024 * 10)
            .error_handler(|err, _req| {
                // create custom error response
                println!("{}", err.to_string());
                error::InternalError::from_response(
                    err.to_string(),
                    HttpResponse::BadRequest().body(err.to_string()),
                )
                .into()
            });
        App::new()
            .wrap(Logger::default())
            .app_data(json_config)
            .wrap(middleware::cors::cors())
            .service(post_calculate_optimal_path)
            .service(
                fs::Files::new("/ui", "./ui-admin")
                    .show_files_listing()
                    .use_last_modified(true)
                    .index_file("index.html"),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
