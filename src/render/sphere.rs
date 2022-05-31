use std::sync::atomic::{AtomicUsize, Ordering};

use super::{
    hit::{Hittable, Intersection},
    object::ObjectType,
};
use crate::math::{ray::Ray, tuple::Tuple};

#[derive(Copy, Clone)]
pub struct Sphere {
    id: usize,
    tp: ObjectType,
}

static SPHERE_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: SPHERE_COUNTER.fetch_add(1, Ordering::SeqCst),
            tp: ObjectType::Sphere,
        }
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
        let sphere_to_ray = ray.origin - Tuple::new_point(0.0, 0.0, 0.0);
        let a = ray.direction * ray.direction;
        let b = 2.0 * (ray.direction * sphere_to_ray);
        let c = (sphere_to_ray * sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }
        let hit1 = (-b - (discriminant.sqrt())) / (2.0 * a);
        let hit2 = (-b + (discriminant.sqrt())) / (2.0 * a);
        let intersection1 = Intersection::new(Box::new(self), hit1);
        let intersection2 = Intersection::new(Box::new(self), hit2);
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
        math::{float_equal, ray::Ray},
        render::hit::{Hittable, Intersection},
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
    fn sphere_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(Box::new(s), 3.5);
        assert!(i.t == 3.5);
        assert!(i.object.get_id() == s.id);
    }
}
