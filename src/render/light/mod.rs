pub mod point;
use std::sync::atomic::AtomicUsize;

use crate::{draw::color::Color, math::tuple::Tuple};

use self::point::PointLight;

use super::material::Material;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LightType {
    Point,
}

static LIGHT_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub enum Light {
    Point(PointLight),
}

impl Light {
    pub fn get_id(&self) -> usize {
        match self {
            Self::Point(p) => p.get_id(),
        }
    }

    pub fn get_type(&self) -> LightType {
        match self {
            Self::Point(p) => p.get_type(),
        }
    }

    pub fn lighting(
        &self,
        material: &Material,
        point: Tuple,
        eye_vector: Tuple,
        normal_vector: Tuple,
    ) -> Color {
        match self {
            Self::Point(p) => p.lighting(material, point, eye_vector, normal_vector),
        }
    }
}
