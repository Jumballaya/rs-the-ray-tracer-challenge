use crate::math::{
    matrix::{Matrix, Transformation},
    ray::Ray,
    tuple::Tuple,
};

#[derive(Debug)]
pub struct Camera {
    pub hsize: usize,       // Horizontal size (px) of the picture that will be rendered
    pub vsize: usize,       // Vertical size (px) of the picture that will be rendered
    pub field_of_view: f64, // Angle of vision width
    pub transforms: Vec<Transformation>, // Transformations
    pub pixel_size: f64,    // Relative size of pixel in world space
    pub half_width: f64,    // Half of the picture's width in world space units
    pub half_height: f64,   // Half of the picture's height in world space units
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect_ratio = (hsize as f64) / (vsize as f64);
        let (half_width, half_height) = {
            if aspect_ratio >= 1.0 {
                (half_view, half_view / aspect_ratio)
            } else {
                (half_view * aspect_ratio, half_view)
            }
        };
        let pixel_size = (half_width * 2.0) / (hsize as f64);
        Self {
            hsize,
            vsize,
            field_of_view,
            pixel_size,
            half_height,
            half_width,
            transforms: vec![],
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = ((x as f64) + 0.5) * self.pixel_size;
        let y_offset = ((y as f64) + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transform_inverse = self.get_transform().inverse();
        let pixel = &transform_inverse * Tuple::new_point(world_x, world_y, -1.0);
        let origin = transform_inverse * Tuple::new_point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        let (o_x, o_y, o_z, _) = origin.as_tuple();
        let (d_x, d_y, d_z, _) = direction.as_tuple();
        Ray::new((o_x, o_y, o_z), (d_x, d_y, d_z))
    }

    pub fn add_transform(&mut self, tform: Transformation) {
        self.transforms.push(tform);
        self.transforms.reverse();
    }

    pub fn get_transform(&self) -> Matrix {
        if self.transforms.len() == 0 {
            return Matrix::identity_matrix(4);
        }
        Matrix::transform_chain(&self.transforms)
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::math::{
        float_equal,
        matrix::{Matrix, Transformation},
        tuple::Tuple,
    };

    use super::Camera;

    #[test]
    fn camera_can_create_camera() {
        let c = Camera::new(160, 120, PI / 2.0);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert!(float_equal(c.field_of_view, PI / 2.0));
        assert_eq!(c.get_transform(), Matrix::identity_matrix(4));
    }

    #[test]
    fn camera_sets_correct_pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(float_equal(c.pixel_size, 0.01));
    }

    #[test]
    fn camera_sets_correct_pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(float_equal(c.pixel_size, 0.01));
    }

    #[test]
    fn camera_constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let ray = c.ray_for_pixel(100, 50);
        assert_eq!(ray.origin, Tuple::new_point(0.0, 0.0, 0.0));
        assert_eq!(ray.direction, Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn camera_constructing_a_ray_through_the_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let ray = c.ray_for_pixel(0, 0);
        assert_eq!(ray.origin, Tuple::new_point(0.0, 0.0, 0.0));
        assert_eq!(ray.direction, Tuple::new_vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn camera_constructing_a_ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.add_transform(Transformation::RotateY(PI / 4.0));
        c.add_transform(Transformation::Translate(0.0, -2.0, 5.0));
        let ray = c.ray_for_pixel(100, 50);
        let root_2_2 = (2.0 as f64).sqrt() / 2.0;
        assert_eq!(ray.origin, Tuple::new_point(0.0, 2.0, -5.0));
        assert_eq!(ray.direction, Tuple::new_vector(root_2_2, 0.0, -root_2_2));
    }
}
