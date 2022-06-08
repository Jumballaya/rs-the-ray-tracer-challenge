use std::ops::{Add, Mul, Sub};

use super::epsilon::ApproxEq;
use super::tuple::Tuple;
use super::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    fn default() -> Self {
        Point {
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
        1.0
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x() + rhs.x,
            y: self.y() + rhs.y,
            z: self.z() + rhs.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[cfg(test)]
mod test {

    use super::{Point, Tuple, Vector};

    #[test]
    fn point_w_is_1() {
        let v = Point::new(4.0, -4.0, 3.0);
        assert_eq!(v.w(), 1.0);
    }

    #[test]
    fn can_add_points() {
        let t1 = Point::new(3.0, -2.0, 5.0);
        let t2 = Point::new(-2.0, 3.0, 1.0);
        let want = Point::new(1.0, 1.0, 6.0);
        let got = t1 + t2;
        assert_eq!(got, want);
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        let difference = p - v;
        assert_eq!(difference.w(), 1.0);
    }

    #[test]
    fn subtracting_two_points_gives_vector() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        let difference = p1 - p2;
        assert_eq!(difference.w(), 0.0);
    }
}
