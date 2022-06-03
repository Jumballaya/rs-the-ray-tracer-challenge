mod draw;
mod math;
mod render;

use std::f64::consts::PI;

use draw::color::*;
use math::{matrix::Transformation, tuple::*};
use render::{
    light::{point::PointLight, Light},
    material::Material,
    object::{sphere::Sphere, Object},
    world::World,
};

fn create_floor() -> Object {
    let mut floor = Sphere::new();
    let tform = Transformation::Scale(10.0, 0.01, 10.0);
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.9, 0.9);
    material.specular = 0.0;
    floor.set_material(material);
    floor.set_transform(tform);
    Object::Sphere(floor)
}

fn create_left_wall() -> Object {
    let mut left_wall = Sphere::new();
    let tform = Transformation::Chain(vec![
        Transformation::Scale(10.0, 0.01, 10.0),
        Transformation::RotateX(PI / 2.0),
        Transformation::RotateY(-PI / 4.0),
        Transformation::Translate(0.0, 0.0, 5.0),
    ]);
    left_wall.set_transform(tform);
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.9, 0.9);
    material.specular = 0.0;
    left_wall.set_material(material);
    Object::Sphere(left_wall)
}

fn create_right_wall() -> Object {
    let mut right_wall = Sphere::new();
    let tform = Transformation::Chain(vec![
        Transformation::Scale(10.0, 0.01, 10.0),
        Transformation::RotateX(PI / 2.0),
        Transformation::RotateY(PI / 4.0),
        Transformation::Translate(0.0, 0.0, 5.0),
    ]);
    right_wall.set_transform(tform);
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.9, 0.9);
    material.specular = 0.0;
    right_wall.set_material(material);
    Object::Sphere(right_wall)
}

fn create_middle() -> Object {
    let mut middle = Sphere::new();
    let tform = Transformation::Translate(-0.5, 1.0, 0.5);
    let mut material = Material::default();
    material.color = Color::new(0.1, 1.0, 0.5);
    material.specular = 0.3;
    material.diffuse = 0.7;
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
    left.set_transform(tform);
    let mut material = Material::default();
    material.color = Color::new(0.5, 1.0, 0.1);
    material.specular = 0.3;
    material.diffuse = 0.7;
    left.set_material(material);
    Object::Sphere(left)
}

fn create_right() -> Object {
    let mut right = Sphere::new();
    let tform = Transformation::Chain(vec![
        Transformation::Scale(0.5, 0.5, 0.5),
        Transformation::Translate(1.5, 0.5, -0.5),
    ]);
    right.set_transform(tform);
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.8, 0.1);
    material.specular = 0.3;
    material.diffuse = 0.7;
    right.set_material(material);
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
    let width: usize = 100;
    let height: usize = 100;
    let field_of_view = PI / 3.0;

    let floor = create_floor();
    let left_wall = create_left_wall();
    let right_wall = create_right_wall();
    let left = create_left();
    let middle = create_middle();
    let right = create_right();
    let light = create_light();

    let mut world = World::new(width, height, field_of_view);
    world.add_object(floor);
    world.add_object(left_wall);
    world.add_object(right_wall);
    world.add_object(left);
    world.add_object(middle);
    world.add_object(right);
    world.add_light(light);

    world.camera.add_transform(Transformation::View(
        Tuple::new_point(0.0, 1.5, -5.0),
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(0.0, 1.0, 0.0),
    ));

    let canvas = world.render();
    canvas.save("./", "circle")
}
