use std::f64::consts::PI;

/**
 *
 * Cubes!!
 *
 */
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
        Point::new(2.0, 10.0, -5.0),
        Color::new(0.9, 0.9, 0.9),
    ))
}

fn create_floor() -> Object {
    let light = Color::new(0.9, 0.9, 0.9);
    let dark = Color::new(0.15, 0.15, 0.15);
    Object::new_plane()
        .with_pattern(Pattern::new_checker(dark, light))
        .with_ambient(0.8)
        .with_diffuse(0.2)
        .with_specular(0.0)
        .with_shininess(900.0)
        .with_reflective(1.0)
}

fn gradient() -> Pattern {
    let orange = Color::new(0.9764705882352941, 0.6509803921568628, 0.12549019607843137);
    let pink = Color::new(0.8862745098039215, 0.7137254901960784, 0.8117647058823529);
    Pattern::new_gradient(orange, pink)
}

fn create_cube(u: f64, v: f64) -> Object {
    Object::new_cube()
        .rotate_y(PI / 4.0)
        .rotate_x(PI / 4.0)
        .scale(0.25, 0.25, 0.25)
        .translate(u, 1.25, 5.0 + v)
        .with_reflective(0.25)
        .with_shininess(600.0)
        .with_pattern(gradient())
}

fn main() -> std::io::Result<()> {
    let width: usize = 500;
    let height: usize = 250;
    let mut world = World::new();
    world.add_light(create_light());
    world.add_object(create_floor());

    let cube_count = 10;
    let count_half = cube_count as f64 / 2.0;

    for v in 0..cube_count {
        for u in 0..cube_count {
            let u_f = u as f64 - count_half;
            let v_f = v as f64;
            world.add_object(create_cube(u_f, v_f));
        }
    }

    let camera = Camera::new(width, height, 0.45).view_transform(
        &Point::new(-4.0, 7.0, -2.25),
        &Point::new(-0.5, 1.5, 5.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    world.render(&camera).save("./", "chapter12")
}
