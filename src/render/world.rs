use crate::{
    draw::{canvas::Canvas, color::Color},
    math::{
        epsilon::ApproxEq, point::Point, ray::Ray, transformation::Transformable, tuple::Tuple,
    },
    render::{
        intersections::Intersections, light::Light, lights::point_light::PointLight,
        material::Materialable, object::Object, pattern::Pattern,
    },
};

use super::{camera::Camera, intersections::HitComputation};

const REMAINING: usize = 5;

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

    pub fn shade_hit(&self, comp: &HitComputation, remaining: usize) -> Color {
        self.lights.iter().fold(Color::black(), |acc, light| {
            let material = comp.object.get_material();
            let over_point = comp.over_point;
            let eye_vector = comp.eye;
            let normal_vector = comp.normal;
            let in_shadow = self.is_shadowed(&comp.over_point);
            let surface = light.lighting(
                comp.object,
                &material,
                over_point,
                eye_vector,
                normal_vector,
                in_shadow,
            );

            let reflected = self.reflected_color(&comp, remaining);
            let refracted = self.refracted_color(&comp, remaining);

            let is_reflective = comp.object.get_material().reflective > 0.0;
            let is_transparent = comp.object.get_material().transparency > 0.0;

            if is_reflective && is_transparent {
                let reflectance = comp.schlick();

                acc + surface + reflected * reflectance + refracted * (1.0 - reflectance)
            } else {
                acc + surface + reflected + refracted
            }
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

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> Color {
        let mut intersections = Intersections::new();
        self.intersect(ray, &self.objects, &mut intersections);

        match intersections.get_hit_index() {
            Some(index) => {
                let comp = HitComputation::new(&intersections, index, ray);
                self.shade_hit(&comp, remaining)
            }
            None => Color::black(),
        }
    }

    pub fn reflected_color(&self, comp: &HitComputation, remaining: usize) -> Color {
        if remaining == 0 || comp.object.get_material().reflective.approx_eq(0.0) {
            return Color::black();
        }
        let reflect_ray = Ray::new(comp.over_point, comp.reflect);
        let color = self.color_at(&reflect_ray, remaining - 1);
        color * comp.object.get_material().reflective
    }

    pub fn refracted_color(&self, comp: &HitComputation, remaining: usize) -> Color {
        if remaining == 0 || comp.object.get_material().transparency.approx_eq(0.0) {
            return Color::black();
        }

        let (n1, n2) = comp.n();
        let n_ratio = n1 / n2;
        let cos_i = comp.cos_i;
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);

        if sin2_t > 1.0 {
            Color::black()
        } else {
            let cos_t = f64::sqrt(1.0 - sin2_t);
            let direction = comp.normal * (n_ratio * cos_i - cos_t) - comp.eye * n_ratio;
            let refract_ray = Ray::new(comp.under_point, direction);
            self.color_at(&refract_ray, remaining - 1) * comp.object.get_material().transparency
        }
    }

    pub fn render(&self, camera: &Camera) -> Canvas {
        let width = camera.hsize();
        let height = camera.vsize();
        let mut canvas = Canvas::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let ray = camera.ray_for_pixel(x, y);
                let color = self.color_at(&ray, REMAINING);
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
            .with_pattern(Pattern::new_solid(Color::new(0.8, 1.0, 0.6)))
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
            pattern::Pattern,
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
        let comp = HitComputation::new(
            &Intersections::new().with_intersections(vec![intersection]),
            0,
            &ray,
        );
        let got = world.shade_hit(&comp, 5);
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
        let comp = HitComputation::new(
            &Intersections::new().with_intersections(vec![intersection]),
            0,
            &ray,
        );
        let got = world.shade_hit(&comp, 5);
        let want = Color::new(0.90498, 0.90498, 0.90498);
        assert_eq!(got, want);
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r, 5);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r, 5);
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
        let c = w.color_at(&r, 5);
        assert_eq!(
            c,
            w.objects[1]
                .get_material()
                .pattern
                .pattern_at(&Point::new(0.0, 0.0, 0.0))
        );
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

        let comp = HitComputation::new(
            &Intersections::new().with_intersections(vec![intersection]),
            0,
            &ray,
        );
        let color = w.shade_hit(&comp, 5);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn reflected_color_for_non_reflective_material() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let obj = w.objects[1].clone().with_ambient(1.0);
        let int = Intersection::new(1.0, &obj);
        let comp = HitComputation::new(&Intersections::new().with_intersections(vec![int]), 0, &r);
        let got = w.reflected_color(&comp, 5);
        let want = Color::black();
        assert_eq!(got, want);
    }

    #[test]
    fn reflected_color_for_reflective_material() {
        let root2 = f64::sqrt(2.0);
        let root_2_2 = root2 / 2.0;

        let mut w = World::default();

        w.add_object(
            Object::new_plane()
                .with_material(Material::default().with_reflective(0.5))
                .translate(0.0, -1.0, 0.0),
        );

        let object = &w.objects.last().unwrap();

        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -root_2_2, root_2_2),
        );
        let int = Intersection::new(root2, &object);
        let comp =
            HitComputation::new(&Intersections::new().with_intersections(vec![int]), 0, &ray);
        let want = Color::new(0.190346, 0.23793, 0.142759);
        let got = w.reflected_color(&comp, 5);
        assert_eq!(got, want);
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let root2 = f64::sqrt(2.0);
        let root_2_2 = root2 / 2.0;

        let mut w = World::default();
        w.add_object(
            Object::new_plane()
                .with_reflective(0.5)
                .translate(0.0, -1.0, 0.0),
        );
        let object = &w.objects.last().unwrap();
        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -root_2_2, root_2_2),
        );
        let int = Intersection::new(root2, &object);
        let comp =
            HitComputation::new(&Intersections::new().with_intersections(vec![int]), 0, &ray);
        let want = Color::new(0.87677, 0.92436, 0.82918);
        let got = w.shade_hit(&comp, 5);
        assert_eq!(got, want);
    }

    #[test]
    fn mutually_reflective_surfaces() {
        let mut w = World::new();
        w.add_light(Light::Point(PointLight::new(
            Point::new(0.0, 0.0, 0.0),
            Color::new(1.0, 1.0, 1.0),
        )));
        w.add_object(
            Object::new_plane()
                .with_reflective(1.0)
                .translate(0.0, -1.0, 0.0),
        );
        w.add_object(
            Object::new_plane()
                .with_reflective(1.0)
                .translate(0.0, 1.0, 0.0),
        );

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));

        w.color_at(&ray, 5);
    }

    #[test]
    fn reflected_color_at_max_recursive_depth() {
        let root2 = f64::sqrt(2.0);
        let root_2_2 = root2 / 2.0;

        let mut w = World::default();
        w.add_object(
            Object::new_plane()
                .with_reflective(0.5)
                .translate(0.0, -1.0, 0.0),
        );
        let object = &w.objects.last().unwrap();
        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -root_2_2, root_2_2),
        );
        let int = Intersection::new(root2, &object);
        let comp =
            HitComputation::new(&Intersections::new().with_intersections(vec![int]), 0, &ray);
        let want = Color::black();
        let got = w.reflected_color(&comp, 0);
        assert_eq!(got, want);
    }

    #[test]
    fn refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let shape = w.objects.first().unwrap();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = Intersections::new().with_intersections(vec![
            Intersection::new(4.0, shape),
            Intersection::new(6.0, shape),
        ]);
        let comp = HitComputation::new(&intersections, 0, &r);
        let got = w.refracted_color(&comp, 5);
        let want = Color::black();
        assert_eq!(got, want);
    }

    #[test]
    fn refracted_color_at_max_recursive_depth() {
        let mut w = World::default();
        w.objects[0] = World::default()
            .objects
            .first()
            .unwrap()
            .clone()
            .with_transparency(1.0)
            .with_refractive_index(1.5);
        let obj1 = w.objects.first().unwrap();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = Intersections::new().with_intersections(vec![
            Intersection::new(4.0, &obj1),
            Intersection::new(6.0, &obj1),
        ]);
        let comp = HitComputation::new(&intersections, 0, &ray);
        let got = w.refracted_color(&comp, 0);
        let want = Color::black();
        assert_eq!(got, want);
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let root_2_2 = (2.0 as f64).sqrt() / 2.0;
        let mut w = World::default();
        w.objects[0] = w
            .objects
            .first()
            .unwrap()
            .clone()
            .with_transparency(1.0)
            .with_refractive_index(1.5);
        let obj1 = w.objects.first().unwrap();
        let ray = Ray::new(Point::new(0.0, 0.0, root_2_2), Vector::new(0.0, 1.0, 0.0));
        let intersections = Intersections::new().with_intersections(vec![
            Intersection::new(-root_2_2, &obj1),
            Intersection::new(root_2_2, &obj1),
        ]);
        let comp = HitComputation::new(&intersections, 1, &ray);
        let got = w.refracted_color(&comp, 5);
        let want = Color::black();
        assert_eq!(got, want);
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut w = World::default();
        w.objects[0] = w
            .objects
            .first()
            .unwrap()
            .clone()
            .with_ambient(1.0)
            .with_pattern(Pattern::new_test());
        w.objects[1] = w
            .objects
            .last()
            .unwrap()
            .clone()
            .with_transparency(1.0)
            .with_refractive_index(1.5);
        let obj1 = w.objects.first().unwrap();
        let obj2 = w.objects.last().unwrap();

        let ray = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
        let intersections = Intersections::new().with_intersections(vec![
            Intersection::new(-0.9899, &obj1),
            Intersection::new(-0.4899, &obj2),
            Intersection::new(0.4899, &obj2),
            Intersection::new(0.9899, &obj1),
        ]);
        let comp = HitComputation::new(&intersections, 2, &ray);
        let got = w.refracted_color(&comp, 5);
        let want = Color::new(0.0, 0.99878, 0.04724);
        assert_eq!(got, want);
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let root_2 = f64::sqrt(2.0);
        let root_2_2 = root_2 / 2.0;
        let mut w = World::default();
        let floor = Object::new_plane()
            .with_transparency(0.5)
            .with_refractive_index(1.5)
            .translate(0.0, -1.0, 0.0);
        let ball = Object::new_sphere()
            .with_pattern(Pattern::new_solid(Color::red()))
            .with_ambient(0.5)
            .translate(0.0, -3.5, -0.5);
        w.add_object(floor.clone());
        w.add_object(ball);
        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -root_2_2, root_2_2),
        );
        let intersections =
            Intersections::new().with_intersections(vec![Intersection::new(root_2, &floor)]);
        let comp = HitComputation::new(&&intersections, 0, &ray);
        let got = w.shade_hit(&comp, 5);
        let want = Color::new(0.93642, 0.68642, 0.68642);
        assert_eq!(got, want);
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let root_2 = f64::sqrt(2.0);
        let root_2_2 = root_2 / 2.0;
        let mut w = World::default();
        let floor = Object::new_plane()
            .with_transparency(0.5)
            .with_reflective(0.5)
            .with_refractive_index(1.5)
            .translate(0.0, -1.0, 0.0);
        let ball = Object::new_sphere()
            .with_pattern(Pattern::new_solid(Color::red()))
            .with_ambient(0.5)
            .translate(0.0, -3.5, -0.5);
        w.add_object(floor.clone());
        w.add_object(ball);
        let ray = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -root_2_2, root_2_2),
        );
        let intersections =
            Intersections::new().with_intersections(vec![Intersection::new(root_2, &floor)]);
        let comp = HitComputation::new(&&intersections, 0, &ray);
        let got = w.shade_hit(&comp, 5);
        let want = Color::new(0.93391, 0.69643, 0.69243);
        assert_eq!(got, want);
    }
}
