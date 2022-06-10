use std::f64::INFINITY;

use crate::{
    math::{
        epsilon::{ApproxEq, EPSILON},
        point::Point,
        ray::Ray,
        tuple::Tuple,
        vector::Vector,
    },
    render::{
        intersections::{Intersection, Intersections},
        object::Object,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cylinder {
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Cylinder {
    pub fn new() -> Self {
        Self {
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }

    pub fn with_min(self, min: f64) -> Self {
        Self {
            minimum: min,
            ..self
        }
    }

    pub fn with_max(self, max: f64) -> Self {
        Self {
            maximum: max,
            ..self
        }
    }

    pub fn with_closed(self, closed: bool) -> Self {
        Self { closed, ..self }
    }

    pub fn intersect<'a>(&self, ray: &Ray, obj: &'a Object, intersections: &mut Intersections<'a>) {
        let a = ray.direction.x().powi(2) + ray.direction.z().powi(2);
        if a.approx_eq(0.0) {
            self.intersect_caps(ray, obj, intersections);
        } else {
            let b = 2.0 * (ray.origin.x() * ray.direction.x() + ray.origin.z() * ray.direction.z());
            let c = ray.origin.x().powi(2) + ray.origin.z().powi(2) - 1.0;

            let disc = b.powi(2) - 4.0 * a * c;

            if disc < 0.0 {
                return;
            }

            let t0 = (-b - disc.sqrt()) / (2.0 * a);
            let t1 = (-b + disc.sqrt()) / (2.0 * a);

            let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

            let y0 = ray.origin.y() + t0 * ray.direction.y();
            if self.min() < y0 && y0 < self.max() {
                intersections.push(Intersection::new(t0, &obj));
            }

            let y1 = ray.origin.y() + t1 * ray.direction.y();
            if self.min() < y1 && y1 < self.max() {
                intersections.push(Intersection::new(t1, &obj));
            }

            self.intersect_caps(ray, obj, intersections);
        }
    }

    fn check_cap(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        (x.powi(2) + z.powi(2)) <= 1.0
    }

    pub fn intersect_caps<'a>(
        &self,
        ray: &Ray,
        obj: &'a Object,
        intersections: &mut Intersections<'a>,
    ) {
        if !self.closed || ray.direction.y().approx_eq(0.0) {
            return;
        }

        let t = (self.min() - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(ray, t) {
            intersections.push(Intersection::new(t, &obj));
        }

        let t = (self.max() - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(ray, t) {
            intersections.push(Intersection::new(t, &obj));
        }
    }

    pub fn normal_at(&self, point: &Point) -> Vector {
        let dist = point.x().powi(2) + point.z().powi(2);

        if dist < 1.0 && point.y() >= (self.max() - EPSILON) {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y() <= (self.min() + EPSILON) {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            Vector::new(point.x(), 0.0, point.z())
        }
    }

    pub fn min(&self) -> f64 {
        self.minimum
    }

    pub fn max(&self) -> f64 {
        self.maximum
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn open(&self) -> bool {
        !self.closed
    }
}

#[cfg(test)]
mod test {

    use std::f64::INFINITY;

    use crate::{
        math::{epsilon::ApproxEq, point::Point, ray::Ray, tuple::Tuple, vector::Vector},
        render::{intersections::Intersections, object::Object},
    };

    use super::Cylinder;

    fn test_runner<T>(test: fn(T), tests: Vec<T>) {
        for t in tests.into_iter() {
            test(t);
        }
    }

    #[test]
    fn ray_misses_a_cylinder() {
        fn test((origin, direction, len): (Point, Vector, usize)) {
            let obj = Object::new_test_shape();
            let c = Cylinder::new();

            let r = Ray::new(origin, direction.normalize());
            let mut ints = Intersections::new();
            c.intersect(&r, &obj, &mut ints);
            assert_eq!(ints.len(), len);
        }

        let tests = vec![
            (Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0), 0),
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0), 0),
            (Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0), 0),
        ];

        test_runner(test, tests);
    }

    #[test]
    fn ray_hits_cylinder() {
        fn test((origin, direction, t0, t1): (Point, Vector, f64, f64)) {
            let obj = Object::new_test_shape();
            let c = Cylinder::new();
            let r = Ray::new(origin, direction.normalize());
            let mut ints = Intersections::new();
            c.intersect(&r, &obj, &mut ints);
            assert_eq!(ints.len(), 2);
            assert!(ints[0].t().approx_eq(t0));
            assert!(ints[1].t().approx_eq(t1));
        }

        let tests = vec![
            (
                Point::new(1.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];

        test_runner(test, tests);
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        fn test((p, want): (Point, Vector)) {
            let c = Cylinder::new();
            let got = c.normal_at(&p);
            assert_eq!(got, want);
        }

        let tests = vec![
            (Point::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(0.0, 5.0, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(0.0, -2.0, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(-1.0, 1.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];

        test_runner(test, tests);
    }

    #[test]
    fn default_min_and_max_for_cylinder() {
        let c = Cylinder::new();
        assert_eq!(c.min(), -INFINITY);
        assert_eq!(c.max(), INFINITY);
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        fn test((origin, direction, len): (Point, Vector, usize)) {
            let obj = Object::new_test_shape();
            let c = Cylinder::new().with_min(1.0).with_max(2.0);

            let r = Ray::new(origin, direction.normalize());
            let mut ints = Intersections::new();
            c.intersect(&r, &obj, &mut ints);
            assert_eq!(ints.len(), len);
        }

        let tests = vec![
            (Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
            (Point::new(0.0, 3.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
        ];

        test_runner(test, tests);
    }

    #[test]
    fn default_closed_value_for_cylinder() {
        let c = Cylinder::new();
        assert!(!c.closed());
        assert!(c.open());
    }

    #[test]
    fn intersecting_caps_of_closed_cylinder() {
        fn test((origin, direction): (Point, Vector)) {
            let obj = Object::new_test_shape();
            let c = Cylinder::new()
                .with_min(1.0)
                .with_max(2.0)
                .with_closed(true);

            let r = Ray::new(origin, direction.normalize());
            let mut ints = Intersections::new();
            c.intersect(&r, &obj, &mut ints);
            assert_eq!(ints.len(), 2);
        }

        let tests = vec![
            (Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0)),
            (Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0)),
            (Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0)),
            (Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0)),
        ];

        test_runner(test, tests);
    }

    #[test]
    fn normal_vector_on_a_cylinders_end_caps() {
        let c = Cylinder::new()
            .with_min(1.0)
            .with_max(2.0)
            .with_closed(true);

        let p = Point::new(0.0, 1.0, 0.0);
        let want = Vector::new(0.0, -1.0, 0.0);
        let got = c.normal_at(&p);
        assert_eq!(got, want);

        let p = Point::new(0.5, 1.0, 0.0);
        let want = Vector::new(0.0, -1.0, 0.0);
        let got = c.normal_at(&p);
        assert_eq!(got, want);

        let p = Point::new(0.0, 1.0, 0.5);
        let want = Vector::new(0.0, -1.0, 0.0);
        let got = c.normal_at(&p);
        assert_eq!(got, want);

        let p = Point::new(0.0, 2.0, 0.0);
        let want = Vector::new(0.0, 1.0, 0.0);
        let got = c.normal_at(&p);
        assert_eq!(got, want);

        let p = Point::new(0.5, 2.0, 0.0);
        let want = Vector::new(0.0, 1.0, 0.0);
        let got = c.normal_at(&p);
        assert_eq!(got, want);

        let p = Point::new(0.0, 2.0, 0.5);
        let want = Vector::new(0.0, 1.0, 0.0);
        let got = c.normal_at(&p);
        assert_eq!(got, want);
    }
}
