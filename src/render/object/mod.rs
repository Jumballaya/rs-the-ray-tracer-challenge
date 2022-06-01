use self::sphere::Sphere;
use super::hit::{Hittable, Intersection};
use crate::math::ray::Ray;
use std::sync::atomic::AtomicUsize;

pub mod sphere;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectType {
    Sphere,
}

static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub enum Object {
    Sphere(Sphere),
}

impl Hittable for Object {
    fn get_id(&self) -> usize {
        match self {
            Self::Sphere(s) => s.get_id(),
        }
    }

    fn get_type(&self) -> ObjectType {
        match self {
            Self::Sphere(s) => s.get_type(),
        }
    }

    fn intersect(self, ray: Ray) -> Vec<Intersection> {
        match self {
            Self::Sphere(s) => s.intersect(ray),
        }
    }
}
