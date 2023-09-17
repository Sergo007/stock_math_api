#![allow(dead_code, unused_imports, unused_variables)]

use actix_files as fs;
use std::time::{SystemTime, UNIX_EPOCH};
mod basket;
mod bfs_alg;
mod branch_and_bound;
mod distance_matrix_calculate;
mod logging;
mod middleware;
mod naive_solution_min_path;
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
// todo https://habr.com/ru/articles/701458/
// https://synset.com/ai/ru/tsp/Salesman_Intro.html
// http://extremal-mechanics.org/archives/13094
// https://ru.stackoverflow.com/questions/482044/%D0%97%D0%B0%D0%B4%D0%B0%D1%87%D0%B0-%D0%BA%D0%BE%D0%BC%D0%BC%D0%B8%D0%B2%D0%BE%D1%8F%D0%B6%D0%B5%D1%80%D0%B0-%D0%BD%D0%B0-php-%D0%9F%D0%BE%D0%B8%D1%81%D0%BA-%D0%BA%D1%80%D0%B0%D1%82%D1%87%D0%B0%D0%B9%D1%88%D0%B5%D0%B3%D0%BE-%D0%BC%D0%B0%D1%80%D1%88%D1%80%D1%83%D1%82%D0%B0
// https://www.lancaster.ac.uk/fas/psych/software/TSP/TSP.html
// https://demonstrations.wolfram.com/TheTravelingSalesmanProblem22OptRemovalOfIntersections/
// https://www.geeksforgeeks.org/travelling-salesman-problem-using-dynamic-programming/
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
    // let matrix_distance =
    //     distance_matrix_calculate::get_distances_matrix(req.points.as_slice(), |point1, point2| {
    //         let path = bfs_alg::bfs(&(req.geometry), *point1, *point2, |item| item == 'W');
    //         match path {
    //             Some(v) => v.len() as f64,
    //             None => -1.0,
    //         }
    //     });
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    resp.distance_matrix_time = format!("{:?}", now1);

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    resp.path = naive_solution_min_path::solve(&req.geometry, &req.points);
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    resp.traveling_salesman_time = format!("{:?}", now1);
    resp.distance = 0.0;

    actix_web::web::Json(resp)
}

// todo https://habr.com/ru/articles/701458/
// https://synset.com/ai/ru/tsp/Salesman_Intro.html
// http://extremal-mechanics.org/archives/13094
// https://ru.stackoverflow.com/questions/482044/%D0%97%D0%B0%D0%B4%D0%B0%D1%87%D0%B0-%D0%BA%D0%BE%D0%BC%D0%BC%D0%B8%D0%B2%D0%BE%D1%8F%D0%B6%D0%B5%D1%80%D0%B0-%D0%BD%D0%B0-php-%D0%9F%D0%BE%D0%B8%D1%81%D0%BA-%D0%BA%D1%80%D0%B0%D1%82%D1%87%D0%B0%D0%B9%D1%88%D0%B5%D0%B3%D0%BE-%D0%BC%D0%B0%D1%80%D1%88%D1%80%D1%83%D1%82%D0%B0
// https://www.lancaster.ac.uk/fas/psych/software/TSP/TSP.html
// https://demonstrations.wolfram.com/TheTravelingSalesmanProblem22OptRemovalOfIntersections/
// https://www.geeksforgeeks.org/travelling-salesman-problem-using-dynamic-programming/
#[post("/calculate_optimal_path_old")]
async fn post_calculate_optimal_path_old(
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
    let matrix_distance =
        distance_matrix_calculate::get_distances_matrix(req.points.as_slice(), |point1, point2| {
            let path = bfs_alg::bfs(&(req.geometry), *point1, *point2, |item| item == 'W');
            match path {
                Some(v) => v.len() as f64,
                None => -1.0,
            }
        });
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    resp.distance_matrix_time = format!("{:?}", now1);

    // let matrix = basket::sum_mx(
    //     &basket::normalized_matrix(&matrix_distance),
    //     &basket::normalized_matrix(&basket::build_basket_weight_matrix(&vec![
    //         1.0;
    //         matrix_distance
    //             .len()
    //     ])),
    // );
    // let matrix = basket::sum_mx(
    //     &basket::normalized_matrix(&matrix_distance),
    //     &basket::build_basket_weight_matrix(&vec![1.0; matrix_distance.len()]),
    // );
    // let matrix = basket::build_basket_weight_matrix(&vec![1.0; matrix_distance.len()]);
    // travelling_salesman calculation
    if matrix_distance.len() > 10 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let tour = travelling_salesman::simulated_annealing::solve_by_distance_matrix(
            &matrix_distance,
            time::Duration::seconds(1),
        );
        let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
        resp.traveling_salesman_time = format!("{:?}", now1);
        resp.distance = tour.distance;
        resp.path = transform_route::get_transformed_route(req.points.as_slice(), tour.route);
    } else {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let tour = travelling_salesman::brute_force::solve_by_distance_matrix(&matrix_distance);
        let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
        resp.traveling_salesman_time = format!("{:?}", now1);
        resp.distance = tour.distance;
        resp.path = transform_route::get_transformed_route(req.points.as_slice(), tour.route);
    }

    actix_web::web::Json(resp)
}

#[post("/simulated_annealing/solve_by_distance_matrix")]
async fn post_solve_by_distance_matrix(req: web::Json<Vec<Vec<f64>>>) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let tour = travelling_salesman::simulated_annealing::solve_by_distance_matrix(
        &req,
        time::Duration::seconds(1),
    );
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    // resp.traveling_salesman_time = format!("{:?}", now1);
    // resp.distance = tour.distance;
    let mut resp = "".to_string();
    resp = resp + "time: " + &format!("{:?}", now1) + "\n";
    resp = resp + "tour.distance: " + &format!("{:?}", tour.distance) + "\n";
    resp = resp + "tour.route: " + &format!("{:?}", tour.route) + "\n";
    resp
}

#[post("/branch_and_bound/solve_by_distance_matrix")]
async fn post_solve_by_distance_matrix1(req: web::Json<Vec<Vec<f64>>>) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let (path, distance) = branch_and_bound::tsp_solver(&req);
    let now1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - now;
    // resp.traveling_salesman_time = format!("{:?}", now1);
    // resp.distance = tour.distance;
    let mut resp = "".to_string();
    resp = resp + "time: " + &format!("{:?}", now1) + "\n";
    resp = resp + "tour.distance: " + &format!("{:?}", distance) + "\n";
    resp = resp + "tour.route: " + &format!("{:?}", path) + "\n";
    resp
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
    println!("http://localhost:8080/ui");
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
            .service(post_calculate_optimal_path_old)
            .service(post_calc_distances_matrix)
            .service(post_calc_distances_matrix_text)
            .service(post_solve_by_distance_matrix)
            .service(post_solve_by_distance_matrix1)
            // .service(fs::Files::new("/ui", "./ui-admin").index_file("index.html"))
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
