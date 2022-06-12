use std::f64::consts::PI;

use raytracer::{
    draw::{color::Color, io::obj::ObjFileParser},
    math::{point::Point, transformation::Transformable, tuple::Tuple, vector::Vector},
    render::{
        camera::Camera,
        light::Light,
        lights::point_light::PointLight,
        material::{Material, Materialable},
        object::Object,
        pattern::Pattern,
        world::World,
    },
};

/**
 *
 * Triangles
 *
 */

fn floor() -> Object {
    let c_a = Color::new(0.9686274509803922, 0.6313725490196078, 0.7686274509803922);
    let c_b = Color::new(
        0.050980392156862744,
        0.09411764705882353,
        0.12941176470588237,
    );
    Object::new_plane().with_pattern(Pattern::new_checker(c_a, c_b))
}

fn light() -> Light {
    Light::Point(PointLight::new(
        Point::new(2.0, 10.0, -5.0),
        Color::new(0.9, 0.9, 0.9),
    ))
}

fn main() -> std::io::Result<()> {
    let width = 100;
    let height = 100;
    let fov = PI / 3.0;
    let camera = Camera::new(width, height, fov).view_transform(
        &Point::new(0.0, 5.5, -10.0),
        &Point::new(0.0, 2.0, 1.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    let cow = ObjFileParser::new_file("./assets/obj/pumpkin.obj")
        .build_with_material(Material::default().with_pattern(Pattern::new_noise(
            Color::black(),
            Color::white(),
            -0.1,
        )))
        .scale(0.1, 0.1, 0.1)
        .rotate_y(PI)
        .translate(0.0, 2.5, -2.0);

    let mut world = World::new();
    world.add_light(light());
    world.add_object(floor());
    world.add_object(cow);

    world.render(&camera).save("./", "chapter15")
}
