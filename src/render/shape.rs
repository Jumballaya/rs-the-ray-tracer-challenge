use crate::{
    math::{point::Point, ray::Ray, vector::Vector},
    render::{intersections::Intersections, object::Object},
};

use crate::render::shapes::{plane::Plane, sphere::Sphere, test_shape::TestShape};

use super::shapes::{cone::Cone, cube::Cube, cylinder::Cylinder};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    TestShape(TestShape),
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
}

impl Shape {
    pub fn normal_at(&self, local_point: &Point) -> Vector {
        match self {
            Self::Plane(_) => Plane::normal_at(local_point),
            Self::Sphere(s) => s.normal_at(local_point),
            Self::TestShape(ts) => ts.normal_at(local_point),
            Self::Cube(c) => c.normal_at(local_point),
            Self::Cylinder(c) => c.normal_at(local_point),
            Self::Cone(c) => c.normal_at(local_point),
        }
    }

    pub fn intersect<'a>(
        &'a self,
        local_ray: &Ray,
        obj: &'a Object,
        intersections: &mut Intersections<'a>,
    ) {
        match self {
            Self::TestShape(ts) => ts.intersect(local_ray, obj, intersections),
            Self::Sphere(s) => s.intersect(local_ray, obj, intersections),
            Self::Plane(p) => p.intersect(local_ray, obj, intersections),
            Self::Cube(c) => c.intersect(local_ray, obj, intersections),
            Self::Cylinder(c) => c.intersect(local_ray, obj, intersections),
            Self::Cone(c) => c.intersect(local_ray, obj, intersections),
        }
    }
}
