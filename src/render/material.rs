use super::pattern::Pattern;

pub const REFRACTION_VACUUM: f64 = 1.0;
pub const REFRACTION_AIR: f64 = 1.00029;
pub const REFRACTION_WATER: f64 = 1.333;
pub const REFRACTION_GLASS: f64 = 1.52;
pub const REFRACTION_DIAMOND: f64 = 2.417;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub pattern: Pattern,
}

impl Material {
    pub fn new(
        pattern: Pattern,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
        transparency: f64,
        refractive_index: f64,
    ) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
            pattern,
            reflective,
            transparency,
            refractive_index,
        }
    }

    pub fn with_pattern(self, pattern: Pattern) -> Self {
        Self { pattern, ..self }
    }

    pub fn with_ambient(self, ambient: f64) -> Self {
        Self { ambient, ..self }
    }

    pub fn with_diffuse(self, diffuse: f64) -> Self {
        Self { diffuse, ..self }
    }

    pub fn with_specular(self, specular: f64) -> Self {
        Self { specular, ..self }
    }

    pub fn with_shininess(self, shininess: f64) -> Self {
        Self { shininess, ..self }
    }

    pub fn with_reflective(self, reflective: f64) -> Self {
        Self { reflective, ..self }
    }

    pub fn with_transparency(self, transparency: f64) -> Self {
        Self {
            transparency,
            ..self
        }
    }

    pub fn with_refractive_index(self, refractive_index: f64) -> Self {
        Self {
            refractive_index,
            ..self
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: Pattern::default(),
        }
    }
}

pub trait Materialable {
    fn with_material(self, material: Material) -> Self;
    fn get_material(&self) -> Material;

    fn with_pattern(self, pattern: Pattern) -> Self
    where
        Self: Sized,
    {
        let mut mat = self.get_material();
        mat.pattern = pattern;
        self.with_material(mat)
    }

    fn with_ambient(self, ambient: f64) -> Self
    where
        Self: Sized,
    {
        let mut mat = self.get_material();
        mat.ambient = ambient;
        self.with_material(mat)
    }

    fn with_diffuse(self, diffuse: f64) -> Self
    where
        Self: Sized,
    {
        let mut mat = self.get_material();
        mat.diffuse = diffuse;
        self.with_material(mat)
    }

    fn with_specular(self, specular: f64) -> Self
    where
        Self: Sized,
    {
        let mut mat = self.get_material();
        mat.specular = specular;
        self.with_material(mat)
    }

    fn with_shininess(self, shininess: f64) -> Self
    where
        Self: Sized,
    {
        let mut mat = self.get_material();
        mat.shininess = shininess;
        self.with_material(mat)
    }

    fn with_reflective(self, reflective: f64) -> Self
    where
        Self: Sized,
    {
        let mut mat = self.get_material();
        mat.reflective = reflective;
        self.with_material(mat)
    }

    fn with_transparency(self, transparency: f64) -> Self
    where
        Self: Sized,
    {
        let mut mat = self.get_material();
        mat.transparency = transparency;
        self.with_material(mat)
    }

    fn with_refractive_index(self, refractive_index: f64) -> Self
    where
        Self: Sized,
    {
        let mut mat = self.get_material();
        mat.refractive_index = refractive_index;
        self.with_material(mat)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        draw::color::Color,
        math::{epsilon::ApproxEq, point::Point, tuple::Tuple, vector::Vector},
        render::{light::Light, lights::point_light::PointLight, object::Object, pattern::Pattern},
    };

    use super::Material;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(
            m.pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)),
            Color::new(1.0, 1.0, 1.0)
        );
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
        assert_eq!(m.reflective, 0.0);
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }

    #[test]
    fn material_with_eye_between_light_and_surface() {
        let obj = Object::new_test_shape();
        let m = Material::default();
        let pos = Point::new(0.0, 0.0, 0.0);
        let eye_vector = Vector::new(0.0, 0.0, -1.0);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let got = light.lighting(&obj, &m, pos, eye_vector, normal_vector, false);
        let want = Color::new(1.9, 1.9, 1.9);
        assert_eq!(got, want);
    }

    #[test]
    fn material_with_eye_between_light_and_surface_eye_offset_45_deg() {
        let obj = Object::new_test_shape();
        let m = Material::default();
        let pos = Point::new(0.0, 0.0, 0.0);

        let root_2_2 = (2.0 as f64).sqrt() / 2.0;

        let eye_vector = Vector::new(0.0, root_2_2, -root_2_2);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let got = light.lighting(&obj, &m, pos, eye_vector, normal_vector, false);
        let want = Color::new(1.0, 1.0, 1.0);
        assert_eq!(got, want);
    }

    #[test]
    fn material_with_eye_opposite_surface_light_offset_45_deg() {
        let obj = Object::new_test_shape();
        let m = Material::default();
        let pos = Point::new(0.0, 0.0, 0.0);
        let eye_vector = Vector::new(0.0, 0.0, -1.0);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let got = light.lighting(&obj, &m, pos, eye_vector, normal_vector, false);
        let want = Color::new(0.7364, 0.7364, 0.7364);
        assert_eq!(got, want);
    }

    #[test]
    fn material_with_eye_in_path_of_reflection_vector() {
        let obj = Object::new_test_shape();
        let m = Material::default();
        let pos = Point::new(0.0, 0.0, 0.0);

        let root_2_2 = (2.0 as f64).sqrt() / 2.0;

        let eye_vector = Vector::new(0.0, -root_2_2, -root_2_2);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let got = light.lighting(&obj, &m, pos, eye_vector, normal_vector, false);
        let want = Color::new(1.6364, 1.6364, 1.6364);
        assert_eq!(got, want);
    }

    #[test]
    fn material_with_light_behind_surface() {
        let obj = Object::new_test_shape();
        let m = Material::default();
        let pos = Point::new(0.0, 0.0, 0.0);
        let eye_vector = Vector::new(0.0, 0.0, -1.0);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let got = light.lighting(&obj, &m, pos, eye_vector, normal_vector, true);
        let want = Color::new(0.1, 0.1, 0.1);
        assert_eq!(got, want);
    }

    #[test]
    fn material_lighting_with_surface_in_shadow() {
        let obj = Object::new_test_shape();
        let m = Material::default();
        let point = Point::new(0.0, 0.0, 0.0);
        let eye_vector = Vector::new(0.0, 0.0, -1.0);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let got = light.lighting(&obj, &m, point, eye_vector, normal_vector, in_shadow);
        let want = Color::new(0.1, 0.1, 0.1);
        assert_eq!(got, want);
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let obj = Object::new_test_shape();
        let material = Material::new(
            Pattern::new_stripe(Color::white(), Color::black()),
            1.0,
            0.0,
            0.0,
            200.0,
            0.0,
            0.0,
            1.0,
        );
        let eye_vector = Vector::new(0.0, 0.0, -1.0);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let in_shadow = false;
        let light = Light::Point(PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white()));

        let c1 = light.lighting(
            &obj,
            &material,
            Point::new(0.9, 0.0, 0.0),
            eye_vector,
            normal_vector,
            in_shadow,
        );
        let c2 = light.lighting(
            &obj,
            &material,
            Point::new(1.1, 0.0, 0.0),
            eye_vector,
            normal_vector,
            in_shadow,
        );

        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }

    #[test]
    fn reflectivity_for_default_material() {
        let m = Material::default();
        assert!(m.reflective.approx_eq(0.0));
    }
}
