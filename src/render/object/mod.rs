use self::sphere::Sphere;
use super::{
    hit::{Hittable, Intersection},
    material::Material,
};
use crate::math::{ray::Ray, tuple::Tuple};
use std::sync::atomic::AtomicUsize;

pub mod sphere;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectType {
    Sphere,
}

static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
}

impl Object {
    pub fn get_id(&self) -> usize {
        match self {
            Self::Sphere(s) => s.get_id(),
        }
    }

    pub fn get_type(&self) -> ObjectType {
        match self {
            Self::Sphere(s) => s.get_type(),
        }
    }

    pub fn get_material(&self) -> &Material {
        match self {
            Self::Sphere(s) => s.get_material(),
        }
    }

    pub fn set_material(&mut self, mat: Material) {
        match self {
            Self::Sphere(s) => s.set_material(mat),
        }
    }

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        match self {
            Self::Sphere(s) => s.normal_at(world_point),
        }
    }
}

impl Hittable for Object {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        match self {
            Self::Sphere(s) => s.intersect(ray),
        }
    }
}
