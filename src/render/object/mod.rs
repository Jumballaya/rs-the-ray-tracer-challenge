use self::{plane::Plane, sphere::Sphere};
use super::{intersection::Intersection, material::Material};
use crate::math::{
    matrix::{Matrix, Transformation},
    ray::Ray,
    tuple::{Tuple, TupleType},
};
use std::sync::atomic::AtomicUsize;

pub mod plane;
pub mod sphere;

#[derive(Clone)]
struct TestShape {
    material: Material,
    transform: Transformation,
    pub saved_ray: Ray,
}

impl TestShape {
    pub fn new() -> Self {
        Self {
            transform: Transformation::None,
            material: Material::default(),
            saved_ray: Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
        }
    }

    pub fn get_id(&self) -> usize {
        0
    }

    pub fn get_type(&self) -> ObjectType {
        ObjectType::Test
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn set_transform(&mut self, tform: Transformation) {
        self.transform = tform;
    }

    pub fn get_transform(&self) -> Matrix {
        Matrix::transform(&self.transform)
    }

    pub fn mut_intersect(&mut self, r: &Ray) -> Vec<Intersection> {
        self.saved_ray = r.clone();
        vec![]
    }

    pub fn normal_at(&self, _: &Tuple) -> Tuple {
        Tuple::new_vector(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectType {
    Sphere,
    Plane,
    Test,
}

static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
    Test(TestShape),
}

impl Object {
    pub fn get_id(&self) -> usize {
        match self {
            Self::Plane(p) => p.get_id(),
            Self::Sphere(s) => s.get_id(),
            Self::Test(t) => t.get_id(),
        }
    }

    pub fn get_type(&self) -> ObjectType {
        match self {
            Self::Plane(p) => p.get_type(),
            Self::Sphere(s) => s.get_type(),
            Self::Test(t) => t.get_type(),
        }
    }

    pub fn get_material(&self) -> &Material {
        match self {
            Self::Plane(p) => p.get_material(),
            Self::Sphere(s) => s.get_material(),
            Self::Test(t) => t.get_material(),
        }
    }

    pub fn set_material(&mut self, mat: Material) {
        match self {
            Self::Plane(p) => p.set_material(mat),
            Self::Sphere(s) => s.set_material(mat),
            Self::Test(t) => t.set_material(mat),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(&self.get_transform().inverse());
        match self {
            Self::Plane(p) => p.intersect(&local_ray),
            Self::Sphere(s) => s.intersect(&local_ray),
            Self::Test(_) => vec![],
        }
    }

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let local_point = self.get_transform().inverse() * world_point;
        let local_normal = match self {
            Self::Plane(p) => {
                return p.normal_at(&local_point);
            }
            Self::Sphere(s) => s.normal_at(&local_point),
            Self::Test(t) => t.normal_at(&local_point),
        };
        let mut world_normal = self.get_transform().inverse().transpose() * local_normal;
        world_normal.w = 0.0;
        world_normal.tp = TupleType::Vector;
        world_normal.normalize()
    }

    pub fn set_transform(&mut self, tform: Transformation) {
        match self {
            Self::Plane(p) => p.set_transform(tform),
            Self::Sphere(s) => s.set_transform(tform),
            Self::Test(t) => t.set_transform(tform),
        }
    }

    pub fn get_transform(&self) -> Matrix {
        match self {
            Self::Plane(p) => p.get_transform(),
            Self::Sphere(s) => s.get_transform(),
            Self::Test(t) => t.get_transform(),
        }
    }

    fn mut_intersect(&mut self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(&self.get_transform().inverse());
        match self {
            Self::Plane(p) => p.intersect(&local_ray),
            Self::Sphere(s) => s.intersect(&local_ray),
            Self::Test(t) => t.mut_intersect(&local_ray),
        }
    }

    fn get_saved_ray(&self) -> Ray {
        match self {
            Self::Plane(_) => Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
            Self::Sphere(_) => Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
            Self::Test(t) => t.saved_ray,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type() && self.get_id() == other.get_id()
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::{
        math::{
            matrix::{Matrix, Transformation},
            ray::Ray,
            tuple::Tuple,
        },
        render::material::Material,
        render::object::{Object, Sphere},
    };

    use super::TestShape;

    #[test]
    fn object_can_get_default_tform() {
        let s = Sphere::new();
        let obj = Object::Sphere(s);

        assert_eq!(obj.get_transform(), Matrix::identity_matrix(4));
    }

    #[test]
    fn object_can_be_assigned_tform() {
        let s = Sphere::new();
        let mut obj = Object::Sphere(s);
        let tform = Transformation::Translate(2.0, 3.0, 4.0);
        obj.set_transform(tform);

        assert_eq!(
            obj.get_transform(),
            Matrix::transform(&Transformation::Translate(2.0, 3.0, 4.0))
        );
    }

    #[test]
    fn object_can_get_defualt_material() {
        let s = Sphere::new();
        let mut obj = Object::Sphere(s);
        let material = Material::default();
        obj.set_material(material);
        assert_eq!(obj.get_material().ambient, 0.1);
    }

    #[test]
    fn object_can_be_assigned_material() {
        let s = Sphere::new();
        let mut obj = Object::Sphere(s);
        let mut material = Material::default();
        material.ambient = 1.0;
        obj.set_material(material);
        assert_eq!(obj.get_material().ambient, 1.0);
    }

    #[test]
    fn object_intersecting_scaled_shape_with_ray() {
        let r = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let mut obj = Object::Test(TestShape::new());
        obj.set_transform(Transformation::Scale(2.0, 2.0, 2.0));
        _ = obj.mut_intersect(&r);
        let want_origin = Tuple::new_point(0.0, 0.0, -2.5);
        let want_direction = Tuple::new_vector(0.0, 0.0, 0.5);

        assert_eq!(obj.get_saved_ray().origin, want_origin);
        assert_eq!(obj.get_saved_ray().direction, want_direction);
    }

    #[test]
    fn object_intersecting_translated_shape_with_ray() {
        let r = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let mut obj = Object::Test(TestShape::new());
        obj.set_transform(Transformation::Translate(5.0, 0.0, 0.0));
        _ = obj.mut_intersect(&r);
        let want_origin = Tuple::new_point(-5.0, 0.0, -5.0);
        let want_direction = Tuple::new_vector(0.0, 0.0, 1.0);

        assert_eq!(obj.get_saved_ray().origin, want_origin);
        assert_eq!(obj.get_saved_ray().direction, want_direction);
    }

    #[test]
    fn object_compute_normal_on_translated_shape() {
        let mut obj = Object::Sphere(Sphere::new());
        obj.set_transform(Transformation::Translate(0.0, 1.0, 0.0));
        let got = obj.normal_at(&Tuple::new_point(0.0, 1.70711, -0.70711));
        let want = Tuple::new_vector(0.0, 0.70711, -0.70711);
        assert_eq!(got, want);
    }

    #[test]
    fn object_compute_normal_on_transformed_shape() {
        let mut obj = Object::Sphere(Sphere::new());
        obj.set_transform(Transformation::Chain(vec![
            Transformation::RotateZ(PI / 5.0),
            Transformation::Scale(1.0, 0.5, 1.0),
        ]));
        let root_2_2 = (2.0 as f64).sqrt() / 2.0;
        let world_point = Tuple::new_point(0.0, root_2_2, -root_2_2);
        let got = obj.normal_at(&world_point);
        let want = Tuple::new_vector(0.0, 0.97014, -0.24254);
        assert_eq!(got, want);
    }
}
