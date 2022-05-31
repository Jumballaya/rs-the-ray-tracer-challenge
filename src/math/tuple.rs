use super::float_equal;
use std::ops::{self, Index};

#[derive(Debug, Clone, Copy)]
pub enum TupleType {
    Vector,
    Point,
    None,
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
    pub tp: TupleType,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            x,
            y,
            z,
            w,
            tp: TupleType::None,
        }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            w: 1.0,
            tp: TupleType::Point,
        }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            w: 0.0,
            tp: TupleType::Vector,
        }
    }

    pub fn from(x: f64, y: f64, z: f64, w: f64) -> Self {
        if float_equal(w, 0.0) {
            Tuple::new_vector(x, y, z)
        } else if float_equal(w, 1.0) {
            Tuple::new_point(x, y, z)
        } else {
            Tuple::new(x, y, z, w)
        }
    }

    pub fn as_tuple(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.z, self.w)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        let x = self.x / mag;
        let y = self.y / mag;
        let z = self.z / mag;
        let w = self.w / mag;
        Tuple::from(x, y, z, w)
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        let x = (self.y * other.z) - (self.z * other.y);
        let y = (self.z * other.x) - (self.x * other.z);
        let z = (self.x * other.y) - (self.y * other.x);
        Tuple::new_vector(x, y, z)
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;
        Tuple::from(x, y, z, w)
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        let w = self.w - rhs.w;
        Tuple::from(x, y, z, w)
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        let w = -self.w;
        Tuple::from(x, y, z, w)
    }
}

impl ops::Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Tuple> for Tuple {
    type Output = f64;

    fn mul(self, rhs: Tuple) -> Self::Output {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        let w = self.w * rhs;
        Tuple::from(x, y, z, w)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_equal(self.x, other.x)
            && float_equal(self.y, other.y)
            && float_equal(self.z, other.z)
            && float_equal(self.w, other.w)
    }

    fn ne(&self, other: &Self) -> bool {
        !(float_equal(self.x, other.x)
            && float_equal(self.y, other.y)
            && float_equal(self.z, other.z)
            && float_equal(self.w, other.w))
    }
}

impl Index<usize> for Tuple {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => &0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_tuple_as_point() {
        let point = Tuple::new_point(4.0, -4.0, 3.0);
        assert!(point.as_tuple() == (4.0, -4.0, 3.0, 1.0));
        assert!(match point.tp {
            TupleType::Point => true,
            _ => false,
        });
    }

    #[test]
    fn tuple_tuple_as_vector() {
        let vector = Tuple::new_vector(4.0, -4.0, 3.0);
        assert!(vector.as_tuple() == (4.0, -4.0, 3.0, 0.0));
        assert!(match vector.tp {
            TupleType::Vector => true,
            _ => false,
        });
    }

    #[test]
    fn tuple_can_add_tuples() {
        let t1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let want = Tuple::new(1.0, 1.0, 6.0, 1.0);
        let got = t1 + t2;
        assert!(got == want);
    }

    #[test]
    fn tuple_subtracting_two_points_gives_vector() {
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let p2 = Tuple::new_point(5.0, 6.0, 7.0);
        let difference = p1 - p2;
        assert!(match difference.tp {
            TupleType::Vector => true,
            _ => false,
        });
    }

    #[test]
    fn tuple_subtracting_a_vector_from_a_point() {
        let p = Tuple::new_point(3.0, 2.0, 1.0);
        let v = Tuple::new_vector(5.0, 6.0, 7.0);
        let difference = p - v;
        assert!(match difference.tp {
            TupleType::Point => true,
            _ => false,
        });
    }

    #[test]
    fn tuple_subtracting_two_vectors() {
        let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let v2 = Tuple::new_vector(5.0, 6.0, 7.0);
        let want = Tuple::new_vector(-2.0, -4.0, -6.0);
        let got = v1 - v2;
        assert!(match got.tp {
            TupleType::Vector => true,
            _ => false,
        });
        assert!(got == want);
    }

    #[test]
    fn tuple_subtract_vector_from_zero_vector() {
        let zero = Tuple::new_vector(0.0, 0.0, 0.0);
        let v = Tuple::new_vector(1.0, -2.0, 3.0);
        let want = Tuple::new_vector(-1.0, 2.0, -3.0);
        let got = zero - v;
        assert!(got == want);
    }

    #[test]
    fn tuple_negating_tuple() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let neg_t = -t;
        let want = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        assert!(neg_t == want);
    }

    #[test]
    fn tuple_multiply_tuple_by_scalar() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar = 3.5;
        let product = t * scalar;
        let want = Tuple::new(3.5, -7.0, 10.5, -14.0);
        assert!(product == want);
    }

    #[test]
    fn tuple_multiply_tuple_by_fraction() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar = 0.5;
        let product = scalar * t;
        let want = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert!(product == want);
    }

    #[test]
    fn tuple_can_compute_magnitude() {
        let v1 = Tuple::new_vector(1.0, 0.0, 0.0);
        let v2 = Tuple::new_vector(0.0, 1.0, 0.0);
        let v3 = Tuple::new_vector(0.0, 0.0, 1.0);

        let v4 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v5 = Tuple::new_vector(-1.0, -2.0, -3.0);

        assert!(float_equal(v1.magnitude(), 1.0));
        assert!(float_equal(v2.magnitude(), 1.0));
        assert!(float_equal(v3.magnitude(), 1.0));

        assert!(float_equal(v4.magnitude(), (14.0 as f64).sqrt()));
        assert!(float_equal(v5.magnitude(), (14.0 as f64).sqrt()));
    }

    #[test]
    fn tuple_can_normalize_vectors() {
        let v1 = Tuple::new_vector(4.0, 0.0, 0.0);
        let v2 = Tuple::new_vector(1.0, 2.0, 3.0);

        let got1 = v1.normalize();
        let got2 = v2.normalize();

        let root14 = (14.0 as f64).sqrt();
        let want1 = Tuple::new_vector(1.0, 0.0, 0.0);
        let want2 = Tuple::new_vector(1.0 / root14, 2.0 / root14, 3.0 / root14);

        assert_eq!(got1, want1);
        assert_eq!(got2, want2);
    }

    #[test]
    fn tuple_can_compute_dot_product() {
        let v1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v2 = Tuple::new_vector(2.0, 3.0, 4.0);
        let v3 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v4 = Tuple::new_vector(2.0, 3.0, 4.0);
        let got1 = v1 * v2;
        let got2 = v4 * v3;
        let want = 20.0;
        assert_eq!(got1, want);
        assert_eq!(got2, want);
    }

    #[test]
    fn tuple_can_compute_cross_product() {
        let v1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v2 = Tuple::new_vector(2.0, 3.0, 4.0);

        let want1 = Tuple::new_vector(-1.0, 2.0, -1.0);
        let want2 = Tuple::new_vector(1.0, -2.0, 1.0);

        let got1 = v1.cross(&v2);
        let got2 = v2.cross(&v1);

        assert_eq!(want1, got1);
        assert_eq!(want2, got2);
    }
}
