use std::ops::{Add, Div, Mul, Rem, Sub};

fn is_barricade(item: char) -> bool {
    item == 'W'
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

fn can_move_to(geometry: &Vec<Vec<char>>, point: &(usize, usize), direction: Direction) -> bool {
    let (y, x) = *point;

    match &direction {
        Direction::Left => {
            let (edge_y, edge_x) = (y, (x as isize - 1) as usize);
            if edge_y >= geometry.len() || edge_x >= geometry[edge_y].len() {
                return false;
            }
            if is_barricade(geometry[edge_y][edge_x]) {
                return false;
            }
            return true;
        }
        Direction::Right => {
            let (edge_y, edge_x) = (y, (x as isize + 1) as usize);
            if edge_y >= geometry.len() || edge_x >= geometry[edge_y].len() {
                return false;
            }
            if is_barricade(geometry[edge_y][edge_x]) {
                return false;
            }
            return true;
        }

        Direction::Top => {
            let (edge_y, edge_x) = ((y as isize - 1) as usize, x);
            if edge_y >= geometry.len() || edge_x >= geometry[edge_y].len() {
                return false;
            }
            if is_barricade(geometry[edge_y][edge_x]) {
                return false;
            }
            return true;
        }

        Direction::Bottom => {
            let (edge_y, edge_x) = ((y as isize + 1) as usize, x);
            if edge_y >= geometry.len() || edge_x >= geometry[edge_y].len() {
                return false;
            }
            if is_barricade(geometry[edge_y][edge_x]) {
                return false;
            }
            return true;
        }
    }
}

// region *** tests function here ***
#[cfg(test)]
mod can_move_to_tests {
    use super::*;
    #[test]
    fn test_1() {
        let geometry: Vec<Vec<char>> = vec![
            //j   0    1    2    3    4    5    6    7       i
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 0
            vec!['W', 'P', 'P', '_', '_', 'P', '_', 'W'], // 1
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 2
            vec!['W', '_', '_', '_', '_', '_', 'P', 'W'], // 3
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 4
            vec!['W', 'P', '_', '_', '_', '_', '_', 'W'], // 5
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 6
            vec!['W', '_', '_', '_', '_', '_', 'P', 'W'], // 7
            vec!['W', 'S', '_', '_', '_', '_', '_', '_'], // 8
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 9
        ];
        let point: (usize, usize) = (1, 1);
        assert_eq!(can_move_to(&geometry, &point, Direction::Left), false);
        assert_eq!(can_move_to(&geometry, &point, Direction::Right), true);
        assert_eq!(can_move_to(&geometry, &point, Direction::Top), false);
        assert_eq!(can_move_to(&geometry, &point, Direction::Bottom), false);
        let point: (usize, usize) = (1, 2);
        assert_eq!(can_move_to(&geometry, &point, Direction::Left), true);
        assert_eq!(can_move_to(&geometry, &point, Direction::Right), true);
        assert_eq!(can_move_to(&geometry, &point, Direction::Top), false);
        assert_eq!(can_move_to(&geometry, &point, Direction::Bottom), false);
        let point: (usize, usize) = (3, 3);
        assert_eq!(can_move_to(&geometry, &point, Direction::Left), true);
        assert_eq!(can_move_to(&geometry, &point, Direction::Right), true);
        assert_eq!(can_move_to(&geometry, &point, Direction::Top), true);
        assert_eq!(can_move_to(&geometry, &point, Direction::Bottom), true);
    }
}
//endregion

fn is_dead_end_left(geometry: &Vec<Vec<char>>, point: &(usize, usize)) -> bool {
    let (i, mut j) = *point;
    loop {
        let left = can_move_to(&geometry, &(i, j), Direction::Left);
        let right = can_move_to(&geometry, &(i, j), Direction::Right);
        let top = can_move_to(&geometry, &(i, j), Direction::Top);
        let bottom = can_move_to(&geometry, &(i, j), Direction::Bottom);
        if !(left || top || bottom) {
            return true;
        }
        if top || bottom {
            break;
        }
        j -= 1;
    }
    false
}

#[cfg(test)]
mod is_dead_end_left_tests {
    use super::*;
    #[test]
    fn test_1() {
        let geometry: Vec<Vec<char>> = vec![
            //j   0    1    2    3    4    5    6    7       i
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 0
            vec!['W', 'P', 'P', '_', '_', 'P', '_', 'W'], // 1
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 2
            vec!['W', '_', '_', '_', '_', '_', 'P', 'W'], // 3
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 4
            vec!['W', 'P', '_', '_', '_', '_', '_', 'W'], // 5
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 6
            vec!['W', '_', '_', '_', '_', '_', 'P', 'W'], // 7
            vec!['W', 'S', '_', '_', '_', '_', '_', '_'], // 8
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 9
        ];
        assert_eq!(is_dead_end_left(&geometry, &(1, 1)), true);
        assert_eq!(is_dead_end_left(&geometry, &(1, 2)), true);
        assert_eq!(is_dead_end_left(&geometry, &(1, 5)), false);
        assert_eq!(is_dead_end_left(&geometry, &(3, 6)), false);
        assert_eq!(is_dead_end_left(&geometry, &(7, 6)), false);
    }
}
/// Utility function to convert city coordinates to a distance matrix
///
/// `cities` is an array slice, containing `(x,y)` tuple coordinates for each city.
///
/// Returns a `Vec<Vec<f64>>`, containing the distance matrix.
///```
pub fn solve(geometry: &Vec<Vec<char>>, points: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut m: Vec<Vec<char>> = geometry.clone();

    let points_len = points.len();
    for i in 1..points_len {
        m[points[i].0][points[i].1] = 'P';
    }
    let mut resp: Vec<(usize, usize)> = Vec::with_capacity(points_len + 1);
    resp.push(points[0]);
    let mut points1: Vec<(usize, usize)> = Vec::new();
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == 'P' {
                if is_dead_end_left(geometry, &(i, j)) {
                    if points1.len() > 0 {
                        points1.reverse();
                        resp.append(&mut points1);
                    }
                    resp.push((i, j));
                } else {
                    points1.push((i, j));
                }
            }
            // print!("{}", m[i][j])
        }
        if points1.len() > 0 {
            points1.reverse();
            resp.append(&mut points1);
        }
        // println!("")
    }
    resp.push(points[0]);
    resp
}

