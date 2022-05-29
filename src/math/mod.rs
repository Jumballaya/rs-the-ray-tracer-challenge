pub mod tuple;

static EPSILON: f64 = 0.00001;

pub fn float_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
