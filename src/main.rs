#![allow(dead_code, unused_imports, unused_variables, deprecated)]

use actix_files as fs;
use dotenv::dotenv;
use std::{
    env,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::*;
use tracing_json2::JsonTracing;
mod anonymize_emails;
mod app_error;
mod basket;
mod bfs_alg;
mod branch_and_bound;
mod config;
mod db;
mod distance_matrix_calculate;
mod logging;
mod middleware;
mod naive_solution_min_path;
mod transform_route;
mod travelling_salesman;
use actix_web::{error, middleware::Logger, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

use sqlx::postgres::PgPoolOptions;

use crate::db::MedicationMappingsSet;

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
    // dotenv::from_filename(".env").ok();
    let iter = dotenv::from_filename_iter(".env").unwrap();
    for item in iter {
        let (key, val) = item.unwrap();
        env::set_var(&key, val);
    }
    logging::init();
}

#[actix_web::main]
async fn main() -> Result<(), app_error::AppError> {
    // dotenv::from_filename(".env").ok();

    println!("app start");
    info!("app start");
    let app_cfg = config::Application::from_env();
    let pg_cfg = config::Postgresql::from_env();
    info!("starting application {}:{}", app_cfg.url(), app_cfg.port());
    info!("geometry builder {}:{}/ui", app_cfg.url(), app_cfg.port());
    info!("PG: {}", pg_cfg.url());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(pg_cfg.url().as_str())
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let mm = MedicationMappingsSet {};
    let mms = mm
        .many_by_medication_mapping_id_list(&pool, vec![1])
        .await?;

    info!("medication_mapping {:?}", mms);

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    info!("SELECT1 {}", row.0);

    Ok(HttpServer::new(|| {
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
    .await?)
}
