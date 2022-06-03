use self::{plane::Plane, sphere::Sphere};
use super::{
    hit::{Hittable, Intersection},
    material::Material,
};
use crate::math::{
    matrix::{Matrix, Transformation},
    ray::Ray,
    tuple::Tuple,
};
use std::sync::atomic::AtomicUsize;

pub mod plane;
pub mod sphere;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectType {
    Sphere,
    Plane,
}

static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Object {
    pub fn get_id(&self) -> usize {
        match self {
            Self::Plane(p) => p.get_id(),
            Self::Sphere(s) => s.get_id(),
        }
    }

    pub fn get_type(&self) -> ObjectType {
        match self {
            Self::Plane(p) => p.get_type(),
            Self::Sphere(s) => s.get_type(),
        }
    }

    pub fn get_material(&self) -> &Material {
        match self {
            Self::Plane(p) => p.get_material(),
            Self::Sphere(s) => s.get_material(),
        }
    }

    pub fn set_material(&mut self, mat: Material) {
        match self {
            Self::Plane(p) => p.set_material(mat),
            Self::Sphere(s) => s.set_material(mat),
        }
    }

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let local_point = self.get_transform().inverse() * world_point;
        match self {
            Self::Plane(p) => p.local_normal_at(&local_point),
            Self::Sphere(s) => s.local_normal_at(&local_point),
        }
    }

    pub fn set_transform(&mut self, tform: Transformation) {
        match self {
            Self::Plane(p) => p.set_transform(tform),
            Self::Sphere(s) => s.set_transform(tform),
        }
    }

    pub fn get_transform(&self) -> Matrix {
        match self {
            Self::Plane(p) => p.get_transform(),
            Self::Sphere(s) => s.get_transform(),
        }
    }
}

impl Hittable for Object {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        match self {
            Self::Plane(p) => p.intersect(&ray),
            Self::Sphere(s) => s.intersect(&ray),
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
            tuple::Tuple,
        },
        render::material::Material,
        render::object::{Object, Sphere},
    };

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