// region *** tests function here ***
#[cfg(test)]
mod solve_tests {
    use super::*;
    #[test]
    fn test_solve1() {
        let geometry: Vec<Vec<char>> = vec![
            //j   0    1    2    3    4    5    6    7       i
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 0
            vec!['W', 'P', '_', '_', '_', 'P', '_', 'W'], // 1
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 2
            vec!['W', '_', '_', '_', '_', '_', 'P', 'W'], // 3
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 4
            vec!['W', 'P', '_', '_', '_', '_', '_', 'W'], // 5
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 6
            vec!['W', '_', '_', '_', '_', '_', 'P', 'W'], // 7
            vec!['W', 'S', '_', '_', '_', '_', '_', '_'], // 8
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 9
        ];
        let points: Vec<(usize, usize)> = vec![(8, 1), (1, 1), (1, 5), (3, 6), (5, 1), (7, 6)];
        let path = solve(&geometry, &points);
        let path_should_be: Vec<(usize, usize)> =
            vec![(8, 1), (1, 1), (1, 5), (3, 6), (5, 1), (7, 6), (8, 1)];
        assert_eq!(path, path_should_be)
    }

    #[test]
    fn test_solve2() {
        let geometry: Vec<Vec<char>> = vec![
            //j   0    1    2    3    4    5    6    7       i
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 0
            vec!['W', 'P', 'P', '_', 'P', 'P', 'P', 'W'], // 1
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 2
            vec!['W', 'P', '_', '_', '_', 'P', 'P', 'W'], // 3
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 4
            vec!['W', '_', 'P', '_', 'P', '_', '_', 'W'], // 5
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W'], // 6
            vec!['W', 'S', '_', '_', 'P', '_', 'P', 'W'], // 7
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 8
            vec!['W', '_', '_', '_', '_', '_', '_', '_'], // 9
        ];
        let points: Vec<(usize, usize)> = vec![
            (7, 1),
            (1, 1),
            (1, 2),
            (1, 4),
            (1, 5),
            (1, 6),
            (3, 1),
            (3, 5),
            (3, 6),
            (5, 2),
            (5, 4),
            (7, 4),
            (7, 6),
        ];
        let path = solve(&geometry, &points);
        let path_should_be: Vec<(usize, usize)> = vec![
            (7, 1),
            //
            (1, 1),
            (1, 2),
            //
            (1, 6),
            (1, 5),
            (1, 4),
            //
            (3, 1),
            //
            (3, 6),
            (3, 5),
            //
            (5, 2),
            //
            (5, 4),
            //
            (7, 6),
            (7, 4),
            //
            (7, 1),
        ];
        assert_eq!(path, path_should_be)
    }

