use std::sync::atomic::Ordering;

use crate::{
    math::{
        matrix::{Matrix, Transformation},
        ray::Ray,
        tuple::Tuple,
        EPSILON,
    },
    render::{hit::Intersection, material::Material},
};

use super::{Object, ObjectType, OBJECT_COUNTER};

#[derive(Debug, Clone)]
pub struct Plane {
    id: usize,
    tp: ObjectType,
    transform: Transformation,
    pub material: Material,
    cached_matrix: Matrix,
}

impl Plane {
    pub fn new() -> Self {
        Self {
            id: OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst),
            tp: ObjectType::Plane,
            transform: Transformation::None,
            material: Material::default(),
            cached_matrix: Matrix::identity_matrix(4),
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if ray.direction.y.abs() < EPSILON {
            return vec![];
        }
        let t = -ray.origin.y / ray.direction.y;
        vec![Intersection::new(Object::Plane(self.clone()), t)]
    }

    pub fn local_normal_at(&self, object_point: &Tuple) -> Tuple {
        Tuple::new_vector(0.0, 1.0, 0.0)
    }

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let object_point = &self.get_transform().inverse() * world_point;
        self.local_normal_at(&object_point)
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let tformed_ray = ray.transform(&self.get_transform().inverse());
        self.local_intersect(&tformed_ray)
    }

    pub fn set_transform(&mut self, tform: Transformation) {
        let m = Matrix::transform(&tform);
        self.transform = tform;
        self.cached_matrix = m;
    }

    pub fn get_transform(&self) -> Matrix {
        self.cached_matrix.clone()
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_type(&self) -> ObjectType {
        self.tp
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.tp == other.tp && self.id == other.id
    }
}

#[cfg(test)]
mod test {
    use crate::math::{ray::Ray, tuple::Tuple};

    use super::Plane;

    #[test]
    fn plane_normal_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(&Tuple::new_point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(&Tuple::new_point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(&Tuple::new_point(-5.0, 0.0, 150.0));

        let want = &Tuple::new_vector(0.0, 1.0, 0.0);
        assert_eq!(want, &n1);
        assert_eq!(want, &n2);
        assert_eq!(want, &n3);
    }

    #[test]
    fn plane_intersect_with_a_ray_parallel_to_plane() {
        let p = Plane::new();
        let ray = Ray::new((0.0, 10.0, 0.0), (0.0, 0.0, 1.0));
        let xs = p.local_intersect(&ray);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn plane_intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let xs = p.local_intersect(&ray);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn plane_ray_intersecting_from_above() {
        let p = Plane::new();
        let obj_id = p.get_id();
        let ray = Ray::new((0.0, 1.0, 0.0), (0.0, -1.0, 0.0));
        let xs = p.local_intersect(&ray);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].object.get_id(), obj_id);
    }

    #[test]
    fn plane_ray_intersecting_from_below() {
        let p = Plane::new();
        let obj_id = p.get_id();
        let ray = Ray::new((0.0, -1.0, 0.0), (0.0, 1.0, 0.0));
        let xs = p.local_intersect(&ray);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].object.get_id(), obj_id);
    }
}
