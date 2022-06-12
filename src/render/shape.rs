use crate::{
    math::{point::Point, ray::Ray, vector::Vector},
    render::{intersections::Intersections, object::Object},
};

use crate::render::shapes::{plane::Plane, sphere::Sphere, test_shape::TestShape};

use super::shapes::{cone::Cone, cube::Cube, cylinder::Cylinder, group::Group, triangle::Triangle};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    TestShape(TestShape),
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Group),
    Triangle(Triangle),
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
            Self::Group(g) => g.normal_at(local_point),
            Self::Triangle(t) => t.normal_at(local_point),
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
            Self::Group(g) => g.intersect(local_ray, obj, intersections),
            Self::Triangle(t) => t.intersect(local_ray, obj, intersections),
        }
    }

    pub fn skip_world_to_local(&self) -> bool {
        matches!(self, Shape::Group(_))
    }

    pub fn as_triangle(&self) -> Option<Triangle> {
        match &self {
            Self::Triangle(t) => Some(t.clone()),
            _ => None,
        }
    }
}
