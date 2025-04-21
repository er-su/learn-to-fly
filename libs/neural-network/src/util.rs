pub fn matrix_vector_mult(matrix: &[f32], vector: &[f32], num_rows: usize, num_cols: usize) -> Vec<f32> {
    assert_eq!(matrix.len(), num_rows * num_cols);
    assert_eq!(vector.len(), num_cols);

    let mut new_vec = Vec::new();
    for i in 0..num_rows {
        let mut sum: f32 = 0.0;
        for j in 0..num_cols {
            sum += matrix[i * num_cols + j] * vector[j];
        }
        new_vec.push(sum);
    }
    new_vec
}

pub fn vector_vector_add(vector1: &[f32], vector2: &[f32]) -> Vec<f32> {
    assert_eq!(vector1.len(), vector2.len());
    vector1.into_iter()
        .zip(vector2)
        .map(|(val1, val2)| val1 + val2)
        .collect()
}

#[allow(unused)]
mod test {
    use super::*;

    #[test]
    fn testing_matrix_vector_mult() {
        // Assume 2x3 matrix 
        let matrix: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let vec: Vec<f32> = vec![1.0, 1.0];
        let num_rows = 2;
        let num_cols = 3;
        matrix_vector_mult(&matrix, &vec, num_rows, num_cols);
    }
}