    #[test]
    fn test_solve3() {
        let geometry: Vec<Vec<char>> = vec![
            //j   0    1    2    3    4    5    6    7    8    9       i
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 0
            vec!['W', '_', '_', 'P', '_', '_', '_', '_', '_', 'W'], // 1
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W', 'W', 'W'], // 2
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W', 'W', 'W'], // 3
            vec!['W', 'P', '_', '_', '_', '_', '_', '_', '_', 'W'], // 4
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W', 'W', 'W'], // 5
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W', 'W', 'W'], // 6
            vec!['W', '_', 'P', '_', 'P', '_', '_', '_', '_', 'W'], // 7
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W', 'W', 'W'], // 8
            vec!['W', 'W', 'W', '_', 'W', 'W', 'W', 'W', 'W', 'W'], // 9
            vec!['W', '_', '_', '_', 'P', '_', '_', '_', '_', 'W'], // 10
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', '_', 'W'], // 11
            vec!['_', '_', '_', '_', '_', '_', '_', 'W', 'S', 'W'], // 12
            vec!['_', '_', '_', '_', '_', '_', '_', 'W', 'W', 'W'], // 13
        ];
        let points: Vec<(usize, usize)> = vec![(12, 8), (1, 3), (4, 1), (7, 2), (7, 4), (10, 4)];
        let path = solve(&geometry, &points);
        let path_should_be: Vec<(usize, usize)> =
            vec![(12, 8), (1, 3), (4, 1), (7, 2), (7, 4), (10, 4), (12, 8)];
        assert_eq!(path, path_should_be)
    }

    #[test]
    fn test_solve4() {
        let geometry: Vec<Vec<char>> = vec![
            //j   0    1    2    3    4    5    6    7    8    9       i
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 0
            vec!['W', '_', '_', 'P', '_', '_', 'P', 'P', '_', 'W'], // 1
            vec!['W', '_', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 2
            vec!['W', '_', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 3
            vec!['W', '_', '_', '_', '_', 'P', '_', 'P', '_', 'W'], // 4
            vec!['W', '_', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 5
            vec!['W', '_', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 6
            vec!['W', '_', 'P', '_', 'P', '_', '_', '_', 'P', 'W'], // 7
            vec!['W', '_', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 8
            vec!['W', '_', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'], // 9
            vec!['W', '_', '_', '_', 'P', '_', '_', 'P', '_', 'W'], // 10
            vec!['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', '_', 'W'], // 11
            vec!['_', '_', '_', '_', '_', '_', '_', 'W', 'S', 'W'], // 12
            vec!['_', '_', '_', '_', '_', '_', '_', 'W', 'W', 'W'], // 13
        ];
        let points: Vec<(usize, usize)> = vec![
            (12, 8),
            (1, 3),
            (1, 6),
            (1, 7),
            (4, 5),
            (4, 7),
            (7, 2),
            (7, 4),
            (7, 8),
            (10, 7),
            (10, 4),
        ];
        let path = solve(&geometry, &points);
        let path_should_be: Vec<(usize, usize)> = vec![
            (12, 8),
            //
            (1, 7),
            (1, 6),
            (1, 3),
            //
            (4, 7),
            (4, 5),
            //
            (7, 8),
            (7, 4),
            (7, 2),
            //
            (10, 7),
            (10, 4),
            //
            (12, 8),
        ];
        assert_eq!(path, path_should_be)
    }
}
//endregion
