/**
 *
 * Reflection/Refraction/Transparency
 *
 */
use std::f64::consts::PI;

use raytracer::{
    draw::color::Color,
    math::{point::Point, transformation::Transformable, tuple::Tuple, vector::Vector},
    render::{
        camera::Camera, light::Light, lights::point_light::PointLight, material::Materialable,
        object::Object, pattern::Pattern, world::World,
    },
};

fn create_light() -> Light {
    Light::Point(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ))
}

fn create_floor() -> Object {
    Object::new_plane()
        .with_pattern(Pattern::new_checker(Color::black(), Color::white()))
        .with_reflective(0.9)
}

fn create_middle() -> Object {
    Object::new_sphere()
        .translate(-0.5, 1.0, 0.5)
        .with_pattern(
            Pattern::new_gradient(Color::new(0.1, 0.4, 0.7), Color::new(0.7, 0.1, 0.4))
                .scale(0.66, 0.66, 0.66)
                .rotate_y(PI / 6.0),
        )
        .with_diffuse(0.9)
        .with_specular(0.1)
        .with_shininess(50.0)
        .with_reflective(0.6)
}

fn create_right() -> Object {
    Object::new_sphere()
        .scale(0.5, 0.5, 0.5)
        .translate(1.5, 0.5, -0.5)
        .with_pattern(
            Pattern::new_solid(Color::new(0.83921, 0.0, 0.1098))
                .scale(0.25, 0.25, 0.25)
                .rotate_x(PI / 3.0)
                .rotate_z(PI / 3.33)
                .rotate_y(PI / 1.76),
        )
        .with_diffuse(0.7)
        .with_specular(0.3)
}

fn create_left() -> Object {
    Object::new_sphere()
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -0.75)
        .with_pattern(
            Pattern::new_ring(Color::new(1.0, 0.8, 0.1), Color::white())
                .scale(0.5, 0.5, 0.5)
                .rotate_x(PI / 2.0)
                .rotate_y(PI / 4.0),
        )
        .with_diffuse(0.7)
        .with_specular(0.3)
}

fn main() -> std::io::Result<()> {
    let width: usize = 10000;
    let height: usize = 5000;
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

    world.render(&camera).save("./", "chapter11")
}
