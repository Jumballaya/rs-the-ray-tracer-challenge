use crate::{
    math::{epsilon::EPSILON, point::Point, ray::Ray, tuple::Tuple, vector::Vector},
    render::intersections::{Intersection, Intersections},
    render::object::Object,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {}

impl Plane {
    pub fn new() -> Self {
        Plane {}
    }

    pub fn normal_at(_: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    pub fn intersect<'a>(&self, ray: &Ray, obj: &'a Object, intersections: &mut Intersections<'a>) {
        if ray.direction.y().abs() >= EPSILON {
            let t = -ray.origin.y() / ray.direction.y();
            intersections.push(Intersection::new(t, obj));
        }
    }
}

#[cfg(test)]
mod test {
    use super::Plane;
    use crate::math::{point::Point, ray::Ray, tuple::Tuple, vector::Vector};
    use crate::render::{intersections::Intersections, object::Object};

    #[test]
    fn plane_normal_is_constant_everywhere() {
        let n1 = Plane::normal_at(&Point::new(0.0, 0.0, 0.0));
        let n2 = Plane::normal_at(&Point::new(10.0, 0.0, -10.0));
        let n3 = Plane::normal_at(&Point::new(-5.0, 0.0, 150.0));

        let want = &Vector::new(0.0, 1.0, 0.0);
        assert_eq!(want, &n1);
        assert_eq!(want, &n2);
        assert_eq!(want, &n3);
    }

    #[test]
    fn plane_intersect_with_a_ray_parallel_to_plane() {
        let p = Object::new_plane();
        let ray = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut xs = Intersections::new();
        p.intersect(&ray, &mut xs);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn plane_intersect_with_a_coplanar_ray() {
        let p = Object::new_plane();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut xs = Intersections::new();
        p.intersect(&ray, &mut xs);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn plane_ray_intersecting_from_above() {
        let p = Object::new_plane();
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let mut xs = Intersections::new();
        p.intersect(&ray, &mut xs);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].object(), &p);
    }

    #[test]
    fn plane_ray_intersecting_from_below() {
        let p = Object::new_plane();
        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let mut xs = Intersections::new();
        p.intersect(&ray, &mut xs);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].object(), &p);
    }
}
