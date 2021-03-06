pub const EPSILON: f64 = 1.0e-4;

pub fn round(a: f64) -> f64 {
    (a * 10000.0).round() / 10000.0
}

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(self, other: Rhs) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: Self) -> bool {
        let dif = (round(self) - round(other)).abs();
        let rounded = (dif * 10000.0).round() / 10000.0;
        rounded < EPSILON
    }
}
