/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    assert!(mat1.len() > 0, "mat1.len(): {}", mat1.len());
    assert!(mat1[0].len() > 0, "mat1[0].len(): {}", mat1[0].len());
    assert_eq!(
        mat1[0].len(),
        mat2.len(),
        "mat1[0].len(): {}, mat2.len(): {}",
        mat1[0].len(),
        mat2.len()
    );
    assert!(mat2[0].len() > 0, "mat2[0].len(): {}", mat2[0].len());
    // fill the result matrix with 0s
    let mut result = vec![vec![0.; mat2[0].len()]; mat1.len()];

    for i in 0..result.len() {
        for j in 0..result[i].len() {
            for k in 0..mat1[1].len() {
                result[i][j] += mat1[i][k] * mat2[k][j]
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // provided tests
    #[test]
    fn test_mat_mult_identity() {
        let mut mat1 = vec![vec![0.; 3]; 3];
        for i in 0..mat1.len() {
            mat1[i][i] = 1.;
        }
        let mat2 = vec![vec![5.; 3]; 3];
        let result = mat_mult(&mat1, &mat2);
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                assert_eq!(result[i][j], mat2[i][j]);
            }
        }
    }

    // my own tests
    #[test]
    #[should_panic(expected = "mat1.len(): 0")]
    fn test_mat1_row_count_0() {
        let mat1 = vec![];
        let mat2 = vec![vec![1.; 3]; 5];
        mat_mult(&mat1, &mat2);
    }

    #[test]
    #[should_panic(expected = "mat1[0].len(): 0")]
    fn test_mat1_col_count_is_0() {
        let mat1 = vec![vec![]; 2];
        let mat2 = vec![vec![5.; 4]; 3];
        mat_mult(&mat1, &mat2);
    }

    #[test]
    #[should_panic(expected = "mat2[0].len(): 0")]
    fn test_mat2_col_count_is_0() {
        let mat1 = vec![vec![1.; 3]; 4];
        let mat2 = vec![vec![]; 3];
        mat_mult(&mat1, &mat2);
    }

    #[test]
    #[should_panic(expected = "mat1[0].len(): 3, mat2.len(): 6")]
    fn test_mat1_col_count_ne_mat2_row_count() {
        let mat1 = vec![vec![1.; 3]; 4];
        let mat2 = vec![vec![5.; 7]; 6];
        mat_mult(&mat1, &mat2);
    }
}