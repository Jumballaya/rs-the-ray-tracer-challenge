use crate::{
    draw::{canvas::Canvas, color::Color},
    math::ray::Ray,
};

use super::{
    camera::Camera,
    hit::{HitComputation, Hittable, Intersection},
    light::Light,
    object::Object,
};

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub camera: Camera,

    width: usize,
    height: usize,
}

impl World {
    pub fn new(width: usize, height: usize, field_of_view: f64) -> Self {
        Self {
            height,
            width,
            objects: vec![],
            lights: vec![],
            camera: Camera::new(height, width, field_of_view),
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn empty_lights(&mut self) {
        self.lights.clear();
    }

    pub fn add_object(&mut self, obj: Object) {
        self.objects.push(obj);
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut hits: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|obj| obj.intersect(ray))
            .collect();
        hits.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        hits
    }

    pub fn shade_hit(&self, intersection: &HitComputation) -> Color {
        if self.lights.len() == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let obj = intersection.object;
        let point = intersection.point;
        let eye_vector = intersection.eye_vector;
        let normal_vector = intersection.normal_vector;
        self.lights[0].lighting(obj.get_material(), point, eye_vector, normal_vector)
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(ray);
        if intersections.len() > 0 {
            let comp = Intersection::prepare_computation(&intersections[0], ray);
            self.shade_hit(&comp)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    pub fn render(&self) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);

        for y in 0..self.width {
            for x in 0..self.height {
                let ray = self.camera.ray_for_pixel(x, y);
                let color = self.color_at(&ray);
                canvas.set_pixel((x, y), &color);
            }
        }

        canvas
    }
}

#[cfg(test)]
mod test {

    use crate::{
        draw::color::Color,
        math::{matrix::Transformation, ray::Ray, tuple::Tuple},
        render::{
            hit::Intersection,
            light::{point::PointLight, Light, LightType},
            material::Material,
            object::{sphere::Sphere, Object, ObjectType},
        },
    };

    use super::World;

    fn default_world() -> World {
        let light = PointLight::new(
            Tuple::new_point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        let mut s1 = Sphere::new();
        let mut s2 = Sphere::new();
        let mut material = Material::default();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        s1.set_material(material);
        s2.set_transform(Transformation::Scale(0.5, 0.5, 0.5));

        let mut world = World::new(1, 1, 1.0);
        world.add_light(Light::Point(light));
        world.add_object(Object::Sphere(s1));
        world.add_object(Object::Sphere(s2));
        world
    }

    #[test]
    fn world_new_world_has_no_objects_or_lights() {
        let world = World::new(1, 1, 1.0);
        assert_eq!(world.objects.len(), 0);
        assert_eq!(world.lights.len(), 0);
    }

    #[test]
    fn world_test_default_world() {
        let world = default_world();

        assert_eq!(world.lights.len(), 1);
        assert_eq!(world.lights[0].get_type(), LightType::Point);

        assert_eq!(world.objects.len(), 2);
        assert_eq!(world.objects[0].get_type(), ObjectType::Sphere);
        assert_eq!(world.objects[1].get_type(), ObjectType::Sphere);
    }

    #[test]
    fn world_intersect_world_with_a_ray() {
        let world = default_world();
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let intersections = world.intersect(&ray);

        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 4.5);
        assert_eq!(intersections[2].t, 5.5);
        assert_eq!(intersections[3].t, 6.0);
    }

    #[test]
    fn world_shading_an_intersection() {
        let world = default_world();
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let intersection = Intersection::new(world.objects[0].clone(), 4.0);
        let comp = Intersection::prepare_computation(&intersection, &ray);
        let got = world.shade_hit(&comp);
        let want = Color::new(0.38066, 0.47583, 0.2855);
        assert_eq!(got, want);
    }

    #[test]
    fn world_shading_an_intersection_from_the_inside() {
        let mut world = default_world();
        let light = PointLight::new(Tuple::new_point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        world.empty_lights();
        world.add_light(Light::Point(light));
        let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let intersection = Intersection::new(world.objects[1].clone(), 0.5);
        let comp = Intersection::prepare_computation(&intersection, &ray);
        let got = world.shade_hit(&comp);
        let want = Color::new(0.90498, 0.90498, 0.90498);
        assert_eq!(got, want);
    }

    #[test]
    fn world_color_when_ray_misses() {
        let w = default_world();
        let r = Ray::new((0.0, 0.0, -5.0), (0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn world_color_when_ray_hits() {
        let w = default_world();
        let r = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn world_color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();
        let mut mat1 = Material::default();
        mat1.ambient = 1.0;
        let mut mat2 = Material::default();
        mat2.ambient = 1.0;
        w.objects[0].set_material(mat1);
        w.objects[1].set_material(mat2);

        let r = Ray::new((0.0, 0.0, 0.75), (0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_eq!(c, w.objects[1].get_material().color);
    }
}
