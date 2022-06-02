use std::sync::atomic::Ordering;

use crate::{
    draw::color::Color,
    math::{tuple::Tuple, EPSILON},
    render::material::Material,
};

use super::{LightType, LIGHT_COUNTER};

pub struct PointLight {
    tp: LightType,
    id: usize,
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            tp: LightType::Point,
            id: LIGHT_COUNTER.fetch_add(1, Ordering::SeqCst),
            position,
            intensity,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_type(&self) -> LightType {
        self.tp
    }

    pub fn lighting(
        &self,
        material: &Material,
        point: Tuple,
        eye_vector: Tuple,
        normal_vector: Tuple,
    ) -> Color {
        let effective_color = material.color * self.intensity;
        let light_vector = (self.position - point).normalize();
        let ambient = effective_color * material.ambient;
        let light_dot_normal = light_vector * normal_vector;

        let mut specular = Color::new(0.0, 0.0, 0.0);
        let mut diffuse = Color::new(0.0, 0.0, 0.0);

        if !(light_dot_normal < 0.0) {
            diffuse = effective_color * material.diffuse * light_dot_normal;
            let reflect_vector = -light_vector.reflect(&normal_vector);
            let reflect_dot_eye = reflect_vector * eye_vector;
            if !(reflect_dot_eye < (0.0 + EPSILON)) {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = self.intensity * material.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod test {
    use crate::{draw::color::Color, math::tuple::Tuple};

    use super::PointLight;

    #[test]
    fn light_point_light_has_pos_and_intensity() {
        let int = Color::new(1.0, 1.0, 1.0);
        let pos = Tuple::new_point(0.0, 0.0, 0.0);
        let light = PointLight::new(pos, int);

        let want_pos = Tuple::new_point(0.0, 0.0, 0.0);
        let want_int = Color::new(1.0, 1.0, 1.0);

        assert_eq!(light.intensity, want_int);
        assert_eq!(light.position, want_pos);
    }
}
