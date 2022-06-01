use super::{matrix::Matrix, tuple::Tuple};

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,    // Point
    pub direction: Tuple, // Vector
}

impl Ray {
    pub fn new(origin: (f64, f64, f64), direction: (f64, f64, f64)) -> Self {
        Self {
            origin: Tuple::new_point(origin.0, origin.1, origin.2),
            direction: Tuple::new_vector(direction.0, direction.1, direction.2),
        }
    }

    pub fn position_at(&self, t: f64) -> Tuple {
        self.origin + (self.direction * t)
    }

    pub fn transform(&self, tform: &Matrix) -> Ray {
        Ray {
            origin: self.origin * tform,
            direction: self.direction * tform,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::math::{
        matrix::{Matrix, Transformation},
        ray::Ray,
        tuple::Tuple,
    };

    #[test]
    fn ray_can_create_ray() {
        let origin = Tuple::new_point(1.0, 2.0, 3.0);
        let direction = Tuple::new_vector(4.0, 5.0, 6.0);
        let r = Ray::new((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn ray_can_get_position_along_ray() {
        let r = Ray::new((2.0, 3.0, 4.0), (1.0, 0.0, 0.0));

        let want1 = Tuple::new_point(2.0, 3.0, 4.0);
        let want2 = Tuple::new_point(3.0, 3.0, 4.0);
        let want3 = Tuple::new_point(1.0, 3.0, 4.0);
        let want4 = Tuple::new_point(4.5, 3.0, 4.0);

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
        let r = Ray::new((1.0, 2.0, 3.0), (0.0, 1.0, 0.0));
        let translate = Transformation::Translate(3.0, 4.0, 5.0);
        let r2 = r.transform(&Matrix::transform(translate));
        assert_eq!(r2.origin, Tuple::new_point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn ray_can_transform_scale_a_ray() {
        let r = Ray::new((1.0, 2.0, 3.0), (0.0, 1.0, 0.0));
        let translate = Transformation::Scale(2.0, 3.0, 4.0);
        let r2 = r.transform(&Matrix::transform(translate));
        assert_eq!(r2.origin, Tuple::new_point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::new_vector(0.0, 3.0, 0.0));
    }
}
