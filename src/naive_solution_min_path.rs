/// Utility function to convert city coordinates to a distance matrix
///
/// `cities` is an array slice, containing `(x,y)` tuple coordinates for each city.
///
/// Returns a `Vec<Vec<f64>>`, containing the distance matrix.
///
///# Examples
///
///```
///extern crate travelling_salesman;
///
///fn main() {
///    let cities = [
///      (27.0, 78.0),
///      (18.0, 24.0),
///      (48.0, 62.0),
///      (83.0, 77.0),
///      (55.0, 56.0),
///    ];
///
///    let distance_matrix = travelling_salesman::get_distance_matrix(&cities);
///
///    println!("The distance between 1 and 2 is: {}", distance_matrix[1][2]);
///}
///```
pub fn solve(geometry: &Vec<Vec<char>>, points: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // copy geometry
    let mut m: Vec<Vec<char>> = Vec::with_capacity(geometry.len());
    for i in 0..geometry.len() {
        let mut row: Vec<char> = Vec::with_capacity(geometry[i].len());
        for j in 0..geometry[i].len() {
            row.push(geometry[i][j]);
        }
        m.push(row);
    }
    // end copy geometry

    let points_len = points.len();
    for i in 1..points_len {
        m[points[i].0][points[i].1] = 'P';
    }
    let mut resp: Vec<(usize, usize)> = Vec::with_capacity(points_len + 1);
    resp.push(points[0]);
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == 'P' {
                resp.push((i, j));
            }
            // print!("{}", m[i][j])
        }
        // println!("")
    }
    resp.push(points[0]);
    resp
}

//region *** tests function here ***
// #[cfg(test)]
// mod get_distances_matrix_tests {
//     use super::*;
//     fn get_distance_matrix<F>(cities: &[(f64, f64)], distance: F) -> Vec<Vec<f64>>
//     where
//         F: Fn(&(f64, f64), &(f64, f64)) -> f64,
//     {
//         cities
//             .iter()
//             .map(|row| {
//                 cities
//                     .iter()
//                     .map(|column| distance(row, column))
//                     .collect::<Vec<f64>>()
//             })
//             .collect::<Vec<Vec<f64>>>()
//     }
//     fn azimut_distanse(point1: &(f64, f64), point2: &(f64, f64)) -> f64 {
//         ((point1.0 - point2.0).powi(2) + (point1.1 - point2.1).powi(2)).sqrt()
//     }
//     #[test]
//     fn test_get_distances_matrix() {
//         let points = [
//             (27.0, 78.0),
//             (18.0, 24.0),
//             (48.0, 62.0),
//             (83.0, 77.0),
//             (55.0, 56.0),
//             (27.0, 78.0),
//             (18.0, 24.0),
//             (48.0, 62.0),
//             (83.0, 77.0),
//             (55.0, 56.0),
//             (70.0, 87.0),
//             (1.0, 1.0),
//             (10.0, 10.0),
//             (17.0, 27.0),
//             (0.0, 0.0),
//             (100.0, 120.0),
//             (-100.0, 120.0),
//         ];
//         let matrix1 = get_distance_matrix(&points, azimut_distanse);
//         let matrix2 = get_distances_matrix(&points, azimut_distanse);
//         // matrix_print(&matrix);

//         assert_eq!(matrix1, matrix2)
//     }
// }
// //endregion
