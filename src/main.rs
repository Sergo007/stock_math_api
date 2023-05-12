use std::time::{SystemTime, UNIX_EPOCH};
mod bfs_alg;
mod branch_and_bound;
mod distance_matrix_calculate;
mod logging;
mod middleware;
mod transform_route;
mod travelling_salesman;
use actix_web::{error, middleware::Logger, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PostCalculateOptimalPathRequest {
    geometry: Vec<Vec<char>>,
    points: Vec<(usize, usize)>,
}

#[derive(Serialize)]
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
    let matrix =
        distance_matrix_calculate::get_distances_matrix(req.points.as_slice(), |point1, point2| {
            let path = bfs_alg::bfs(&(req.geometry), *point1, *point2, |item| item == 'W');
            match path {
                Some(v) => v.len() as f64,
                None => -1.0,
            }
        });
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    resp.distance_matrix_time = format!("{:?}", now1);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let tour = travelling_salesman::simulated_annealing::solve_by_distance_matrix(
        &matrix,
        time::Duration::seconds(1),
    );
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    resp.traveling_salesman_time = format!("{:?}", now1);
    resp.distance = tour.distance;
    resp.path = transform_route::get_transformed_route(req.points.as_slice(), tour.route);

    actix_web::web::Json(resp)
}

#[derive(Serialize)]
struct PostCalculateDistancesMatrixResponce {
    time: String,
    matrix: Vec<Vec<f64>>,
}

#[post("/calc_distances_matrix")]
async fn post_calc_distances_matrix(
    req: web::Json<PostCalculateOptimalPathRequest>,
) -> web::Json<PostCalculateDistancesMatrixResponce> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let matrix =
        distance_matrix_calculate::get_distances_matrix(req.points.as_slice(), |point1, point2| {
            let path = bfs_alg::bfs(&(req.geometry), *point1, *point2, |item| item == 'W');
            match path {
                Some(v) => v.len() as f64,
                None => -1.0,
            }
        });
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;

    let resp = PostCalculateDistancesMatrixResponce {
        time: format!("{:?}", now1),
        matrix,
    };
    actix_web::web::Json(resp)
}

#[post("/calc_distances_matrix_text")]
async fn post_calc_distances_matrix_text(
    req: web::Json<PostCalculateOptimalPathRequest>,
) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let matrix =
        distance_matrix_calculate::get_distances_matrix(req.points.as_slice(), |point1, point2| {
            let path = bfs_alg::bfs(&(req.geometry), *point1, *point2, |item| item == 'W');
            match path {
                Some(v) => v.len() as f64,
                None => -1.0,
            }
        });
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    let mut resp = "".to_string();
    resp = resp + "{\n";
    let n = matrix.len();
    for i in 0..n {
        resp = resp + "\t{";
        for j in 0..n {
            if i == j {
                if j != (n - 1) {
                    resp = resp + "INF, ";
                } else {
                    resp = resp + "INF";
                }
                continue;
            }
            if j != (n - 1) {
                resp = resp + &format!("{:}, ", matrix[i][j]).to_string();
            } else {
                resp = resp + &format!("{:}", matrix[i][j]).to_string();
            }
        }
        resp = resp + "},\n";
    }
    resp = resp + "}\n";
    resp
}

#[ctor::ctor]
fn init() {
    // dotenv().ok();
    logging::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("app start");
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
            .service(post_calc_distances_matrix)
            .service(post_calc_distances_matrix_text)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
