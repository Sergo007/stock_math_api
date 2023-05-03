pub fn get_transformed_route<T>(points: &[(T, T)], route: Vec<usize>) -> Vec<(T, T)>
where
    T: Clone,
{
    let points_len = points.len();
    let mut resp = Vec::with_capacity(points_len);
    let start_index = route.iter().position(|&r| r == 0).unwrap();
    if start_index != 0 && start_index != (points_len - 1) {
        for index in start_index..points_len {
            resp.push(points[route[index]].clone())
        }
        for index in 0..(start_index + 1) {
            resp.push(points[route[index]].clone())
        }
    } else if start_index == 0 {
        for index in 0..points_len {
            resp.push(points[route[index]].clone())
        }
        resp.push(points[route[start_index]].clone())
    } else if start_index == (points_len - 1) {
        resp.push(points[route[start_index]].clone());
        for index in 0..points_len {
            resp.push(points[route[index]].clone())
        }
    }
    resp
}

//region *** tests function here ***
#[cfg(test)]
mod get_transformed_route_tests {
    use super::*;
    #[test]
    fn test_1() {
        let points = [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)];
        let positions = vec![0, 1, 2, 3, 4, 5, 0];
        let v_should_be = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (0, 0)];
        let v = get_transformed_route(&points, positions);
        // matrix_print(&matrix);

        assert_eq!(v, v_should_be)
    }
    #[test]
    fn test_2() {
        let points = [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)];
        let positions = vec![1, 0, 2, 3, 4, 5, 1];
        let v_should_be = vec![(0, 0), (2, 2), (3, 3), (4, 4), (5, 5), (1, 1), (0, 0)];
        let v = get_transformed_route(&points, positions);
        // matrix_print(&matrix);

        assert_eq!(v, v_should_be)
    }

    #[test]
    fn test_3() {
        let points = [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)];
        let positions = vec![1, 2, 3, 4, 0, 5, 1];
        let v_should_be = vec![(0, 0), (5, 5), (1, 1), (2, 2), (3, 3), (4, 4), (0, 0)];
        let v = get_transformed_route(&points, positions);
        // matrix_print(&matrix);

        assert_eq!(v, v_should_be)
    }

    #[test]
    fn test_4() {
        let points = [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)];
        let positions = vec![0, 1, 2, 3, 4, 5, 0];
        let v_should_be = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (0, 0)];
        let v = get_transformed_route(&points, positions);
        // matrix_print(&matrix);

        assert_eq!(v, v_should_be)
    }
}
//endregion
