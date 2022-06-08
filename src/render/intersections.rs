use std::{cmp::Ordering, ops::Index, slice};

use crate::math::{epsilon::EPSILON, point::Point, ray::Ray, vector::Vector};
use crate::render::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &'a Object {
        &self.object
    }
}

impl<'a> std::cmp::Eq for Intersection<'a> {}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.t.is_nan() {
            Ordering::Greater
        } else if other.t.is_nan() {
            Ordering::Less
        } else if self.t > other.t {
            Ordering::Greater
        } else if self.t < other.t {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Debug)]
pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new() -> Self {
        Self {
            intersections: Vec::<Intersection<'a>>::with_capacity(16),
        }
    }

    pub fn with_intersections(mut self, intersections: Vec<Intersection<'a>>) -> Self {
        self.intersections = intersections;
        self.sort();
        self
    }

    pub fn sort(&mut self) {
        self.intersections.sort_unstable();
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn push(&mut self, int: Intersection<'a>) {
        self.intersections.push(int);
        self.sort();
    }

    pub fn get_hit(&self) -> Option<&Intersection> {
        self.iter().find(|int| int.t() > 0.0)
    }

    pub fn iter(&self) -> slice::Iter<Intersection> {
        self.intersections.iter()
    }

    pub fn intersections(&self) -> Vec<Intersection> {
        self.intersections.clone()
    }
}

impl Default for Intersections<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HitComputation<'a> {
    pub t: f64,
    pub object: &'a Object,
    pub point: Point,
    pub eye: Vector,
    pub normal: Vector,
    pub inside: bool,
    pub over_point: Point,
}

impl<'a> HitComputation<'a> {
    pub fn new(intersection: &'a Intersection, ray: &Ray) -> Self {
        let point = ray.position_at(intersection.t);
        let eye = -ray.direction;
        let object = &intersection.object;
        let t = intersection.t;

        let (normal, inside) = {
            let normal = intersection.object.normal_at(&point);
            let normal_dot_eye = normal * eye;
            if normal_dot_eye < 0.0 {
                (-normal, true)
            } else {
                (normal, false)
            }
        };

        let over_point = point + normal * EPSILON;

        Self {
            t,
            object,
            point,
            eye,
            normal,
            inside,
            over_point,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{HitComputation, Intersection, Intersections};
    use crate::{
        math::{
            epsilon::{ApproxEq, EPSILON},
            point::Point,
            ray::Ray,
            transformation::Transformable,
            tuple::Tuple,
            vector::Vector,
        },
        render::object::Object,
    };

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Object::new_sphere();
        let i = Intersection::new(3.5, &s);
        assert!(i.t == 3.5);
        assert_eq!(&s, i.object());
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let mut xs = Intersections::new();
        s.intersect(&r, &mut xs);
        assert!(xs.len() == 2);
        assert_eq!(xs[0].object(), &s);
        assert_eq!(xs[1].object(), &s);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(1.0, &s);
        let i1_c = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let intersections = Intersections::new().with_intersections(vec![i1, i2]);
        if let Some(hit) = intersections.get_hit() {
            assert!(hit.t.approx_eq(i1_c.t));
            assert_eq!(hit.object(), i1_c.object());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Object::new_sphere();
        let mut intersections = Intersections::new();

        intersections.push(Intersection::new(-2.0, &s));
        intersections.push(Intersection::new(-1.0, &s));

        if let None = intersections.get_hit() {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn hit_is_lowest_non_negative() {
        let obj = Object::new_sphere();
        let want = Intersection::new(2.0, &obj);
        let intersections = Intersections::new().with_intersections(vec![
            Intersection::new(5.0, &obj),
            Intersection::new(7.0, &obj),
            Intersection::new(-3.0, &obj),
            Intersection::new(2.0, &obj),
        ]);

        if let Some(hit) = intersections.get_hit() {
            assert!(hit.t().approx_eq(want.t()));
            assert_eq!(hit.object(), want.object());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn precompute_state_of_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object::new_sphere();
        let intersection = Intersection::new(4.0, &shape);
        let comp = HitComputation::new(&intersection, &ray);

        assert_eq!(comp.t, intersection.t);
        assert_eq!(comp.object, &shape);
        assert_eq!(comp.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comp.eye, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comp.normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn intersection_is_on_outside() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object::new_sphere();
        let intersection = Intersection::new(4.0, &shape);
        let comp = HitComputation::new(&intersection, &ray);
        assert!(!comp.inside);
    }

    #[test]
    fn intersection_is_on_inside() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object::new_sphere();
        let intersection = Intersection::new(1.0, &shape);
        let comp = HitComputation::new(&intersection, &ray);
        assert_eq!(comp.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comp.eye, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comp.normal, Vector::new(0.0, 0.0, -1.0));
        assert!(comp.inside);
    }

    #[test]
    fn should_offset_the_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Object::new_sphere().translate(0.0, 0.0, 1.0);
        let intersection = Intersection::new(5.0, &shape);
        let comp = HitComputation::new(&intersection, &r);

        assert!(comp.over_point.z() < -EPSILON / 2.0);
        assert!(comp.point.z() > comp.over_point.z());
    }
}
