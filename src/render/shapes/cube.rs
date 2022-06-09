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
pub struct Cube {}

impl Cube {
    pub fn new() -> Self {
        Self {}
    }

    pub fn normal_at(&self, point: &Point) -> Vector {
        let x = point.x().abs();
        let y = point.y().abs();
        let z = point.z().abs();
        let max_c = x.max(y).max(z);

        if max_c.approx_eq(x) {
            Vector::new(point.x(), 0.0, 0.0)
        } else if max_c.approx_eq(y) {
            Vector::new(0.0, point.y(), 0.0)
        } else {
            Vector::new(0.0, 0.0, point.z())
        }
    }

    pub fn intersect<'a>(&self, ray: &Ray, obj: &'a Object, intersections: &mut Intersections<'a>) {
        let (x_min, x_max) = self.check_axis(ray.origin.x(), ray.direction.x());
        let (y_min, y_max) = self.check_axis(ray.origin.y(), ray.direction.y());
        let (z_min, z_max) = self.check_axis(ray.origin.z(), ray.direction.z());

        let t_min = x_min.max(y_min).max(z_min);
        let t_max = x_max.min(y_max).min(z_max);

        if t_max < 0.0 {
            return;
        }

        if t_min <= t_max {
            intersections.push(Intersection::new(t_min, obj));
            intersections.push(Intersection::new(t_max, obj));
        }
    }

    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let (tmin, tmax) = if direction.abs() >= EPSILON {
            let tmin = tmin_numerator / direction;
            let tmax = tmax_numerator / direction;
            (tmin, tmax)
        } else {
            let tmin = tmin_numerator * INFINITY;
            let tmax = tmax_numerator * INFINITY;
            (tmin, tmax)
        };

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}

#[cfg(test)]
mod test {

    use super::Cube;
    use crate::{
        math::{epsilon::ApproxEq, point::Point, ray::Ray, tuple::Tuple, vector::Vector},
        render::{intersections::Intersections, object::Object},
    };

