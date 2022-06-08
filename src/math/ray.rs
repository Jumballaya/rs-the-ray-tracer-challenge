use super::{matrix::Matrix, point::Point, transformation::Transformable, vector::Vector};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    transformation: Matrix,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self {
            origin,
            direction,
            transformation: Matrix::identity(),
        }
    }

    pub fn position_at(&self, t: f64) -> Point {
        self.origin + (self.direction * t)
    }
}

impl Transformable for Ray {
    fn with_transform(self, tform: Matrix) -> Ray {
        Ray {
            origin: tform * self.origin,
            direction: tform * self.direction,
            transformation: tform * self.get_transform(),
        }
    }

    fn get_transform(&self) -> Matrix {
        self.transformation
    }
}

#[cfg(test)]
mod test {

    use super::{Point, Ray, Transformable, Vector};
    use crate::math::tuple::Tuple;

    #[test]
    fn ray_can_create_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn ray_can_get_position_along_ray() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        let want1 = Point::new(2.0, 3.0, 4.0);
        let want2 = Point::new(3.0, 3.0, 4.0);
        let want3 = Point::new(1.0, 3.0, 4.0);
        let want4 = Point::new(4.5, 3.0, 4.0);

        let got1 = r.position_at(0.0);
        let got2 = r.position_at(1.0);
        let got3 = r.position_at(-1.0);
        let got4 = r.position_at(2.5);

        assert_eq!(got1, want1);
        assert_eq!(got2, want2);
        assert_eq!(got3, want3);
        assert_eq!(got4, want4);
    }

    #[test]
    fn ray_can_transform_translate_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0))
            .translate(3.0, 4.0, 5.0);

        assert_eq!(r.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn ray_can_transform_scale_a_ray() {
        let r =
            Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0)).scale(2.0, 3.0, 4.0);
        assert_eq!(r.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r.direction, Vector::new(0.0, 3.0, 0.0));
    }
}
