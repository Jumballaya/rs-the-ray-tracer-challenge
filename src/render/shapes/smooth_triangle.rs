use crate::{
    math::{point::Point, ray::Ray, vector::Vector},
    render::{
        intersections::{Intersection, Intersections},
        object::Object,
        shapes::triangle::Triangle,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct SmoothTriangle {
    triangle: Triangle,
    n1: Vector,
    n2: Vector,
    n3: Vector,
}

impl SmoothTriangle {
    pub fn new(p1: Point, p2: Point, p3: Point, n1: Vector, n2: Vector, n3: Vector) -> Self {
        Self {
            triangle: Triangle::new(p1, p2, p3),
            n1,
            n2,
            n3,
        }
    }

    pub fn intersect<'a>(&self, ray: &Ray, obj: &'a Object, intersections: &mut Intersections<'a>) {
        self.triangle.intersect(ray, obj, intersections)
    }

    pub fn normal_at(&self, _: &Point, int: &Intersection) -> Vector {
        self.n2 * int.u() + self.n3 * int.v() + self.n1 * (1.0 - int.u() - int.v())
    }

    pub fn p1(&self) -> Point {
        self.triangle.p1()
    }

    pub fn p2(&self) -> Point {
        self.triangle.p2()
    }

    pub fn p3(&self) -> Point {
        self.triangle.p3()
    }

    pub fn n1(&self) -> Vector {
        self.n1
    }

    pub fn n2(&self) -> Vector {
        self.n2
    }

    pub fn n3(&self) -> Vector {
        self.n3
    }
}

#[cfg(test)]
mod test {
    use super::SmoothTriangle;
    use crate::{
        math::{epsilon::ApproxEq, point::Point, ray::Ray, tuple::Tuple, vector::Vector},
        render::{
            intersections::{HitComputation, Intersection, Intersections},
            object::Object,
        },
    };

    fn test_tri() -> SmoothTriangle {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let n1 = Vector::new(0.0, 1.0, 0.0);
        let n2 = Vector::new(-1.0, 0.0, 0.0);
        let n3 = Vector::new(1.0, 0.0, 0.0);
        SmoothTriangle::new(p1, p2, p3, n1, n2, n3)
    }

    fn test_tri_obj() -> Object {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let n1 = Vector::new(0.0, 1.0, 0.0);
        let n2 = Vector::new(-1.0, 0.0, 0.0);
        let n3 = Vector::new(1.0, 0.0, 0.0);
        Object::new_smooth_tri(p1, p2, p3, n1, n2, n3)
    }

    #[test]
    fn constructing_a_smooth_triangle() {
        let tri = test_tri();
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let n1 = Vector::new(0.0, 1.0, 0.0);
        let n2 = Vector::new(-1.0, 0.0, 0.0);
        let n3 = Vector::new(1.0, 0.0, 0.0);

        assert_eq!(tri.p1(), p1);
        assert_eq!(tri.p2(), p2);
        assert_eq!(tri.p3(), p3);

        assert_eq!(tri.n1(), n1);
        assert_eq!(tri.n2(), n2);
        assert_eq!(tri.n3(), n3);
    }

    #[test]
    fn an_intersection_with_a_smooth_triangle_stores_u_v() {
        let tri = test_tri();
        let obj = Object::new_test_shape();
        let r = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::new(0.0, 0.0, 1.0));
        let mut ints = Intersections::new();
        tri.intersect(&r, &obj, &mut ints);
        assert_eq!(ints.len(), 1);
        assert!(ints[0].u().approx_eq(0.45));
        assert!(ints[0].v().approx_eq(0.25));
    }

    #[test]
    fn smooth_triangle_uses_u_v_to_interpolate_normal() {
        let obj = test_tri_obj();
        let int = Intersection::new(1.0, &obj).with_u_v(0.45, 0.25);
        let got = obj.normal_at(&Point::new(0.0, 0.0, 0.0), &int);
        let want = Vector::new(-0.5547, 0.83205, 0.0);
        assert_eq!(got, want);
    }

    #[test]
    fn prepare_normal_on_a_smooth_triangle() {
        let obj = test_tri_obj();
        let int = Intersection::new(1.0, &obj).with_u_v(0.45, 0.25);
        let r = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::new(0.0, 0.0, 1.0));
        let ints = Intersections::new().with_intersections(vec![int]);
        let comp = HitComputation::new(&ints, 0, &r);
        let want = Vector::new(-0.5547, 0.83205, 0.0);
        assert_eq!(comp.normal, want);
    }
}
