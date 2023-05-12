fn first_min(adj: &Vec<f64>, i: usize) -> f64 {
    let mut min = f64::MAX;
    for k in 0..adj.len() {
        if adj[k] < min && i != k {
            min = adj[k]
        }
    }
    return min;
}

#[cfg(test)]
mod first_min_tests {
    use super::*;
    #[test]
    fn test_1() {
        let points = vec![10.0, 0.0, 35.0, 25.0];
        assert_eq!(first_min(&points, 1), 10.0)
    }
}

fn second_min(adj: &Vec<f64>, i: usize) -> f64 {
    let mut first = f64::MAX;
    let mut second = f64::MAX;
    for j in 0..adj.len() {
        if i == j {
            continue;
        }
        if adj[j] <= first {
            second = first;
            first = adj[j];
        } else if adj[j] <= second && adj[j] != first {
            second = adj[j];
        }
    }
    return second;
}

#[cfg(test)]
mod second_min_tests {
    use super::*;
    #[test]
    fn test_1() {
        let points = vec![10.0, 0.0, 35.0, 25.0];
        assert_eq!(second_min(&points, 1), 25.0)
    }
}

pub fn tsp_solver(adj: &Vec<Vec<f64>>) -> (Vec<i32>, f64) {
    let adj_len = adj.len();
    let mut curr_path = vec![-1; adj_len];
    let mut curr_bound = 0.0;
    let mut visited = vec![false; adj_len];
    for i in 0..adj_len {
        curr_bound += first_min(&adj[i], i) + second_min(&adj[i], i)
    }
    if curr_bound == 1.0 {
        curr_bound = curr_bound * 0.5 + 1.0;
    } else {
        curr_bound = curr_bound * 0.5;
    }
    visited[0] = true;
    curr_path[0] = 0;
    let mut final_res = f64::MAX;
    let mut final_path = vec![-1; adj_len];
    tsp_rec(
        adj,
        &mut curr_bound,
        &mut 0.0,
        1,
        &mut curr_path,
        &mut final_path,
        &mut final_res,
        &mut visited,
    );
    (final_path, final_res)
}

fn tsp_rec(
    adj: &Vec<Vec<f64>>,
    curr_bound: &mut f64,
    curr_weight: &mut f64,
    level: usize,
    curr_path: &mut Vec<i32>,
    final_path: &mut Vec<i32>,
    final_res: &mut f64,
    visited: &mut Vec<bool>,
) {
    let n = adj.len();
    // console.log('TSPRec  level: ', level)
    // base case is when we have reached level N which
    // means we have covered all the nodes once
    if level == n {
        // check if there is an edge from last vertex in
        // path back to the first vertex
        if adj[curr_path[level - 1] as usize][curr_path[0] as usize] != 0.0 {
            // curr_res has the total weight of the
            // solution we got
            let curr_res = *curr_weight + adj[curr_path[level - 1] as usize][curr_path[0] as usize];

            // Update final result and final path if
            // current result is better.
            if curr_res < *final_res {
                for i in 0..curr_path.len() {
                    final_path[i] = curr_path[i];
                }
                *final_res = curr_res;
            }
        }
        return;
    }

    // for any other level iterate for all vertices to
    // build the search space tree recursively
    for i in 0..n {
        // Consider next vertex if it is not same (diagonal
        // entry in adjacency matrix and not visited
        // already)
        if adj[curr_path[level - 1] as usize][i] != 0.0 && !visited[i] {
            let temp = *curr_bound;
            *curr_weight += adj[curr_path[level - 1] as usize][i];

            // different computation of curr_bound for
            // level 2 from the other levels
            if level == 1 {
                *curr_bound -= (first_min(
                    &adj[curr_path[level - 1] as usize],
                    curr_path[level - 1] as usize,
                ) + first_min(&adj[i], i))
                    * 0.5
            } else {
                *curr_bound -= (second_min(
                    &adj[curr_path[level - 1] as usize],
                    curr_path[level - 1] as usize,
                ) + second_min(&adj[i], i))
                    * 0.5
            }

            // curr_bound + curr_weight is the actual lower bound
            // for the node that we have arrived on
            // If current lower bound < final_res, we need to explore
            // the node further
            if *curr_bound + *curr_weight < *final_res {
                curr_path[level] = i as i32;
                visited[i] = true;
                // call TSPRec for the next level
                // TSPRec(adj, curr_bound, curr_weight, level + 1, curr_path);
                tsp_rec(
                    adj,
                    curr_bound,
                    curr_weight,
                    level + 1,
                    curr_path,
                    final_path,
                    final_res,
                    visited,
                );
            }

            // Else we have to prune the node by resetting
            // all changes to curr_weight and curr_bound
            *curr_weight -= adj[curr_path[level - 1] as usize][i];
            *curr_bound = temp;

            // Also reset the visited array
            visited.fill(false);
            for j in 0..level {
                visited[curr_path[j] as usize] = true;
            }
        }
    }
}

#[cfg(test)]
mod tsp_solver_tests {
    use super::*;
    #[test]
    fn test_1() {
        let graph = vec![
            vec![0.0, 10.0, 15.0, 20.0],
            vec![10.0, 0.0, 35.0, 25.0],
            vec![15.0, 35.0, 0.0, 30.0],
            vec![20.0, 25.0, 30.0, 0.0],
        ];
        assert_eq!(tsp_solver(&graph), (vec![0, 1, 3, 2], 80.0))
    }
}
