// Desc: Utility functions


pub fn round_float64(num: f64, precision: i32) -> f64 {
    let factor = 10.0_f64.powi(precision);
    (num * factor).round() / factor
}


pub fn round_matrix4x4(matrix: &mut nalgebra::Matrix4<f64>, precision: i32) {
    for i in 0..4 {
        for j in 0..4 {
            matrix[(i, j)] = round_float64(matrix[(i, j)], precision);
        }
    }
}
