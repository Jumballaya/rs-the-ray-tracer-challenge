use crate::math::{
    matrix::Matrix, point::Point, ray::Ray, transformation::Transformable, tuple::Tuple,
};

#[derive(Debug)]
pub struct Camera {
    hsize: usize,           // Horizontal size (px) of the picture that will be rendered
    vsize: usize,           // Vertical size (px) of the picture that will be rendered
    field_of_view: f64,     // Angle of vision width
    transformation: Matrix, // Transformation
    pixel_size: f64,        // Relative size of pixel in world space
    half_width: f64,        // Half of the picture's width in world space units
    half_height: f64,       // Half of the picture's height in world space units
    inv_matrix: Matrix,     // Cached inverse calculation of the camera's transform matrix
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
            transformation: Matrix::identity(),
            inv_matrix: Matrix::identity().inverse(),
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = ((x as f64) + 0.5) * self.pixel_size;
        let y_offset = ((y as f64) + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = self.inv_matrix * Point::new(world_x, world_y, -1.0);
        let origin = self.inv_matrix * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn hsize(&self) -> usize {
        self.hsize
    }

    pub fn vsize(&self) -> usize {
        self.vsize
    }
}

impl Transformable for Camera {
    fn get_transform(&self) -> Matrix {
        self.transformation
    }

    fn with_transform(self, tform: Matrix) -> Self {
        let new_tform = tform * self.transformation;
        Self {
            hsize: self.hsize,
            vsize: self.vsize,
            field_of_view: self.field_of_view,
            transformation: new_tform,
            pixel_size: self.pixel_size,
            half_width: self.half_width,
            half_height: self.half_height,
            inv_matrix: new_tform.inverse(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        draw::color::Color,
        math::{
            epsilon::ApproxEq,
            matrix::Matrix,
            point::Point,
            transformation::{rotate_y, translate, Transformable},
            tuple::Tuple,
            vector::Vector,
        },
    };
    use std::f64::consts::PI;

    use super::Camera;
    use crate::render::world::World;

    #[test]
    fn can_create_camera() {
        let c = Camera::new(160, 120, PI / 2.0);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert!(c.field_of_view.approx_eq(PI / 2.0));
        assert_eq!(c.get_transform(), Matrix::identity());
    }

    #[test]
    fn sets_correct_pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(c.pixel_size.approx_eq(0.01));
    }

    #[test]
    fn sets_correct_pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(c.pixel_size.approx_eq(0.01));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let ray = c.ray_for_pixel(100, 50);
        assert_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(ray.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_the_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let ray = c.ray_for_pixel(0, 0);
        assert_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(ray.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_camera_is_transformed() {
        let tform = rotate_y(PI / 4.0) * translate(0.0, -2.0, 5.0);
        let c = Camera::new(201, 101, PI / 2.0)
            .translate(0.0, -2.0, 5.0)
            .rotate_y(PI / 4.0);

        assert_eq!(tform, c.get_transform());
        assert_eq!(tform.inverse(), c.inv_matrix);
        let ray = c.ray_for_pixel(100, 50);
        let root_2_2 = (2.0 as f64).sqrt() / 2.0;
        assert_eq!(ray.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(ray.direction, Vector::new(root_2_2, 0.0, -root_2_2));
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let c = Camera::new(11, 11, PI / 2.0).view_transform(&from, &to, &up);
        let img = w.render(&c);

        let want = Color::new(0.380392, 0.474509, 0.282352);
        if let Some(got) = img.pixel_at((5, 5)) {
            assert_eq!(got, want);
        } else {
            assert!(false);
        }
    }
}
