use crate::{
    math::{point::Point, ray::Ray, vector::Vector},
    render::{intersections::Intersections, object::Object},
};

use crate::render::shapes::{plane::Plane, sphere::Sphere, test_shape::TestShape};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    TestShape(TestShape),
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape {
    pub fn normal_at(&self, local_point: &Point) -> Vector {
        match self {
            Self::Plane(_) => Plane::normal_at(local_point),
            Self::Sphere(s) => s.normal_at(local_point),
            Self::TestShape(ts) => ts.normal_at(local_point),
        }
    }

    pub fn intersect<'a>(
        &'a self,
        local_ray: &Ray,
        obj: &'a Object,
        intersections: &mut Intersections<'a>,
    ) {
        match self {
            Shape::TestShape(ts) => ts.intersect(local_ray, obj, intersections),
            Shape::Sphere(s) => s.intersect(local_ray, obj, intersections),
            Shape::Plane(p) => p.intersect(local_ray, obj, intersections),
        }
    }
}
