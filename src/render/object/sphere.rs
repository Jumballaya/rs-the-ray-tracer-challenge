use std::sync::atomic::Ordering;

use super::{Object, ObjectType, OBJECT_COUNTER};
use crate::math::matrix::{Matrix, Transformation};
use crate::math::{ray::Ray, tuple::Tuple};
use crate::render::hit::{Hittable, Intersection};

#[derive(Debug, Clone)]
pub struct Sphere {
    id: usize,
    tp: ObjectType,
    pub transform: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst),
            tp: ObjectType::Sphere,
            transform: Matrix::identity_matrix(4),
        }
    }

    pub fn set_transform(&mut self, tform: Transformation) {
        let m = Matrix::transform(tform);
        self.transform = m;
    }

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        (point - Tuple::new_point(0.0, 0.0, 0.0)).normalize()
    }
}

impl Hittable for Sphere {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_type(&self) -> ObjectType {
        self.tp
    }

    fn intersect(self, ray: Ray) -> Vec<Intersection> {
        let tformed_ray = ray.transform(&self.clone().transform.inverse());
        let sphere_to_ray = tformed_ray.origin - Tuple::new_point(0.0, 0.0, 0.0);
        let a = tformed_ray.direction * tformed_ray.direction;
        let b = 2.0 * (tformed_ray.direction * sphere_to_ray);
        let c = (sphere_to_ray * sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }
        let hit1 = (-b - (discriminant.sqrt())) / (2.0 * a);
        let hit2 = (-b + (discriminant.sqrt())) / (2.0 * a);
        let s_clone = self.clone();
        let intersection1 = Intersection::new(Object::Sphere(self), hit1);
        let intersection2 = Intersection::new(Object::Sphere(s_clone), hit2);
        vec![intersection1, intersection2]
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.tp == other.tp && self.id == other.id
    }
}

#[cfg(test)]
mod test {
    use crate::{
        math::{
            float_equal,
            matrix::{Matrix, Transformation},
            ray::Ray,
            tuple::Tuple,
        },
        render::hit::Hittable,
    };

    use super::Sphere;

    #[test]
    fn sphere_ray_intersects_at_2_points() {
        let r = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert!(xs.len() == 2);
        assert!(float_equal(xs[0].t, 4.0));
        assert!(float_equal(xs[1].t, 6.0));
    }

    #[test]
    fn sphere_ray_intersects_at_tangent() {
        let r = Ray::new((0.0, 1.0, -5.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert!(xs.len() == 2);
        assert!(float_equal(xs[0].t, 5.0));
        assert!(float_equal(xs[1].t, 5.0));
    }

    #[test]
    fn sphere_ray_misses_sphere() {
        let r = Ray::new((0.0, 2.0, -0.5), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert!(xs.len() == 0);
    }

    #[test]
    fn sphere_ray_originates_inside_sphere() {
        let r = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert!(xs.len() == 2);
        assert!(float_equal(xs[0].t, -1.0));
        assert!(float_equal(xs[1].t, 1.0));
    }

    #[test]
    fn sphere_sphere_behind_ray() {
        let r = Ray::new((0.0, 0.0, 5.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert!(xs.len() == 2);
        assert!(float_equal(xs[0].t, -6.0));
        assert!(float_equal(xs[1].t, -4.0));
    }

    #[test]
    fn sphere_default_tfrom_is_ident() {
        let s = Sphere::new();
        let m = Matrix::identity_matrix(4);
        assert_eq!(s.transform, m);
    }

    #[test]
    fn sphere_change_spheres_tform() {
        let mut s = Sphere::new();
        let translate = Transformation::Translate(2.0, 3.0, 4.0);
        s.set_transform(translate);
        assert_eq!(s.transform, Matrix::transform(translate));
    }

    #[test]
    fn sphere_intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Transformation::Scale(2.0, 2.0, 2.0));
        let intersections = s.intersect(r);
        assert!(intersections.len() == 2);
        assert_eq!(intersections[0].t, 3.0);
        assert_eq!(intersections[1].t, 7.0);
    }

    #[test]
    fn sphere_intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Transformation::Translate(5.0, 0.0, 0.0));
        let intersections = s.intersect(r);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn sphere_normal_on_a_sphere_x_axis() {
        let s = Sphere::new();
        let p = Tuple::new_point(1.0, 0.0, 0.0);
        let want = Tuple::new_vector(1.0, 0.0, 0.0);
        let got = s.normal_at(p);
        assert_eq!(got, want);
    }

    #[test]
    fn sphere_normal_on_a_sphere_y_axis() {
        let s = Sphere::new();
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let want = Tuple::new_vector(0.0, 1.0, 0.0);
        let got = s.normal_at(p);
        assert_eq!(got, want);
    }

    #[test]
    fn sphere_normal_on_a_sphere_z_axis() {
        let s = Sphere::new();
        let p = Tuple::new_point(0.0, 0.0, 1.0);
        let want = Tuple::new_vector(0.0, 0.0, 1.0);
        let got = s.normal_at(p);
        assert_eq!(got, want);
    }

    #[test]
    fn sphere_normal_on_a_sphere_non_axial() {
        let root_3_3 = ((3.0 as f64).sqrt()) / 3.0;
        let s = Sphere::new();
        let p = Tuple::new_point(root_3_3, root_3_3, root_3_3);
        let want = Tuple::new_vector(root_3_3, root_3_3, root_3_3);
        let got = s.normal_at(p);
        assert_eq!(got, want);
    }

    #[test]
    fn sphere_normal_is_normalized_vector() {
        let root_3_3 = ((3.0 as f64).sqrt()) / 3.0;
        let s = Sphere::new();
        let p = Tuple::new_point(root_3_3, root_3_3, root_3_3);
        let want = Tuple::new_vector(root_3_3, root_3_3, root_3_3).normalize();
        let got = s.normal_at(p);
        assert_eq!(got, want);
    }
}
