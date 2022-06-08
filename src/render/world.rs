use crate::{
    draw::{canvas::Canvas, color::Color},
    math::{point::Point, ray::Ray, transformation::Transformable, tuple::Tuple},
    render::{
        intersections::Intersections, light::Light, lights::point_light::PointLight,
        material::Materialable, object::Object,
    },
};

use super::{camera::Camera, intersections::HitComputation};

#[derive(Debug)]
pub struct World {
    objects: Vec<Object>,
    lights: Vec<Light>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_object(&mut self, obj: Object) {
        self.objects.push(obj);
    }

    pub fn intersect<'a>(
        &self,
        ray: &Ray,
        objects: &'a [Object],
        intersections: &mut Intersections<'a>,
    ) {
        for obj in objects {
            obj.intersect(ray, intersections);
        }
    }

    pub fn shade_hit(&self, comp: &HitComputation) -> Color {
        if self.lights.len() == 0 {
            return Color::black();
        }
        self.lights.iter().fold(Color::black(), |acc, light| {
            let material = comp.object.get_material();
            let over_point = comp.over_point;
            let in_shadow = self.is_shadowed(&comp.over_point);
            let eye_vector = comp.eye;
            let normal_vector = comp.normal;
            acc + light.lighting(&material, over_point, eye_vector, normal_vector, in_shadow)
        })
    }

    pub fn is_shadowed(&self, point: &Point) -> bool {
        if self.lights.len() == 0 {
            return false;
        }
        let vector = self.lights[0].get_position() - *point;
        let distance = vector.magnitude();
        let direction = vector.normalize();
        let ray = Ray::new(*point, direction);
        let mut intersections = Intersections::new();
        self.intersect(&ray, &self.objects, &mut intersections);
        if let Some(hit) = intersections.get_hit() {
            return hit.t() < distance;
        }
        false
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let mut intersections = Intersections::new();
        self.intersect(ray, &self.objects, &mut intersections);
        if let Some(hit) = intersections.get_hit() {
            let comp = HitComputation::new(hit, ray);
            self.shade_hit(&comp)
        } else {
            Color::black()
        }
    }

    pub fn render(&self, camera: &Camera) -> Canvas {
        let width = camera.hsize();
        let height = camera.vsize();
        let mut canvas = Canvas::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let ray = camera.ray_for_pixel(x, y);
                let color = self.color_at(&ray);
                canvas.set_pixel((x, y), &color);
            }
        }

        canvas
    }
}

impl Default for World {
    fn default() -> Self {
        let light = Light::Point(PointLight::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));

        let s1 = Object::new_sphere()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s2 = Object::new_sphere().scale(0.5, 0.5, 0.5);

        let mut world = World::new();
        world.add_light(light);
        world.add_object(s1);
        world.add_object(s2);
        world
    }
}

#[cfg(test)]
mod test {

    use super::World;

    use crate::{
        draw::color::Color,
        math::{
            point::Point, ray::Ray, transformation::Transformable, tuple::Tuple, vector::Vector,
        },
        render::{
            intersections::{HitComputation, Intersection, Intersections},
            light::Light,
            lights::point_light::PointLight,
            material::{Material, Materialable},
            object::Object,
        },
    };
    #[test]
    fn new_world_has_no_objects_or_lights() {
        let world = World::new();
        assert_eq!(world.objects.len(), 0);
        assert_eq!(world.lights.len(), 0);
    }

    #[test]
    fn test_default_world() {
        let world = World::default();
        assert_eq!(world.lights.len(), 1);
        assert_eq!(world.objects.len(), 2);
    }

    #[test]
    fn intersect_world_with_a_ray() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut intersections = Intersections::new();
        world.intersect(&ray, &world.objects, &mut intersections);

        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].t(), 4.0);
        assert_eq!(intersections[1].t(), 4.5);
        assert_eq!(intersections[2].t(), 5.5);
        assert_eq!(intersections[3].t(), 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let world = World::default();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersection = Intersection::new(4.0, &world.objects[0]);
        let comp = HitComputation::new(&intersection, &ray);
        let got = world.shade_hit(&comp);
        let want = Color::new(0.38066, 0.47583, 0.2855);
        assert_eq!(got, want);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut world = World::default();
        let light = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        world.lights.clear();
        world.add_light(Light::Point(light));
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let intersection = Intersection::new(0.5, &world.objects[1]);
        let comp = HitComputation::new(&intersection, &ray);
        let got = world.shade_hit(&comp);
        let want = Color::new(0.90498, 0.90498, 0.90498);
        assert_eq!(got, want);
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        let mut mat1 = Material::default();
        mat1.ambient = 1.0;
        let mut mat2 = Material::default();
        mat2.ambient = 1.0;
        let mut obj2 = w.objects.pop().unwrap();
        let mut obj1 = w.objects.pop().unwrap();
        obj1 = obj1.with_material(mat1);
        obj2 = obj2.with_material(mat2);
        w.add_object(obj1);
        w.add_object(obj2);

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_eq!(c, w.objects[1].get_material().color);
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn shadow_when_object_is_between_pint_and_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(&p), true);
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.add_light(Light::Point(PointLight::new(
            Point::new(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        )));
        let s1 = Object::new_sphere();
        let s2 = Object::new_sphere().translate(0.0, 0.0, 10.0);
        let s2_copy = s2.clone();
        w.add_object(s1);
        w.add_object(s2);

        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let intersection = Intersection::new(4.0, &s2_copy);

        let comp = HitComputation::new(&intersection, &ray);
        let color = w.shade_hit(&comp);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }
}
