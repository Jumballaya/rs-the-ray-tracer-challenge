pub mod matrix;
pub mod ray;
pub mod tuple;

pub static EPSILON: f64 = 0.00001;

pub fn round(a: f64) -> f64 {
    (a * 100000.0).round() / 100000.0
}

pub fn float_equal(a: f64, b: f64) -> bool {
    let dif = (round(a) - round(b)).abs();
    let rounded = (dif * 100000.0).round() / 100000.0;
    rounded < EPSILON
}
