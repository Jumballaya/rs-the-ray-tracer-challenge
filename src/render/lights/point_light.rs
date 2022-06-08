use crate::math::point::Point;
use crate::math::vector::Vector;
use crate::render::object::Object;
use crate::{draw::color::Color, render::material::Material};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity,
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
        let effective_color = material.pattern.pattern_at_object(object, &point) * self.intensity;
        let light_vector = (self.position - point).normalize();
        let ambient = effective_color * material.ambient;
        let light_dot_normal = light_vector * normal_vector;

        let mut specular = Color::new(0.0, 0.0, 0.0);
        let mut diffuse = Color::new(0.0, 0.0, 0.0);

        if !(light_dot_normal < 0.0) && !in_shadow {
            diffuse = effective_color * material.diffuse * light_dot_normal;
            let reflect_vector = -light_vector.reflect(&normal_vector);
            let reflect_dot_eye = reflect_vector * eye_vector;
            if !(reflect_dot_eye < 0.0) {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = self.intensity * material.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::math::tuple::Tuple;

    #[test]
    fn light_point_light_has_pos_and_intensity() {
        let int = Color::new(1.0, 1.0, 1.0);
        let pos = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(pos, int);

        let want_pos = Point::new(0.0, 0.0, 0.0);
        let want_int = Color::new(1.0, 1.0, 1.0);

        assert_eq!(light.intensity, want_int);
        assert_eq!(light.position, want_pos);
    }
}
