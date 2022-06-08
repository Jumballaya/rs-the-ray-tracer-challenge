/**
 *
 * Slight change of the Chapter 7/8 scenes to add a Plane for a floor
 *
 */
use std::f64::consts::PI;

use raytracer::{
    draw::color::Color,
    math::{point::Point, transformation::Transformable, tuple::Tuple, vector::Vector},
    render::{
        camera::Camera,
        light::Light,
        lights::point_light::PointLight,
        material::{Material, Materialable},
        object::Object,
        world::World,
    },
};

fn create_light() -> Light {
    Light::Point(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ))
}

fn create_floor_mat() -> Material {
    let mut mat = Material::default();
    mat.color = Color::new(1.0, 0.9, 0.9);
    mat.specular = 0.0;
    mat
}

fn create_floor() -> Object {
    Object::new_plane().with_material(create_floor_mat())
}

fn create_left() -> Object {
    Object::new_sphere()
        .translate(-0.5, 1.0, 0.5)
        .with_color(Color::new(0.1, 1.0, 0.5))
        .with_diffuse(0.7)
        .with_specular(0.3)
}

fn create_middle() -> Object {
    Object::new_sphere()
        .scale(0.5, 0.5, 0.5)
        .translate(1.5, 0.5, -0.5)
        .with_color(Color::new(0.5, 1.0, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
}

fn create_right() -> Object {
    Object::new_sphere()
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -0.75)
        .with_color(Color::new(1.0, 0.8, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
}

fn main() -> std::io::Result<()> {
    let width: usize = 10;
    let height: usize = 5;
    let mut world = World::new();
    world.add_light(create_light());
    world.add_object(create_floor());
    world.add_object(create_left());
    world.add_object(create_middle());
    world.add_object(create_right());

    let camera = Camera::new(width, height, PI / 3.0).view_transform(
        &Point::new(0.0, 0.5, -5.0),
        &Point::new(0.0, 1.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    world.render(&camera).save("./", "chapter9")
}
