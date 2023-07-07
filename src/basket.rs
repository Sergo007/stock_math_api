pub fn build_basket_weight_matrix(weights: &[f64]) -> Vec<Vec<f64>> {
    let num_points = weights.len();
    let mut weight_matrix = vec![vec![0.0; num_points]; num_points];

    for i in 0..num_points {
        let mut current_weight = weights[i];
        for j in 0..num_points {
            if j != i {
                weight_matrix[i][j] = current_weight;
                current_weight += weights[j];
            }
        }
    }
    weight_matrix
}

pub fn max_item(m: &Vec<Vec<f64>>) -> f64 {
    let num_points = m.len();
    let mut m_res = 0.0;

    for i in 0..num_points {
        for j in 0..num_points {
            if m[i][j] > m_res {
                m_res = m[i][j];
            }
        }
    }
    m_res
}

pub fn normalized_matrix(m: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let num_points = m.len();
    let mut weight_matrix = vec![vec![0.0; num_points]; num_points];
    let max_m = max_item(m);
    for i in 0..num_points {
        for j in 0..num_points {
            if j != i {
                weight_matrix[i][j] = weight_matrix[i][j] / max_m;
            }
        }
    }
    weight_matrix
}

pub fn sum_mx(m1: &Vec<Vec<f64>>, m2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let num_points = m1.len();
    let mut m_res = vec![vec![0.0; num_points]; num_points];

    for i in 0..num_points {
        for j in 0..num_points {
            m_res[i][j] = m1[i][j] + m2[i][j];
        }
    }
    m_res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_basket_weight_matrix() {
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let weight_matrix = build_basket_weight_matrix(&weights);
        println!("{:?}", weight_matrix);
        // Проверяем размер матрицы
        assert_eq!(weight_matrix.len(), weights.len());
        assert_eq!(weight_matrix[0].len(), weights.len());

        // Проверяем значения весов
        // Ожидаемый результат сравниваем с помощью вложенных циклов
        // for i in 0..weights.len() {
        //     for j in 0..weights.len() {
        //         if j != i {
        //             let expected_weight = weights[i] + weights[j];
        //             assert_eq!(weight_matrix[i][j], expected_weight);
        //         } else {
        //             assert_eq!(weight_matrix[i][j], 0.0);
        //         }
        //     }
        // }
    }
}

//endregion
