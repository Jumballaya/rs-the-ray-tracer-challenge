/**
 *
 * Reflection/Refraction/Transparency
 *
 */
use raytracer::{
    draw::color::Color,
    math::{point::Point, transformation::Transformable, tuple::Tuple, vector::Vector},
    render::{
        camera::Camera,
        light::Light,
        lights::point_light::PointLight,
        material::{Materialable, REFRACTION_AIR, REFRACTION_GLASS},
        object::Object,
        pattern::Pattern,
        world::World,
    },
};

fn create_light() -> Light {
    Light::Point(PointLight::new(
        Point::new(2.0, 10.0, -5.0),
        Color::new(0.9, 0.9, 0.9),
    ))
}

fn create_wall() -> Object {
    let light = Color::new(0.9, 0.9, 0.9);
    let dark = Color::new(0.15, 0.15, 0.15);
    Object::new_plane()
        .rotate_x(1.5708)
        .translate(0.0, 0.0, 10.0)
        .with_pattern(Pattern::new_checker(dark, light))
        .with_ambient(0.8)
        .with_diffuse(0.2)
        .with_specular(0.0)
}

fn create_outer() -> Object {
    Object::new_sphere()
        .with_pattern(Pattern::new_solid(Color::white()))
        .with_ambient(0.0)
        .with_diffuse(0.0)
        .with_specular(0.9)
        .with_shininess(300.0)
        .with_reflective(0.9)
        .with_transparency(0.9)
        .with_refractive_index(REFRACTION_GLASS)
}

fn create_center() -> Object {
    Object::new_sphere()
        .scale(0.75, 0.75, 0.75)
        .with_pattern(Pattern::new_solid(Color::white()))
        .with_ambient(0.0)
        .with_diffuse(0.0)
        .with_specular(0.9)
        .with_shininess(300.0)
        .with_reflective(0.9)
        .with_transparency(0.9)
        .with_refractive_index(REFRACTION_AIR)
}

fn main() -> std::io::Result<()> {
    let width: usize = 1000;
    let height: usize = 1000;
    let mut world = World::new();
    world.add_light(create_light());
    world.add_object(create_wall());
    world.add_object(create_outer());
    world.add_object(create_center());

    let camera = Camera::new(width, height, 0.45).view_transform(
        &Point::new(0.0, 0.0, -5.0),
        &Point::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    world.render(&camera).save("./", "chapter11")
}
