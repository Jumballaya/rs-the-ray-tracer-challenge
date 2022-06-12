use crate::{
    draw::io::obj::ObjFileParser,
    math::{matrix::Matrix, point::Point, ray::Ray, transformation::Transformable, vector::Vector},
    render::{
        intersections::Intersections,
        material::{Material, Materialable},
        shape::Shape,
        shapes::{
            cone::Cone, cube::Cube, cylinder::Cylinder, group::GroupTree, plane::Plane,
            sphere::Sphere, test_shape::TestShape, triangle::Triangle,
        },
    },
};

use super::{intersections::Intersection, shapes::smooth_triangle::SmoothTriangle};

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    shape: Shape,
    material: Material,
    transformation: Matrix,
    inv_transformation: Matrix,
    inv_transpose_transformation: Matrix,
}

impl Object {
    pub fn new_sphere() -> Self {
        Object {
            shape: Shape::Sphere(Sphere::new()),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity(),
            inv_transpose_transformation: Matrix::identity(),
        }
    }

    pub fn new_plane() -> Self {
        Object {
            shape: Shape::Plane(Plane::new()),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity(),
            inv_transpose_transformation: Matrix::identity(),
        }
    }

    pub fn new_cube() -> Self {
        Object {
            shape: Shape::Cube(Cube::new()),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity(),
            inv_transpose_transformation: Matrix::identity(),
        }
    }

    pub fn new_cone(min: f64, max: f64, closed: bool) -> Self {
        Object {
            shape: Shape::Cone(Cone::new().with_closed(closed).with_max(max).with_min(min)),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity(),
            inv_transpose_transformation: Matrix::identity(),
        }
    }

    pub fn new_cylinder(min: f64, max: f64, closed: bool) -> Self {
        Object {
            shape: Shape::Cylinder(
                Cylinder::new()
                    .with_closed(closed)
                    .with_max(max)
                    .with_min(min),
            ),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity(),
            inv_transpose_transformation: Matrix::identity(),
        }
    }

    pub fn new_tri(p1: Point, p2: Point, p3: Point) -> Self {
        Object {
            shape: Shape::Triangle(Triangle::new(p1, p2, p3)),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity(),
            inv_transpose_transformation: Matrix::identity(),
        }
    }

    pub fn new_smooth_tri(
        p1: Point,
        p2: Point,
        p3: Point,
        n1: Vector,
        n2: Vector,
        n3: Vector,
    ) -> Self {
        Object {
            shape: Shape::SmoothTriangle(SmoothTriangle::new(p1, p2, p3, n1, n2, n3)),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity(),
            inv_transpose_transformation: Matrix::identity(),
        }
    }

    pub fn new_model(path: &str) -> Object {
        ObjFileParser::new_file(path).build()
    }

    pub fn new_group(children: Vec<Object>) -> Self {
        let children_group_builders = children
            .iter()
            .filter_map(|child| match child.get_shape() {
                Shape::Group(g) => {
                    if g.children().is_empty() {
                        None
                    } else {
                        Some(GroupTree::from_object(child))
                    }
                }

                _ => Some(GroupTree::from_object(child)),
            })
            .collect();
        let group_builder = GroupTree::Node(Object::new_test_shape(), children_group_builders);
        let object = group_builder.build();
        object
    }

    pub fn new_test_shape() -> Self {
        Object {
            shape: Shape::TestShape(TestShape::new()),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity(),
            inv_transpose_transformation: Matrix::identity(),
        }
    }

    pub fn new_empty_shape() -> Self {
        Object {
            shape: Shape::TestShape(TestShape::new()),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity(),
            inv_transpose_transformation: Matrix::identity(),
        }
    }

    pub fn world_to_object(&self, p: &Point) -> Point {
        self.inv_transformation * *p
    }

    pub fn children(&self) -> Option<&Vec<Object>> {
        match &self.shape {
            Shape::Group(g) => Some(g.children()),
            _ => None,
        }
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    pub fn normal_at(&self, world_point: &Point, int: &Intersection) -> Vector {
        let local_point = self.world_to_object(world_point);
        let local_normal = self.shape.normal_at(&local_point, int);

        self.normal_to_world(&local_normal)
    }

    pub fn normal_to_world(&self, normal: &Vector) -> Vector {
        (self.inv_transpose_transformation * *normal).normalize()
    }

    pub fn intersect<'a>(&'a self, ray: &Ray, intersections: &mut Intersections<'a>) {
        if self.shape.skip_world_to_local() {
            self.shape.intersect(ray, &self, intersections);
        } else {
            let local_ray = ray.with_transform(self.inv_transformation);
            self.shape.intersect(&local_ray, self, intersections);
        }
    }

    pub fn get_transform_inv(&self) -> Matrix {
        self.inv_transformation
    }

    pub fn with_shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }
}

impl Transformable for Object {
    fn get_transform(&self) -> Matrix {
        self.transformation
    }

    fn with_transform(self, new_transformation: Matrix) -> Self {
        match self.get_shape() {
            Shape::Group(g) => {
                let children_group_builders = g
                    .children()
                    .iter()
                    .map(|child| GroupTree::from_object(child))
                    .collect();

                let group_builder = GroupTree::Node(
                    Object::new_test_shape().with_transform(new_transformation),
                    children_group_builders,
                );

                group_builder.build()
            }
            _ => {
                let new_transformation = new_transformation * self.transformation;
                Object {
                    transformation: new_transformation,
                    inv_transformation: new_transformation.inverse(),
                    inv_transpose_transformation: new_transformation.inverse().transpose(),
                    ..self
                }
            }
        }
    }
}

