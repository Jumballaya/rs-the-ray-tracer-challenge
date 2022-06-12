use crate::{
    math::{matrix::Matrix, point::Point, ray::Ray, transformation::Transformable, vector::Vector},
    render::{intersections::Intersections, object::Object, shape::Shape},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    children: Vec<Object>,
}

impl Group {
    pub fn new(children: Vec<Object>) -> Self {
        Self { children }
    }

    pub fn new_empty() -> Self {
        Self { children: vec![] }
    }

    pub fn intersect<'a>(
        &'a self,
        ray: &Ray,
        _: &'a Object,
        intersections: &mut Intersections<'a>,
    ) {
        for child in &self.children {
            child.intersect(ray, intersections)
        }
    }

    pub fn normal_at(&self, _point: &Point) -> Vector {
        unreachable!()
    }

    pub fn children(&self) -> &Vec<Object> {
        &self.children
    }

    pub fn add_child(&mut self, child: Object) {
        self.children.push(child);
    }
}

#[derive(Clone, Debug)]
pub enum GroupTree {
    Leaf(Object),
    Node(Object, Vec<GroupTree>),
}

impl GroupTree {
    pub fn build(self) -> Object {
        GroupTree::walk(self, &Matrix::identity())
    }

    fn walk(tree: Self, transform: &Matrix) -> Object {
        match tree {
            GroupTree::Leaf(o) => o.with_transform(*transform),
            GroupTree::Node(group, children) => {
                let child_transform = *transform * group.get_transform();
                let new_children = children
                    .into_iter()
                    .map(|child| GroupTree::walk(child, &child_transform))
                    .collect();
                let obj = group.with_shape(Shape::Group(Group::new(new_children)));

                Object::new_test_shape().with_shape(obj.get_shape().clone())
            }
        }
    }

    pub fn from_object(object: &Object) -> Self {
        match object.get_shape() {
            Shape::Group(g) => GroupTree::Node(
                object.clone(),
                g.children()
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
                    .collect(),
            ),
            _other => GroupTree::Leaf(object.clone()),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        math::{
            point::Point, ray::Ray, transformation::Transformable, tuple::Tuple, vector::Vector,
        },
        render::{intersections::Intersections, object::Object},
    };

    use super::Group;

    #[test]
    fn add_a_child_to_a_group() {
        let mut g = Group::new_empty();
        let s = Object::new_sphere();
        g.add_child(s.clone());
        assert_eq!(&g.children()[0], &s);
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = Group::new_empty();
        let obj = Object::new_test_shape();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut ints = Intersections::new();
        g.intersect(&r, &obj, &mut ints);
        assert_eq!(ints.len(), 0);
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut g = Group::new_empty();
        let obj = Object::new_test_shape();
        let s1 = Object::new_sphere();
        let s2 = Object::new_sphere().translate(0.0, 0.0, -3.0);
        let s3 = Object::new_sphere().translate(5.0, 0.0, 0.0);
        g.add_child(s1.clone());
        g.add_child(s2.clone());
        g.add_child(s3.clone());

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut ints = Intersections::new();
        g.intersect(&r, &obj, &mut ints);
        assert_eq!(ints.len(), 4);
        assert_eq!(ints[0].object(), &s2);
        assert_eq!(ints[1].object(), &s2);
        assert_eq!(ints[2].object(), &s1);
        assert_eq!(ints[3].object(), &s1);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let s = Object::new_sphere().translate(5.0, 0.0, 0.0);
        let g = Object::new_group(vec![s]).scale(2.0, 2.0, 2.0);
        let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::new(0.0, 0.0, 1.0));
        let mut ints = Intersections::new();
        g.intersect(&r, &mut ints);
        assert_eq!(ints.len(), 2);
    }
}
