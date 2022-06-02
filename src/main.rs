mod draw;
mod math;
mod render;

use draw::{canvas::*, color::*};
use math::{ray::Ray, tuple::*};
use render::{
    hit::{Hittable, Intersection},
    light::{point::PointLight, Light},
    material::Material,
    object::{sphere::Sphere, Object},
};

pub fn create_object() -> Object {
    let mut sphere = Sphere::new();
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.2, 1.0);
    sphere.set_material(material);
    Object::Sphere(sphere)
}

pub fn create_light() -> Light {
    let pos = Tuple::new_point(-10.0, 10.0, -10.0);
    let color = Color::new(1.0, 1.0, 1.0);
    let point_light: PointLight = PointLight::new(pos, color);
    Light::Point(point_light)
}

pub fn draw(width: usize, height: usize, canvas: &mut Canvas) {
    let origin = &Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / (width as f64);
    let half = wall_size / 2.0;

    let obj = create_object();
    let light = create_light();

    for y in 0..height {
        let world_y = half - (pixel_size * (y as f64));
        for x in 0..width {
            let world_x = -half + (pixel_size * (x as f64));
            let pos = Tuple::new_point(world_x, world_y, wall_z);
            let origin_tuple = origin.as_tuple();
            let vector_tuple = (pos - origin).normalize().as_tuple();
            let ray = Ray::new(
                (origin_tuple.0, origin_tuple.1, origin_tuple.2),
                (vector_tuple.0, vector_tuple.1, vector_tuple.2),
            );
            let intersections = obj.intersect(ray);
            if let Some(hit) = Intersection::get_hit(&intersections) {
                let point = ray.position_at(hit.t);
                let normal_vector = hit.object.normal_at(&point);
                let eye_vector = -ray.direction;
                let color = &light.lighting(obj.get_material(), point, eye_vector, normal_vector);
                canvas.set_pixel((x, y), color);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut c = Canvas::new(100, 100);
    draw(100, 100, &mut c);
    c.save("./", "circle")
}