impl Materialable for Object {
    fn with_material(self, material: Material) -> Self {
        Object {
            shape: self.shape,
            material,
            transformation: self.transformation,
            inv_transformation: self.inv_transformation,
            inv_transpose_transformation: self.inv_transpose_transformation,
        }
    }

    fn get_material(&self) -> Material {
        self.material
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use super::*;
    use crate::draw::color::Color;
    use crate::math::transformation::{translate, Transformable};
    use crate::math::tuple::Tuple;
    use crate::render::object::Object;
    use crate::render::pattern::Pattern;

    #[test]
    fn default_object_tfrom_is_ident() {
        let obj = Object::new_test_shape();
        let m = Matrix::identity();
        assert_eq!(obj.get_transform(), m);
    }

    #[test]
    fn change_object_tform() {
        let obj = Object::new_test_shape().translate(2.0, 3.0, 4.0);
        assert_eq!(obj.get_transform(), translate(2.0, 3.0, 4.0));
    }

    #[test]
    fn has_default_material() {
        let obj = Object::new_test_shape();
        assert_eq!(obj.get_material(), Material::default());
    }

    #[test]
    fn may_be_assigned_a_material() {
        let obj = Object::new_test_shape()
            .with_pattern(Pattern::new_solid(Color::black()))
            .with_ambient(1.0)
            .with_diffuse(0.9)
            .with_specular(0.9)
            .with_shininess(200.0);
        let want = Material::new(
            Pattern::new_solid(Color::black()),
            1.0,
            0.9,
            0.9,
            200.0,
            0.0,
            0.0,
            1.0,
        );
        let got = obj.get_material();
        assert_eq!(got, want);
    }

    #[test]
    fn intersecting_scaled_shape_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let obj = Object::new_test_shape().scale(2.0, 2.0, 2.0);
        let mut ints = Intersections::new();
        _ = obj.intersect(&r, &mut ints);
        let want_origin = Point::new(0.0, 0.0, -2.5);
        let want_direction = Vector::new(0.0, 0.0, 0.5);

        let ts = match obj.get_shape() {
            Shape::TestShape(ts) => ts,
            _ => panic!(),
        };

        assert_eq!(ts.get_saved_ray().unwrap().origin, want_origin);
        assert_eq!(ts.get_saved_ray().unwrap().direction, want_direction);
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let obj = Object::new_test_shape().translate(5.0, 0.0, 0.0);
        let mut ints = Intersections::new();
        _ = obj.intersect(&r, &mut ints);
        let want_origin = Point::new(-5.0, 0.0, -5.0);
        let want_direction = Vector::new(0.0, 0.0, 1.0);

        let ts = match obj.get_shape() {
            Shape::TestShape(ts) => ts,
            _ => panic!(),
        };

        assert_eq!(ts.get_saved_ray().unwrap().origin, want_origin);
        assert_eq!(ts.get_saved_ray().unwrap().direction, want_direction);
    }

    #[test]
    fn normal_on_translated_shape() {
        let obj = Object::new_sphere().translate(0.0, 1.0, 0.0);
        let got = obj.normal_at(
            &Point::new(0.0, 1.70711, -0.70711),
            &Intersection::new(0.0, &obj),
        );
        let want = Vector::new(0.0, 0.70711, -0.70711);
        assert_eq!(got, want);
    }

    #[test]
    fn compute_normal_on_transformed_shape() {
        let obj = Object::new_sphere().rotate_z(PI / 5.0).scale(1.0, 0.5, 1.0);
        let root_2_2 = (2.0 as f64).sqrt() / 2.0;
        let world_point = Point::new(0.0, root_2_2, -root_2_2);
        let got = obj.normal_at(&world_point, &Intersection::new(0.0, &obj));
        let want = Vector::new(0.0, 0.97014, -0.24254);
        assert_eq!(got, want);
    }

    #[test]
    fn converting_a_point_from_world_to_obj_space() {
        let s = Object::new_sphere().translate(5.0, 0.0, 0.0);
        let g2 = Object::new_group(vec![s]).scale(2.0, 2.0, 2.0);
        let g1 = Object::new_group(vec![g2]).rotate_y(PI / 2.0);

        let group_s = g1.children().unwrap()[0].children().unwrap()[0].clone();

        let got = group_s.world_to_object(&Point::new(-2.0, 0.0, -10.0));
        let want = Point::new(0.0, 0.0, -1.0);
        assert_eq!(got, want);
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let root_3_3 = f64::sqrt(3.0) / 3.0;
        let s = Object::new_sphere().translate(5.0, 0.0, 0.0);
        let g2 = Object::new_group(vec![s]).scale(1.0, 2.0, 3.0);
        let g1 = Object::new_group(vec![g2]).rotate_y(PI / 2.0);

        let group_s = g1.children().unwrap()[0].children().unwrap()[0].clone();

        let got = group_s.normal_to_world(&Vector::new(root_3_3, root_3_3, root_3_3));
        let want = Vector::new(0.2857, 0.4286, -0.8571);
        assert_eq!(got, want);
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let s = Object::new_sphere().translate(5.0, 0.0, 0.0);
        let g2 = Object::new_group(vec![s]).scale(1.0, 2.0, 3.0);
        let g1 = Object::new_group(vec![g2]).rotate_y(PI / 2.0);

        let group_s = g1.children().unwrap()[0].children().unwrap()[0].clone();

        let got = group_s.normal_at(
            &Point::new(1.7321, 1.1547, -5.5774),
            &Intersection::new(0.0, &group_s),
        );
        let want = Vector::new(0.285703, 0.42854, -0.857160);
        assert_eq!(got, want);
    }
}
