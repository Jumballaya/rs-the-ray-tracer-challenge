use crate::{
    math::{matrix::Matrix, point::Point, ray::Ray, transformation::Transformable, vector::Vector},
    render::{
        intersections::Intersections,
        material::{Material, Materialable},
        shape::Shape,
        shapes::{
            cone::Cone, cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere,
            test_shape::TestShape,
        },
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    shape: Shape,
    material: Material,
    transformation: Matrix,
    inv_transformation: Matrix,
}

impl Object {
    pub fn new_sphere() -> Self {
        Object {
            shape: Shape::Sphere(Sphere::new()),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity().inverse(),
        }
    }

    pub fn new_plane() -> Self {
        Object {
            shape: Shape::Plane(Plane::new()),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity().inverse(),
        }
    }

    pub fn new_cube() -> Self {
        Object {
            shape: Shape::Cube(Cube::new()),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity().inverse(),
        }
    }

    pub fn new_cone(min: f64, max: f64, closed: bool) -> Self {
        Object {
            shape: Shape::Cone(Cone::new().with_closed(closed).with_max(max).with_min(min)),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity().inverse(),
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
            inv_transformation: Matrix::identity().inverse(),
        }
    }

    pub fn new_test_shape() -> Self {
        Object {
            shape: Shape::TestShape(TestShape::new()),
            material: Material::default(),
            transformation: Matrix::identity(),
            inv_transformation: Matrix::identity().inverse(),
        }
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let local_point = self.inv_transformation * *world_point;
        let local_normal: Vector = self.shape.normal_at(&local_point);
        let world_normal = self.inv_transformation.transpose() * local_normal;
        world_normal.normalize()
    }

    pub fn intersect<'a>(&'a self, ray: &Ray, intersections: &mut Intersections<'a>) {
        let local_ray = ray.with_transform(self.inv_transformation);
        self.shape.intersect(&local_ray, self, intersections);
    }

    pub fn get_transform_inv(&self) -> Matrix {
        self.inv_transformation
    }
}

impl Transformable for Object {
    fn get_transform(&self) -> Matrix {
        self.transformation
    }

    fn with_transform(self, tform: Matrix) -> Self {
        let new_tform = tform * self.transformation;
        Object {
            shape: self.shape,
            material: self.material,
            transformation: new_tform,
            inv_transformation: new_tform.inverse(),
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
        let got = obj.normal_at(&Point::new(0.0, 1.70711, -0.70711));
        let want = Vector::new(0.0, 0.70711, -0.70711);
        assert_eq!(got, want);
    }

    #[test]
    fn compute_normal_on_transformed_shape() {
        let obj = Object::new_sphere().rotate_z(PI / 5.0).scale(1.0, 0.5, 1.0);
        let root_2_2 = (2.0 as f64).sqrt() / 2.0;
        let world_point = Point::new(0.0, root_2_2, -root_2_2);
        let got = obj.normal_at(&world_point);
        let want = Vector::new(0.0, 0.97014, -0.24254);
        assert_eq!(got, want);
    }
}
