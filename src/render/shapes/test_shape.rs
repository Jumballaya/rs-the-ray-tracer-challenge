use std::sync::{Arc, Mutex};

use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::tuple::Tuple;
use crate::math::vector::Vector;
use crate::render::intersections::Intersections;
use crate::render::object::Object;

#[derive(Debug, Clone)]
pub struct TestShape {
    saved_ray: Arc<Mutex<Option<Ray>>>,
}

impl TestShape {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intersect<'a>(&self, ray: &Ray, _: &'a Object, _: &mut Intersections<'a>) {
        let mut refr = self.saved_ray.lock().unwrap();
        *refr = Some(*ray);
    }

    pub fn normal_at(&self, _: &Point) -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    pub fn get_saved_ray(&self) -> Option<Ray> {
        *self.saved_ray.lock().unwrap()
    }
}

impl Default for TestShape {
    fn default() -> Self {
        Self {
            saved_ray: Arc::new(Mutex::new(None)),
        }
    }
}

impl PartialEq for TestShape {
    fn eq(&self, _other: &TestShape) -> bool {
        unreachable!()
    }
}
