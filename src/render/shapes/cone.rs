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
pub struct Cone {
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Cone {
    pub fn new() -> Self {
        Self {
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }

    pub fn intersect<'a>(&self, ray: &Ray, obj: &'a Object, intersections: &mut Intersections<'a>) {
        let a = ray.direction.x().powi(2) - ray.direction.y().powi(2) + ray.direction.z().powi(2);
        let b = 2.0
            * (ray.origin.x() * ray.direction.x() - ray.origin.y() * ray.direction.y()
                + ray.origin.z() * ray.direction.z());
        let c = ray.origin.x().powi(2) - ray.origin.y().powi(2) + ray.origin.z().powi(2);
        if a.approx_eq(0.0) && !b.approx_eq(0.0) {
            let t = c / (-2.0 * b);
            intersections.push(Intersection::new(t, &obj));
        } else {
            let disc = b.powi(2) - 4.0 * a * c;

            if disc < 0.0 {
                return;
            }

            let double_a = 2.0 * a;
            let t0 = (-b - disc.sqrt()) / double_a;
            let t1 = (-b + disc.sqrt()) / double_a;

            let y0 = ray.origin.y() + t0 * ray.direction.y();
            if self.minimum < y0 && y0 < self.maximum {
                intersections.push(Intersection::new(t0, &obj));
            }

            let y1 = ray.origin.y() + t1 * ray.direction.y();
            if self.minimum < y1 && y1 < self.maximum {
                intersections.push(Intersection::new(t1, &obj));
            }
        }
        self.intersect_caps(ray, obj, intersections);
    }

    pub fn normal_at(&self, point: &Point) -> Vector {
        let dist = point.x().powi(2) + point.z().powi(2);

        if dist < 1.0 && point.y() >= (self.maximum - EPSILON) {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y() <= (self.minimum + EPSILON) {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            Vector::new(
                point.x(),
                if point.y() > 0.0 {
                    -dist.sqrt()
                } else {
                    dist.sqrt()
                },
                point.z(),
            )
        }
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

        let t = (self.minimum - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(ray, t, self.minimum) {
            intersections.push(Intersection::new(t, &obj));
        }

        let t = (self.maximum - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(ray, t, self.maximum) {
            intersections.push(Intersection::new(t, &obj));
        }
    }

    fn check_cap(ray: &Ray, t: f64, radius: f64) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        (x.powi(2) + z.powi(2)) <= radius.powi(2)
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
}

#[cfg(test)]
mod test {
    use super::Cone;
    use crate::{
        math::{epsilon::ApproxEq, point::Point, ray::Ray, tuple::Tuple, vector::Vector},
        render::{intersections::Intersections, object::Object},
    };

    fn test_runner<T>(test: fn(T), tests: Vec<T>) {
        for t in tests.into_iter() {
            test(t);
        }
    }

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        fn test((origin, direction, len, t0, t1): (Point, Vector, usize, f64, f64)) {
            let c = Cone::new();
            let obj = Object::new_test_shape();

            let r = Ray::new(origin, direction.normalize());
            let mut ints = Intersections::new();
            c.intersect(&r, &obj, &mut ints);
            assert_eq!(ints.len(), len);
            assert!(ints[0].t().approx_eq(t0));
            assert!(ints[1].t().approx_eq(t1));
        }

        let tests = vec![
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                2,
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(1.0, 1.0, 1.0),
                2,
                8.66025,
                8.66025,
            ),
            (
                Point::new(1.0, 1.0, -5.0),
                Vector::new(-0.5, -1.0, 1.0),
                2,
                4.55006,
                49.44994,
            ),
        ];

        test_runner(test, tests);
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let c = Cone::new();
        let obj = Object::new_test_shape();

        let direction = Vector::new(0.0, 1.0, 1.0).normalize();
        let origin = Point::new(0.0, 0.0, -1.0);
        let r = Ray::new(origin, direction);
        let mut ints = Intersections::new();
        c.intersect(&r, &obj, &mut ints);
        assert_eq!(ints.len(), 1);
        assert!(ints[0].t().approx_eq(0.3535533));
    }

    #[test]
    fn intersecting_a_cones_end_caps() {
        fn test((origin, direction, len): (Point, Vector, usize)) {
            let c = Cone::new().with_min(-0.5).with_max(0.5).with_closed(true);
            let obj = Object::new_test_shape();
            let r = Ray::new(origin, direction.normalize());
            let mut ints = Intersections::new();
            c.intersect(&r, &obj, &mut ints);
            assert_eq!(ints.len(), len);
        }

        let tests = vec![
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0), 0),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0), 4),
        ];

        test_runner(test, tests);
    }

    #[test]
    fn normal_vector_on_a_cone() {
        fn test((p, want): (Point, Vector)) {
            let c = Cone::new();
            let got = c.normal_at(&p);
            assert_eq!(got, want);
        }

        let tests = vec![
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
            (
                Point::new(1.0, 1.0, 1.0),
                Vector::new(1.0, -f64::sqrt(2.0), 1.0),
            ),
            (Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0)),
        ];

        test_runner(test, tests);
    }
}
