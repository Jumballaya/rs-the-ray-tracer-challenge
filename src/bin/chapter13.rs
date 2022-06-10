use raytracer::{
    draw::color::Color,
    math::{point::Point, transformation::Transformable, tuple::Tuple, vector::Vector},
    render::{
        camera::Camera, light::Light, lights::point_light::PointLight, material::Materialable,
        object::Object, pattern::Pattern, world::World,
    },
};
use std::f64::consts::PI;
/**
 *
 * Cylinders and Cones
 *
 */

fn create_light() -> Light {
    Light::Point(PointLight::new(
        Point::new(1.0, 6.9, -4.9),
        Color::new(0.9, 0.9, 0.9),
    ))
}

fn create_floor() -> Object {
    let light = Color::new(0.75, 0.75, 0.75);
    let dark = Color::new(0.15, 0.15, 0.15);
    Object::new_plane()
        .with_pattern(
            Pattern::new_checker(dark, light)
                .scale(0.25, 0.25, 0.25)
                .rotate_y(0.3),
        )
        .with_ambient(0.2)
        .with_diffuse(0.9)
        .with_specular(0.0)
}

fn create_c1() -> Object {
    Object::new_cylinder(0.0, 0.75, true)
        .scale(0.5, 1.0, 0.5)
        .translate(-1.0, 0.0, 1.0)
        .with_pattern(Pattern::new_solid(Color::new(0.0, 0.0, 0.6)))
        .with_diffuse(0.1)
        .with_specular(0.9)
        .with_shininess(300.0)
        .with_reflective(0.9)
}

fn create_c2() -> Object {
    Object::new_cylinder(0.0, 0.2, false)
        .scale(0.8, 1.0, 0.8)
        .translate(1.0, 0.0, 0.0)
        .with_pattern(Pattern::new_solid(Color::new(1.0, 1.0, 0.3)))
        .with_ambient(0.1)
        .with_diffuse(0.8)
        .with_specular(0.9)
        .with_shininess(300.0)
}

fn create_c3() -> Object {
    Object::new_cylinder(0.0, 0.3, false)
        .scale(0.6, 1.0, 0.6)
        .translate(1.0, 0.0, 0.0)
        .with_pattern(Pattern::new_solid(Color::new(1.0, 0.9, 0.4)))
        .with_ambient(0.1)
        .with_diffuse(0.8)
        .with_specular(0.9)
        .with_shininess(300.0)
}

fn create_c4() -> Object {
    Object::new_cylinder(0.0, 0.4, false)
        .scale(0.4, 1.0, 0.4)
        .translate(1.0, 0.0, 0.0)
        .with_pattern(Pattern::new_solid(Color::new(1.0, 0.8, 0.5)))
        .with_ambient(0.1)
        .with_diffuse(0.8)
        .with_specular(0.9)
        .with_shininess(300.0)
}

fn create_c5() -> Object {
    Object::new_cylinder(0.0, 0.5, true)
        .scale(0.2, 1.0, 0.2)
        .translate(1.0, 0.0, 0.0)
        .with_pattern(Pattern::new_solid(Color::new(1.0, 0.7, 0.6)))
        .with_ambient(0.1)
        .with_diffuse(0.8)
        .with_specular(0.9)
        .with_shininess(300.0)
}

fn create_c6() -> Object {
    Object::new_cylinder(0.0, 0.3, true)
        .scale(0.05, 1.0, 0.05)
        .translate(0.0, 0.0, -0.75)
        .with_pattern(Pattern::new_solid(Color::new(1.0, 0.0, 0.0)))
        .with_ambient(0.1)
        .with_diffuse(0.9)
        .with_specular(0.9)
        .with_shininess(300.0)
}

fn create_c7() -> Object {
    Object::new_cylinder(0.0, 0.3, true)
        .scale(0.05, 1.0, 0.05)
        .translate(0.0, 0.0, 1.5)
        .rotate_y(-0.15)
        .translate(0.0, 0.0, -2.25)
        .with_pattern(Pattern::new_solid(Color::new(1.0, 1.0, 0.0)))
        .with_ambient(0.1)
        .with_diffuse(0.9)
        .with_specular(0.9)
        .with_shininess(300.0)
}

fn create_c8() -> Object {
    Object::new_cylinder(0.0, 0.3, true)
        .scale(0.05, 1.0, 0.05)
        .translate(0.0, 0.0, 1.5)
        .rotate_y(-0.3)
        .translate(0.0, 0.0, -2.25)
        .with_pattern(Pattern::new_solid(Color::new(0.0, 1.0, 0.0)))
        .with_ambient(0.1)
        .with_diffuse(0.9)
        .with_specular(0.9)
        .with_shininess(300.0)
}

fn create_c9() -> Object {
    Object::new_cylinder(0.0, 0.3, true)
        .scale(0.05, 1.0, 0.05)
        .translate(0.0, 0.0, 1.5)
        .rotate_y(-0.45)
        .translate(0.0, 0.0, -2.25)
        .with_pattern(Pattern::new_solid(Color::new(0.0, 1.0, 1.0)))
        .with_ambient(0.1)
        .with_diffuse(0.9)
        .with_specular(0.9)
        .with_shininess(300.0)
}

fn create_c10() -> Object {
    Object::new_cylinder(0.0001, 0.5, true)
        .scale(0.33, 1.0, 0.33)
        .translate(0.0, 0.0, -1.5)
        .with_pattern(Pattern::new_solid(Color::new(0.25, 0.0, 0.0)))
        .with_diffuse(0.1)
        .with_specular(0.9)
        .with_shininess(300.0)
        .with_reflective(0.9)
        .with_transparency(0.9)
        .with_refractive_index(1.5)
}

fn create_cone() -> Object {
    Object::new_cone(0.0, 1.0, false)
        .rotate_x(-PI)
        .translate(-3.5, 1.0, -1.5)
        .scale(0.25, 0.25, 0.25)
        .with_pattern(Pattern::new_gradient(Color::red(), Color::green()))
}

fn main() -> std::io::Result<()> {
    let width: usize = 1000;
    let height: usize = 500;
    let mut world = World::new();
    world.add_light(create_light());
    world.add_object(create_floor());
    world.add_object(create_c1());
    world.add_object(create_c2());
    world.add_object(create_c3());
    world.add_object(create_c4());
    world.add_object(create_c5());
    world.add_object(create_c6());
    world.add_object(create_c7());
    world.add_object(create_c8());
    world.add_object(create_c9());
    world.add_object(create_c10());
    world.add_object(create_cone());

    let camera = Camera::new(width, height, 0.314).view_transform(
        &Point::new(8.0, 3.5, -9.0),
        &Point::new(0.0, 0.3, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    world.render(&camera).save("./", "chapter13")
}
