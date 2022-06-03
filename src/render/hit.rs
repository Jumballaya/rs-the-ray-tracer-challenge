use crate::math::tuple::Tuple;

use super::{super::math::ray::Ray, object::Object};

pub trait Hittable {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}

pub struct HitComputation<'a> {
    pub t: f64,
    pub object: &'a Object,
    pub point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub inside: bool,
}

pub struct Intersection {
    pub t: f64,
    pub object: Object,
}

impl Intersection {
    pub fn new(object: Object, t: f64) -> Self {
        Intersection { t, object }
    }

    pub fn get_hit<'a>(intersections: &'a Vec<Intersection>) -> Option<&'a Intersection> {
        let mut pos_ints: Vec<&'a Intersection> =
            intersections.iter().filter(|int| int.t > 0.0).collect();
        pos_ints.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        if pos_ints.len() > 0 {
            return Some(pos_ints[0]);
        }
        None
    }

    pub fn prepare_computation<'a>(
        intersection: &'a Intersection,
        ray: &Ray,
    ) -> HitComputation<'a> {
        let point = ray.position_at(intersection.t);
        let eye_vector = -ray.direction;
        let object = &intersection.object;
        let t = intersection.t;

        let (normal_vector, inside) = {
            let normal_vector = intersection.object.normal_at(&point);
            let normal_dot_eye = normal_vector * eye_vector;
            if normal_dot_eye < 0.0 {
                (-normal_vector, true)
            } else {
                (normal_vector, false)
            }
        };

        HitComputation {
            t,
            object,
            point,
            eye_vector,
            normal_vector,
            inside,
        }
    }

    pub fn prepare_computations<'a>(
        intersections: &'a Vec<Intersection>,
        ray: &Ray,
    ) -> Vec<HitComputation<'a>> {
        intersections
            .iter()
            .map(|intersection| Intersection::prepare_computation(intersection, ray))
            .collect()
    }
}

#[cfg(test)]

mod test {

    use super::*;
    use crate::render::{
        hit::Intersection,
        object::{sphere::Sphere, Object},
    };

    #[test]
    fn hit_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let s_copy = s.clone();
        let i = Intersection::new(Object::Sphere(s), 3.5);
        assert!(i.t == 3.5);
        assert!(i.object.get_id() == s_copy.get_id());
    }

    #[test]
    fn hit_intersect_sets_object_on_intersection() {
        let r = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let s = &Sphere::new();
        let s_copy = s.clone();
        let xs = s.intersect(&r);
        assert!(xs.len() == 2);
        assert_eq!(xs[0].object.get_id(), s_copy.get_id());
        assert_eq!(xs[1].object.get_id(), s_copy.get_id());
    }

    #[test]
    fn hit_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(Object::Sphere(s.clone()), 1.0);
        let i1_c = Intersection::new(Object::Sphere(s.clone()), 1.0);
        let i2 = Intersection::new(Object::Sphere(s), 2.0);
        if let Some(hit) = Intersection::get_hit(&vec![i1, i2]) {
            assert!(hit.t == i1_c.t);
            assert!(hit.object.get_id() == i1_c.object.get_id());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn hit_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(Object::Sphere(s.clone()), -2.0);
        let i2 = Intersection::new(Object::Sphere(s), -1.0);
        if let None = Intersection::get_hit(&vec![i1, i2]) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn hit_hit_is_lowest_non_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(Object::Sphere(s.clone()), 5.0);
        let i2 = Intersection::new(Object::Sphere(s.clone()), 7.0);
        let i3 = Intersection::new(Object::Sphere(s.clone()), -3.0);
        let i4 = Intersection::new(Object::Sphere(s.clone()), 2.0);
        let i4_c = Intersection::new(Object::Sphere(s), 2.0);

        if let Some(hit) = Intersection::get_hit(&vec![i1, i2, i3, i4]) {
            assert!(hit.t == i4_c.t);
            assert!(hit.object.get_id() == i4_c.object.get_id());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn hit_precompute_state_of_intersection() {
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let id = shape.get_id();
        let intersection = Intersection::new(Object::Sphere(shape), 4.0);
        let comp = Intersection::prepare_computation(&intersection, &ray);

        assert_eq!(comp.t, intersection.t);
        assert_eq!(comp.object.get_id(), id);
        assert_eq!(comp.point, Tuple::new_point(0.0, 0.0, -1.0));
        assert_eq!(comp.eye_vector, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comp.normal_vector, Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_intersection_is_on_outside() {
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let intersection = Intersection::new(Object::Sphere(shape), 4.0);
        let comp = Intersection::prepare_computation(&intersection, &ray);
        assert!(!comp.inside);
    }

    #[test]
    fn hit_intersection_is_on_inside() {
        let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let intersection = Intersection::new(Object::Sphere(shape), 1.0);
        let comp = Intersection::prepare_computation(&intersection, &ray);
        assert_eq!(comp.point, Tuple::new_point(0.0, 0.0, 1.0));
        assert_eq!(comp.eye_vector, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comp.normal_vector, Tuple::new_vector(0.0, 0.0, -1.0));
        assert!(comp.inside);
    }
}
