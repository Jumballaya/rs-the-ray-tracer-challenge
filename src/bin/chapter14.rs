use std::f64::consts::PI;

use raytracer::{
    draw::color::Color,
    math::{point::Point, transformation::Transformable, tuple::Tuple, vector::Vector},
    render::{
        camera::Camera, light::Light, lights::point_light::PointLight, material::Materialable,
        object::Object, pattern::Pattern, world::World,
    },
};

/**
 *
 * Groups
 *
 */

fn floor() -> Object {
    let c_a = Color::new(0.9686274509803922, 0.6313725490196078, 0.7686274509803922);
    let c_b = Color::new(
        0.050980392156862744,
        0.09411764705882353,
        0.12941176470588237,
    );
    Object::new_plane()
        .translate(0.0, -5.0, 0.0)
        .with_pattern(Pattern::new_checker(c_a, c_b))
        .with_ambient(0.1)
        .with_diffuse(0.7)
        .with_specular(0.9)
        .with_shininess(900.0)
        .with_reflective(0.9)
}

fn light() -> Light {
    Light::Point(PointLight::new(
        Point::new(2.0, 10.0, -5.0),
        Color::new(0.9, 0.9, 0.9),
    ))
}

fn hex_corner() -> Object {
    let c_a = Color::new(0.9490196078431372, 0.39215686274509803, 0.18823529411764706);
    let c_b = Color::new(0.7490196078431373, 0.8431372549019608, 0.7098039215686275);
    Object::new_sphere()
        .scale(0.25, 0.25, 0.25)
        .translate(0.0, 0.0, -1.0)
        .with_pattern(Pattern::new_noise(c_a, c_b, 0.1).scale(0.25, 0.25, 0.25))
}

fn hex_edge() -> Object {
    let c_b = Color::new(0.9490196078431372, 0.39215686274509803, 0.18823529411764706);
    let c_a = Color::new(0.7490196078431373, 0.8431372549019608, 0.7098039215686275);
    Object::new_cylinder(0.0, 1.0, false)
        .scale(0.25, 1.0, 0.25)
        .rotate_z(-PI / 2.0)
        .rotate_y(-PI / 6.0)
        .translate(0.0, 0.0, -1.0)
        .with_pattern(Pattern::new_noise(c_a, c_b, 0.1).scale(0.25, 0.25, 0.25))
}

fn hex_side() -> Object {
    Object::new_group(vec![hex_corner(), hex_edge()])
}

fn hexagon() -> Object {
    let mut children = Vec::<Object>::new();
    for n in 0..6 {
        children.push(hex_side().rotate_y((n as f64) * PI / 3.0));
    }
    Object::new_group(children)
}

fn chain() -> Object {
    Object::new_group(vec![
        hexagon().translate(1.0, 0.0, 0.5).rotate_x(PI / 1.5),
        hexagon().rotate_z(-PI / 8.0),
        hexagon().translate(-1.3, 0.0, 0.5).rotate_x(-PI / 1.5),
    ])
}

fn main() -> std::io::Result<()> {
    let width = 2000;
    let height = 1000;
    let fov = PI / 3.0;
    let camera = Camera::new(width, height, fov).view_transform(
        &Point::new(2.0, 3.0, -10.0),
        &Point::new(0.0, -1.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    let mut world = World::new();
    world.add_light(light());
    world.add_object(chain());
    world.add_object(floor());

    world.render(&camera).save("./", "chapter14")
}
