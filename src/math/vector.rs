use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{epsilon::ApproxEq, tuple::Tuple};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    pub fn normalize(&self) -> Vector {
        *self / self.magnitude()
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        *self - (*normal * 2.0) * (*self * *normal)
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    fn default() -> Self {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
    fn w(&self) -> f64 {
        0.0
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

// DOT product
impl Mul for Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::math::tuple::Tuple;

    use super::{super::point::Point, Vector};

    #[test]
    fn vector_w_is_0() {
        let v = Vector::new(4.0, -4.0, 3.0);
        assert_eq!(v.w(), 0.0);
    }

    #[test]
    fn can_add_vectors() {
        let t1 = Vector::new(3.0, -2.0, 5.0);
        let t2 = Vector::new(-2.0, 3.0, 1.0);
        let want = Vector::new(1.0, 1.0, 6.0);
        let got = t1 + t2;
        assert_eq!(got, want);
    }

    #[test]
    fn can_add_vector_and_point() {
        let t1 = Vector::new(3.0, -2.0, 5.0);
        let t2 = Point::new(-2.0, 3.0, 1.0);
        let want = Point::new(1.0, 1.0, 6.0);
        let got = t1 + t2;
        assert_eq!(got, want);
    }

    #[test]
    fn subtracting_2_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        let want = Vector::new(-2.0, -4.0, -6.0);
        let got = v1 - v2;
        assert_eq!(got, want);
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);
        let want = Vector::new(-1.0, 2.0, -3.0);
        let got = zero - v;
        assert_eq!(got, want);
    }

    #[test]
    fn negating_a_vector() {
        let t = Vector::new(1.0, -2.0, 3.0);
        let got = -t;
        let want = Vector::new(-1.0, 2.0, -3.0);
        assert_eq!(got, want);
    }

    #[test]
    fn multiply_vector_by_sclar() {
        let t = Vector::new(1.0, -2.0, 3.0);
        let scalar = 3.5;
        let got = t * scalar;
        let want = Vector::new(3.5, -7.0, 10.5);
        assert_eq!(got, want);
    }

    #[test]
    fn divide_vector_by_scalar() {
        let t = Vector::new(1.0, -2.0, 3.0);
        let scalar = 2.0;
        let got = t / scalar;
        let want = Vector::new(0.5, -1.0, 1.5);
        assert_eq!(got, want);
    }

    #[test]
    fn compute_magnitude() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let v3 = Vector::new(0.0, 0.0, 1.0);

        let v4 = Vector::new(1.0, 2.0, 3.0);
        let v5 = Vector::new(-1.0, -2.0, -3.0);

        assert_eq!(v1.magnitude(), 1.0);
        assert_eq!(v2.magnitude(), 1.0);
        assert_eq!(v3.magnitude(), 1.0);

        assert_eq!(v4.magnitude(), (14.0 as f64).sqrt());
        assert_eq!(v5.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn normalizing_vectors() {
        let v1 = Vector::new(4.0, 0.0, 0.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);

        let got1 = v1.normalize();
        let got2 = v2.normalize();

        let root14 = (14.0 as f64).sqrt();
        let want1 = Vector::new(1.0, 0.0, 0.0);
        let want2 = Vector::new(1.0 / root14, 2.0 / root14, 3.0 / root14);

        assert_eq!(got1, want1);
        assert_eq!(got2, want2);
    }

    #[test]
    fn dot_product_with_mul_operator() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let v3 = Vector::new(1.0, 2.0, 3.0);
        let v4 = Vector::new(2.0, 3.0, 4.0);
        let got1 = v1 * v2;
        let got2 = v4 * v3;
        let want = 20.0;
        assert_eq!(got1, want);
        assert_eq!(got2, want);
    }

    #[test]
    fn cross_product_on_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        let want1 = Vector::new(-1.0, 2.0, -1.0);
        let want2 = Vector::new(1.0, -2.0, 1.0);

        let got1 = v1.cross(&v2);
        let got2 = v2.cross(&v1);

        assert_eq!(want1, got1);
        assert_eq!(want2, got2);
    }

    #[test]
    fn reflecting_vector_approaching_45_deg() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        let got = v.reflect(&n);
        let want = Vector::new(1.0, 1.0, 0.0);
        assert_eq!(got, want);
    }

    #[test]
    fn reflecting_vector_from_slanted_surface() {
        let root_2_2 = (2.0 as f64).sqrt() / 2.0;
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(root_2_2, root_2_2, 0.0);
        let got = v.reflect(&n);
        let want = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(got, want);
    }
}
