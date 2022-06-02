use super::{
    super::math::ray::Ray,
    object::{Object, ObjectType},
};

pub trait Hittable {
    fn get_type(&self) -> ObjectType;
    fn get_id(&self) -> usize;
    fn intersect(&self, ray: Ray) -> Vec<Intersection>;
}

impl PartialEq for dyn Hittable {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type() && self.get_id() == other.get_id()
    }
}

pub struct Intersection {
    pub t: f64,
    pub object: Object,
}

impl<'a> Intersection {
    pub fn new(object: Object, t: f64) -> Self {
        Intersection { t, object }
    }

    pub fn get_hit(intersections: &[Intersection]) -> Option<&Intersection> {
        let mut pos_ints: Vec<&Intersection> =
            intersections.iter().filter(|int| int.t > 0.0).collect();
        pos_ints.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        if pos_ints.len() > 0 {
            return Some(pos_ints[0]);
        }
        None
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
        let xs = s.intersect(r);
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
        if let Some(hit) = Intersection::get_hit(&[i1, i2]) {
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
        if let None = Intersection::get_hit(&[i1, i2]) {
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

        if let Some(hit) = Intersection::get_hit(&[i1, i2, i3, i4]) {
            assert!(hit.t == i4_c.t);
            assert!(hit.object.get_id() == i4_c.object.get_id());
        } else {
            assert!(false);
        }
    }
}
