use std::f64::consts::PI;

use raytracer::{
    draw::color::Color,
    math::{point::Point, transformation::Transformable, tuple::Tuple, vector::Vector},
    render::{
        camera::Camera,
        light::Light,
        lights::point_light::PointLight,
        material::{Material, Materialable, REFRACTION_GLASS},
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

fn glass() -> Material {
    Material::default()
        .with_ambient(0.0)
        .with_diffuse(0.0)
        .with_specular(0.9)
        .with_shininess(300.0)
        .with_reflective(0.9)
        .with_transparency(0.9)
        .with_refractive_index(REFRACTION_GLASS)
}

fn tri() -> Object {
    Object::new_tri(
        Point::new(0.0, 0.5, 0.5),
        Point::new(-0.5, 0.0, 0.0),
        Point::new(0.5, 0.0, 0.0),
    )
    .with_pattern(Pattern::new_noise(Color::red(), Color::blue(), 0.0).scale(0.05, 0.05, 0.05))
}

fn pyramid_half() -> Object {
    let t1 = tri().rotate_y(-PI / 2.0).translate(0.5, 0.0, 0.5);
    let t2 = tri();
    Object::new_group(vec![t1, t2])
}

fn pyramid() -> Object {
    let s1 = pyramid_half().rotate_y(PI).translate(0.0, 0.0, 1.0);
    let s2 = pyramid_half();
    Object::new_group(vec![s1, s2])
}

fn light() -> Light {
    Light::Point(PointLight::new(
        Point::new(2.0, 10.0, -5.0),
        Color::new(0.9, 0.9, 0.9),
    ))
}

fn main() -> std::io::Result<()> {
    let width = 1000;
    let height = 500;
    let fov = PI / 3.0;
    let camera = Camera::new(width, height, fov).view_transform(
        &Point::new(0.0, 1.0, -4.0),
        &Point::new(0.0, 0.0, 1.0),
        &Vector::new(0.0, 1.0, 0.0),
    );

    let mut world = World::new();
    world.add_light(light());
    world.add_object(floor());
    world.add_object(pyramid().scale(1.0, 2.0, 1.0).rotate_y(PI / 6.0));

    world.render(&camera).save("./", "chapter15")
}
