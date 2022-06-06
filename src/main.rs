mod draw;
mod math;
mod render;

use std::f64::consts::PI;

use draw::color::*;
use math::{matrix::Transformation, tuple::*};
use render::{
    camera::Camera,
    light::{point::PointLight, Light},
    material::Material,
    object::{plane::Plane, sphere::Sphere, Object},
    world::World,
};

fn create_floor() -> Object {
    let mut floor = Plane::new();
    let material = Material::new(Color::new(1.0, 0.9, 0.9), 0.1, 0.9, 0.0, 200.0);
    floor.set_material(material);
    floor.set_transform(Transformation::Chain(vec![
        Transformation::RotateX(PI * 0.8),
        Transformation::Translate(0.0, -0.25, 0.0),
    ]));
    Object::Plane(floor)
}

fn create_middle() -> Object {
    let mut middle = Sphere::new();
    let tform = Transformation::Translate(-0.5, 1.0, 0.5);
    let mut material = Material::default();
    material.color = Color::new(0.1, 1.0, 0.5);
    material.diffuse = 0.7;
    material.specular = 0.3;
    middle.set_material(material);
    middle.set_transform(tform);
    Object::Sphere(middle)
}

fn create_left() -> Object {
    let mut left = Sphere::new();
    let tform = Transformation::Chain(vec![
        Transformation::Scale(0.33, 0.33, 0.33),
        Transformation::Translate(-1.5, 0.33, -0.75),
    ]);
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.8, 0.1);
    material.diffuse = 0.7;
    material.specular = 0.3;
    left.set_material(material);
    left.set_transform(tform);
    Object::Sphere(left)
}

fn create_right() -> Object {
    let mut right = Sphere::new();
    let tform = Transformation::Chain(vec![
        Transformation::Scale(0.5, 0.5, 0.5),
        Transformation::Translate(1.5, 0.5, -0.5),
    ]);
    let mut material = Material::default();
    material.color = Color::new(1.0, 1.0, 0.5);
    material.diffuse = 0.7;
    material.specular = 0.3;
    right.set_material(material);
    right.set_transform(tform);
    Object::Sphere(right)
}

fn create_light() -> Light {
    let light = PointLight::new(
        Tuple::new_point(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    );
    Light::Point(light)
}

fn main() -> std::io::Result<()> {
    let width: usize = 50;
    let height: usize = 25;
    let field_of_view = PI / 3.0;

    let floor = create_floor();
    let left = create_left();
    let middle = create_middle();
    let right = create_right();
    let light = create_light();

    let mut world = World::new();
    world.add_object(floor);
    world.add_object(middle);
    world.add_object(left);
    world.add_object(right);
    world.add_light(light);

    let mut camera = Camera::new(width, height, field_of_view);
    camera.set_transform(Transformation::View(
        Tuple::new_point(0.0, 1.5, -5.0),
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(0.0, 1.0, 0.0),
    ));

    let canvas = world.render(width, height, &camera);
    canvas.save("./", "plane")
}