    #[test]
    fn ray_intersects_cube() {
        let obj = Object::new_test_shape();
        let c = Cube::new();

        let ray_plus_x = Ray::new(Point::new(5.0, 0.5, 0.0), Vector::new(-1.0, 0.0, 0.0));
        let mut ints_plus_x = Intersections::new();
        c.intersect(&ray_plus_x, &obj, &mut ints_plus_x);
        assert_eq!(ints_plus_x.len(), 2);
        assert!(ints_plus_x[0].t().approx_eq(4.0));
        assert!(ints_plus_x[1].t().approx_eq(6.0));

        let ray_minus_x = Ray::new(Point::new(-5.0, 0.5, 0.0), Vector::new(1.0, 0.0, 0.0));
        let mut ints_minus_x = Intersections::new();
        c.intersect(&ray_minus_x, &obj, &mut ints_minus_x);
        assert_eq!(ints_minus_x.len(), 2);
        assert!(ints_minus_x[0].t().approx_eq(4.0));
        assert!(ints_minus_x[1].t().approx_eq(6.0));

        let ray_plus_y = Ray::new(Point::new(0.5, 5.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let mut ints_plus_y = Intersections::new();
        c.intersect(&ray_plus_y, &obj, &mut ints_plus_y);
        assert_eq!(ints_plus_y.len(), 2);
        assert!(ints_plus_y[0].t().approx_eq(4.0));
        assert!(ints_plus_y[1].t().approx_eq(6.0));

        let ray_minus_y = Ray::new(Point::new(0.5, -5.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let mut ints_minus_y = Intersections::new();
        c.intersect(&ray_minus_y, &obj, &mut ints_minus_y);
        assert_eq!(ints_minus_y.len(), 2);
        assert!(ints_minus_y[0].t().approx_eq(4.0));
        assert!(ints_minus_y[1].t().approx_eq(6.0));

        let ray_plus_z = Ray::new(Point::new(0.5, 0.0, 5.0), Vector::new(0.0, 0.0, -1.0));
        let mut ints_plus_z = Intersections::new();
        c.intersect(&ray_plus_z, &obj, &mut ints_plus_z);
        assert_eq!(ints_plus_z.len(), 2);
        assert!(ints_plus_z[0].t().approx_eq(4.0));
        assert!(ints_plus_z[1].t().approx_eq(6.0));

        let ray_minus_z = Ray::new(Point::new(0.5, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut ints_minus_z = Intersections::new();
        c.intersect(&ray_minus_z, &obj, &mut ints_minus_z);
        assert_eq!(ints_minus_z.len(), 2);
        assert!(ints_minus_z[0].t().approx_eq(4.0));
        assert!(ints_minus_z[1].t().approx_eq(6.0));

        let ray_inside = Ray::new(Point::new(0.0, 0.5, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut ints_inside = Intersections::new();
        c.intersect(&ray_inside, &obj, &mut ints_inside);
        assert_eq!(ints_inside.len(), 2);
        assert!(ints_inside[0].t().approx_eq(-1.0));
        assert!(ints_inside[1].t().approx_eq(1.0));
    }

    #[test]
    fn ray_misses_a_cube() {
        let obj = Object::new_test_shape();
        let c = Cube::new();

        let ray1 = Ray::new(
            Point::new(-2.0, 0.0, 0.0),
            Vector::new(0.2673, 0.5345, 0.8018),
        );
        let mut ints1 = Intersections::new();
        c.intersect(&ray1, &obj, &mut ints1);
        assert_eq!(ints1.len(), 0);

        let ray2 = Ray::new(
            Point::new(0.0, -2.0, 0.0),
            Vector::new(0.8018, 0.2673, 0.5345),
        );
        let mut ints2 = Intersections::new();
        c.intersect(&ray2, &obj, &mut ints2);
        assert_eq!(ints2.len(), 0);

        let ray3 = Ray::new(
            Point::new(0.0, 0.0, -2.0),
            Vector::new(0.5345, 0.8018, 0.2673),
        );
        let mut ints3 = Intersections::new();
        c.intersect(&ray3, &obj, &mut ints3);
        assert_eq!(ints3.len(), 0);

        let ray4 = Ray::new(Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0));
        let mut ints4 = Intersections::new();
        c.intersect(&ray4, &obj, &mut ints4);
        assert_eq!(ints4.len(), 0);

        let ray5 = Ray::new(Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0));
        let mut ints5 = Intersections::new();
        c.intersect(&ray5, &obj, &mut ints5);
        assert_eq!(ints5.len(), 0);

        let ray6 = Ray::new(Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0));
        let mut ints6 = Intersections::new();
        c.intersect(&ray6, &obj, &mut ints6);
        assert_eq!(ints6.len(), 0);
    }

    #[test]
    fn normal_on_the_surface_of_a_cube() {
        let c = Cube::new();

        let got = c.normal_at(&Point::new(1.0, 0.5, -0.8));
        let want = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(got, want);

        let got = c.normal_at(&Point::new(-1.0, -0.2, 0.9));
        let want = Vector::new(-1.0, 0.0, 0.0);
        assert_eq!(got, want);

        let got = c.normal_at(&Point::new(-0.4, 1.0, -0.1));
        let want = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(got, want);

        let got = c.normal_at(&Point::new(0.3, -1.0, -0.7));
        let want = Vector::new(0.0, -1.0, 0.0);
        assert_eq!(got, want);

        let got = c.normal_at(&Point::new(-0.6, 0.3, 1.0));
        let want = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(got, want);

        let got = c.normal_at(&Point::new(0.4, 0.4, -1.0));
        let want = Vector::new(0.0, 0.0, -1.0);
        assert_eq!(got, want);

        let got = c.normal_at(&Point::new(1.0, 1.0, 1.0));
        let want = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(got, want);

        let got = c.normal_at(&Point::new(-1.0, -1.0, -1.0));
        let want = Vector::new(-1.0, 0.0, 0.0);
        assert_eq!(got, want);
    }
}
