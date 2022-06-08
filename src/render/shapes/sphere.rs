use crate::{
    math::{point::Point, ray::Ray, tuple::Tuple, vector::Vector},
    render::{
        intersections::{Intersection, Intersections},
        object::Object,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Sphere {}
    }
    pub fn normal_at(&self, local_point: &Point) -> Vector {
        *local_point - Point::new(0.0, 0.0, 0.0)
    }

    pub fn intersect<'a>(&self, ray: &Ray, obj: &'a Object, intersections: &mut Intersections<'a>) {
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = ray.direction * ray.direction;
        let b = 2.0 * (ray.direction * sphere_to_ray);
        let c = (sphere_to_ray * sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if !(discriminant < 0.0) {
            let hit1 = (-b - (discriminant.sqrt())) / (2.0 * a);
            let hit2 = (-b + (discriminant.sqrt())) / (2.0 * a);
            intersections.push(Intersection::new(hit1, &obj));
            intersections.push(Intersection::new(hit2, &obj));
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::math::epsilon::ApproxEq;
    use crate::math::transformation::Transformable;
    use crate::math::tuple::Tuple;
    use crate::render::object::Object;

    #[test]
    fn ray_intersects_at_2_points() {
        let obj = Object::new_sphere();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0))
            .with_transform(obj.get_transform().inverse());
        let mut xs = Intersections::new();
        obj.intersect(&r, &mut xs);
        assert!(xs.len() == 2);
        assert!(xs[0].t().approx_eq(4.0));
        assert!(xs[1].t().approx_eq(6.0));
    }

    #[test]
    fn ray_intersects_at_tangent() {
        let obj = Object::new_sphere();
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0))
            .with_transform(obj.get_transform().inverse());
        let mut xs = Intersections::new();
        obj.intersect(&r, &mut xs);
        assert!(xs.len() == 2);
        assert!(xs[0].t().approx_eq(5.0));
        assert!(xs[1].t().approx_eq(5.0));
    }

    #[test]
    fn ray_misses_sphere() {
        let obj = Object::new_sphere();
        let r = Ray::new(Point::new(0.0, 2.0, -0.5), Vector::new(0.0, 0.0, 1.0))
            .with_transform(obj.get_transform().inverse());
        let mut xs = Intersections::new();
        obj.intersect(&r, &mut xs);
        assert!(xs.len() == 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let obj = Object::new_sphere();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0))
            .with_transform(obj.get_transform().inverse());
        let mut xs = Intersections::new();
        obj.intersect(&r, &mut xs);
        assert!(xs.len() == 2);
        assert!(xs[0].t().approx_eq(-1.0));
        assert!(xs[1].t().approx_eq(1.0));
    }

    #[test]
    fn sphere_behind_ray() {
        let obj = Object::new_sphere();
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0))
            .with_transform(obj.get_transform().inverse());
        let mut xs = Intersections::new();
        obj.intersect(&r, &mut xs);
        assert!(xs.len() == 2);
        assert!(xs[0].t().approx_eq(-6.0));
        assert!(xs[1].t().approx_eq(-4.0));
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let obj = Object::new_sphere().scale(2.0, 2.0, 2.0);
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0))
            .with_transform(obj.get_transform().inverse());
        let sphere = obj.get_shape();
        let mut ints = Intersections::new();
        sphere.intersect(&r, &obj, &mut ints);
        assert!(ints.len() == 2);
        assert_eq!(ints[0].t(), 3.0);
        assert_eq!(ints[1].t(), 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let obj = Object::new_sphere().translate(5.0, 0.0, 0.0);
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0))
            .with_transform(obj.get_transform().inverse());
        let mut intersections = Intersections::new();
        obj.intersect(&r, &mut intersections);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn normal_on_a_sphere_x_axis() {
        let obj = Object::new_sphere();
        let p = obj.get_transform().inverse() * Point::new(1.0, 0.0, 0.0);
        let want = Vector::new(1.0, 0.0, 0.0);
        let got = obj.normal_at(&p);
        assert_eq!(got, want);
    }

    #[test]
    fn normal_on_a_sphere_y_axis() {
        let obj = Object::new_sphere();
        let p = obj.get_transform().inverse() * Point::new(0.0, 1.0, 0.0);
        let want = Vector::new(0.0, 1.0, 0.0);
        let got = obj.normal_at(&p);
        assert_eq!(got, want);
    }

    #[test]
    fn normal_on_a_sphere_z_axis() {
        let obj = Object::new_sphere();
        let p = obj.get_transform().inverse() * Point::new(0.0, 0.0, 1.0);
        let want = Vector::new(0.0, 0.0, 1.0);
        let got = obj.normal_at(&p);
        assert_eq!(got, want);
    }

    #[test]
    fn normal_on_a_sphere_non_axial() {
        let root_3_3 = ((3.0 as f64).sqrt()) / 3.0;
        let obj = Object::new_sphere();
        let p = obj.get_transform().inverse() * Point::new(root_3_3, root_3_3, root_3_3);
        let want = Vector::new(root_3_3, root_3_3, root_3_3);
        let got = obj.normal_at(&p);
        assert_eq!(got, want);
    }

    #[test]
    fn normal_is_normalized_vector() {
        let root_3_3 = ((3.0 as f64).sqrt()) / 3.0;
        let obj = Object::new_sphere();
        let p = obj.get_transform().inverse() * Point::new(root_3_3, root_3_3, root_3_3);
        let want = Vector::new(root_3_3, root_3_3, root_3_3).normalize();
        let got = obj.normal_at(&p);
        assert_eq!(got, want);
    }

    #[test]
    fn compute_the_normal_on_a_translated_sphere() {
        let obj = Object::new_sphere().translate(0.0, 1.0, 0.0);
        let point = Point::new(0.0, 1.70711, -0.70711);
        let got = obj.normal_at(&point);
        let want = Vector::new(0.0, 0.70711, -0.70711);
        assert_eq!(got, want);
    }
}
