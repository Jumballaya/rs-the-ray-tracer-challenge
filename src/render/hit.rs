use super::{super::math::ray::Ray, object::ObjectType};

pub trait Hittable {
    fn get_type(&self) -> ObjectType;
    fn get_id(&self) -> usize;
    fn intersect(self, ray: Ray) -> Vec<Intersection>;
}

impl PartialEq for dyn Hittable {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type() && self.get_id() == other.get_id()
    }
}

pub struct Intersection {
    pub t: f64,
    pub object: Box<dyn Hittable>,
}

impl Intersection {
    pub fn new(object: Box<dyn Hittable>, t: f64) -> Self {
        Intersection { t, object }
    }
}
