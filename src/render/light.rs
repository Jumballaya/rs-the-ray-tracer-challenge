use crate::draw::color::Color;
use crate::math::point::Point;
use crate::math::vector::Vector;
use crate::render::material::Material;

use super::lights::point_light::PointLight;
use super::object::Object;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Light {
    Point(PointLight),
}

impl Light {
    pub fn get_position(&self) -> Point {
        match self {
            Self::Point(p) => p.position,
        }
    }

    pub fn lighting(
        &self,
        object: &Object,
        material: &Material,
        point: Point,
        eye_vector: Vector,
        normal_vector: Vector,
        in_shadow: bool,
    ) -> Color {
        match self {
            Self::Point(p) => p.lighting(
                object,
                material,
                point,
                eye_vector,
                normal_vector,
                in_shadow,
            ),
        }
    }
}
