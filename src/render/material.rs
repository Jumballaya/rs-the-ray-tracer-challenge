use crate::{draw::color::Color, math::float_equal};

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && float_equal(self.ambient, other.ambient)
            && float_equal(self.diffuse, other.diffuse)
            && float_equal(self.specular, other.specular)
            && float_equal(self.shininess, other.shininess)
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl PartialEq<Material> for &Material {
    fn eq(&self, other: &Material) -> bool {
        self.color == other.color
            && float_equal(self.ambient, other.ambient)
            && float_equal(self.diffuse, other.diffuse)
            && float_equal(self.specular, other.specular)
            && float_equal(self.shininess, other.shininess)
    }

    fn ne(&self, other: &Material) -> bool {
        !(self == other)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        draw::color::Color,
        math::{matrix::Transformation, tuple::Tuple},
        render::{
            light::{point::PointLight, Light},
            object::{sphere::Sphere, Object},
            world::World,
        },
    };

    use super::Material;

    #[test]
    fn material_default_material() {
        let m = Material::default();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn material_with_eye_between_light_and_surface() {
        let m = Material::default();
        let pos = Tuple::new_point(0.0, 0.0, 0.0);
        let eye_vector = Tuple::new_vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let got = light.lighting(&m, pos, eye_vector, normal_vector, false);
        let want = Color::new(1.9, 1.9, 1.9);
        assert_eq!(got, want);
    }

    #[test]
    fn material_with_eye_between_light_and_surface_eye_offset_45_deg() {
        let m = Material::default();
        let pos = Tuple::new_point(0.0, 0.0, 0.0);

        let root_2_2 = (2.0 as f64).sqrt() / 2.0;

        let eye_vector = Tuple::new_vector(0.0, root_2_2, -root_2_2);
        let normal_vector = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let got = light.lighting(&m, pos, eye_vector, normal_vector, false);
        let want = Color::new(1.0, 1.0, 1.0);
        assert_eq!(got, want);
    }

    #[test]
    fn material_with_eye_opposite_surface_light_offset_45_deg() {
        let m = Material::default();
        let pos = Tuple::new_point(0.0, 0.0, 0.0);
        let eye_vector = Tuple::new_vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple::new_point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        let got = light.lighting(&m, pos, eye_vector, normal_vector, false);
        let want = Color::new(0.7364, 0.7364, 0.7364);
        assert_eq!(got, want);
    }

    #[test]
    fn material_with_eye_in_path_of_reflection_vector() {
        let m = Material::default();
        let pos = Tuple::new_point(0.0, 0.0, 0.0);

        let root_2_2 = (2.0 as f64).sqrt() / 2.0;

        let eye_vector = Tuple::new_vector(0.0, -root_2_2, -root_2_2);
        let normal_vector = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple::new_point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        let got = light.lighting(&m, pos, eye_vector, normal_vector, false);
        let want = Color::new(1.6364, 1.6364, 1.6364);
        assert_eq!(got, want);
    }

    #[test]
    fn material_with_light_behind_surface() {
        let m = Material::default();
        let pos = Tuple::new_point(0.0, 0.0, 0.0);
        let eye_vector = Tuple::new_vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::new_point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let got = light.lighting(&m, pos, eye_vector, normal_vector, true);
        let want = Color::new(0.1, 0.1, 0.1);
        assert_eq!(got, want);
    }

    #[test]
    fn material_lighting_with_surface_in_shadow() {
        let m = Material::default();
        let point = Tuple::new_point(0.0, 0.0, 0.0);
        let eye_vector = Tuple::new_vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::new_point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let got = light.lighting(&m, point, eye_vector, normal_vector, in_shadow);
        let want = Color::new(0.1, 0.1, 0.1);
        assert_eq!(got, want);
    }
}
