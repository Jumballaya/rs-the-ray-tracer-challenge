use crate::{
    math::{epsilon::EPSILON, point::Point, ray::Ray, vector::Vector},
    render::{
        intersections::{Intersection, Intersections},
        object::Object,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    e1: Vector,
    e2: Vector,
    normal: Vector,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(&e1).normalize();
        Self {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
        }
    }

    pub fn normal_at(&self, _: &Point) -> Vector {
        self.normal
    }

    pub fn intersect<'a>(&self, ray: &Ray, obj: &'a Object, intersections: &mut Intersections<'a>) {
        let dir_cross_e2 = ray.direction.cross(&self.e2);
        let det = self.e1 * dir_cross_e2;
        if det.abs() < EPSILON {
            return;
        }

        // Calculate 'u' value
        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * (p1_to_origin * dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return;
        }

        // Calculate 'v' value
        let origin_cross_e1 = p1_to_origin.cross(&self.e1);
        let v = f * (ray.direction * origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return;
        }

        let t = f * (self.e2 * origin_cross_e1);
        intersections.push(Intersection::new(t, obj).with_u_v(u, v));
    }

    pub fn p1(&self) -> Point {
        self.p1
    }

    pub fn p2(&self) -> Point {
        self.p2
    }

    pub fn p3(&self) -> Point {
        self.p3
    }
}

#[cfg(test)]
mod test {

    use crate::{
        math::{epsilon::ApproxEq, point::Point, ray::Ray, tuple::Tuple, vector::Vector},
        render::{intersections::Intersections, object::Object},
    };

    use super::Triangle;

    fn test_triangle() -> (Triangle, (Point, Point, Point)) {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        (Triangle::new(p1, p2, p3), (p1, p2, p3))
    }

    #[test]
    fn constructing_a_triangle() {
        let (t, (p1, p2, p3)) = test_triangle();

        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.e1, Vector::new(-1.0, -1.0, 0.0));
        assert_eq!(t.e2, Vector::new(1.0, -1.0, 0.0));
        assert_eq!(t.normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn finding_normal_vector_on_a_triangle() {
        let (t, _) = test_triangle();

        let n1 = t.normal_at(&Point::new(0.0, 0.5, 0.0));
        let n2 = t.normal_at(&Point::new(-0.5, 0.75, 0.0));
        let n3 = t.normal_at(&Point::new(0.5, 0.25, 0.0));

        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn intersection_a_ray_parallel_to_the_triangle() {
        let obj = Object::new_test_shape();
        let (t, _) = test_triangle();
        let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 0.0));
        let mut ints = Intersections::new();
        t.intersect(&r, &obj, &mut ints);
        assert_eq!(ints.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge() {
        let obj = Object::new_test_shape();
        let (t, _) = test_triangle();
        let r = Ray::new(Point::new(1.0, 1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        let mut ints = Intersections::new();
        t.intersect(&r, &obj, &mut ints);
        assert_eq!(ints.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let obj = Object::new_test_shape();
        let (t, _) = test_triangle();
        let r = Ray::new(Point::new(-1.0, 1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        let mut ints = Intersections::new();
        t.intersect(&r, &obj, &mut ints);
        assert_eq!(ints.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge() {
        let obj = Object::new_test_shape();
        let (t, _) = test_triangle();
        let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        let mut ints = Intersections::new();
        t.intersect(&r, &obj, &mut ints);
        assert_eq!(ints.len(), 0);
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let obj = Object::new_test_shape();
        let (t, _) = test_triangle();
        let r = Ray::new(Point::new(0.0, 0.5, -2.0), Vector::new(0.0, 0.0, 1.0));
        let mut ints = Intersections::new();
        t.intersect(&r, &obj, &mut ints);
        assert_eq!(ints.len(), 1);
        assert!(ints[0].t().approx_eq(2.0));
    }
}
