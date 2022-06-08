use super::{matrix::Matrix, point::Point, tuple::Tuple, vector::Vector};

pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::identity();
    m[0][3] = x;
    m[1][3] = y;
    m[2][3] = z;
    m
}

pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::identity();
    m[0][0] = x;
    m[1][1] = y;
    m[2][2] = z;
    m
}

pub fn rotate_x(angle: f64) -> Matrix {
    let mut m = Matrix::identity();
    m[1][1] = angle.cos();
    m[1][2] = -angle.sin();
    m[2][1] = angle.sin();
    m[2][2] = angle.cos();
    m
}

pub fn rotate_y(angle: f64) -> Matrix {
    let mut m = Matrix::identity();
    m[0][0] = angle.cos();
    m[0][2] = angle.sin();
    m[2][0] = -angle.sin();
    m[2][2] = angle.cos();
    m
}

pub fn rotate_z(angle: f64) -> Matrix {
    let mut m = Matrix::identity();
    m[0][0] = angle.cos();
    m[0][1] = -angle.sin();
    m[1][0] = angle.sin();
    m[1][1] = angle.cos();
    m
}

pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut m = Matrix::identity();
    m[0][1] = xy;
    m[0][2] = xz;
    m[1][0] = yx;
    m[1][2] = yz;
    m[2][0] = zx;
    m[2][1] = zy;

    m
}

pub fn view_transform(from: &Point, to: &Point, up: &Vector) -> Matrix {
    let forward = (*to - *from).normalize();
    let up_normal = up.normalize();
    let left = forward.cross(&up_normal);
    let true_up = left.cross(&forward);

    let orientation = Matrix::new().with_data([
        [left.x(), left.y(), left.z(), 0.0],
        [true_up.x(), true_up.y(), true_up.z(), 0.0],
        [-forward.x(), -forward.y(), -forward.z(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let translation = translate(-from.x(), -from.y(), -from.z());
    orientation * translation
}

pub trait Transformable {
    fn with_transform(self, tform: Matrix) -> Self;
    fn get_transform(&self) -> Matrix;

    fn translate(self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let translate = translate(x, y, z);
        self.with_transform(translate)
    }

    fn scale(self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let scale = scale(x, y, z);
        self.with_transform(scale)
    }

    fn rotate_x(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let rotate = rotate_x(angle);
        self.with_transform(rotate)
    }

    fn rotate_y(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let rotate = rotate_y(angle);
        self.with_transform(rotate)
    }

    fn rotate_z(self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let rotate = rotate_z(angle);
        self.with_transform(rotate)
    }

    fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self
    where
        Self: Sized,
    {
        let rotate = shear(xy, xz, yx, yz, zx, zy);
        self.with_transform(rotate)
    }

    fn view_transform(self, from: &Point, to: &Point, up: &Vector) -> Self
    where
        Self: Sized,
    {
        self.with_transform(view_transform(from, to, up))
    }
}

#[cfg(test)]
mod test {

    use std::f64::consts::PI;

    use super::*;
    use crate::math::ray::Ray;

    #[test]
    fn create_translation_matrix() {
        let tx = translate(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        let want = Point::new(2.0, 1.0, 7.0);
        let got = tx * p;
        assert_eq!(got, want);
    }

    #[test]
    fn can_multiply_by_inverse_of_translation() {
        let tx = translate(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        let want = Point::new(-8.0, 7.0, 3.0);
        let inverse = tx.inverse();
        let got = inverse * p;
        assert_eq!(got, want);
    }

    #[test]
    fn translation_does_not_effect_vectors() {
        let tx = translate(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        let want = Vector::new(-3.0, 4.0, 5.0);
        let got = tx * v;
        assert_eq!(want, got);
    }

    #[test]
    fn can_apply_scaling_to_point() {
        let tx = scale(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        let want = Point::new(-8.0, 18.0, 32.0);
        let got = tx * p;
        assert_eq!(got, want);
    }

    #[test]
    fn can_apply_scaling_to_vector() {
        let tx = scale(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        let want = Vector::new(-8.0, 18.0, 32.0);
        let got = tx * v;
        assert_eq!(got, want);
    }

    #[test]
    fn can_multiply_by_inverse_of_scaling_matrix() {
        let tx = scale(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        let want = Vector::new(-2.0, 2.0, 2.0);

        let inverse = tx.inverse();
        let got = inverse * v;
        assert_eq!(got, want);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let tx = scale(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let want = Point::new(-2.0, 3.0, 4.0);
        let got = tx * p;
        assert_eq!(got, want);
    }

    #[test]
    fn rotate_point_around_x_axis() {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(0.0, 1.0, 0.0);
        let tx_half_quarter = rotate_x(PI / 4.0);
        let tx_full_quarter = rotate_x(PI / 2.0);

        let want1 = Point::new(0.0, (2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0);
        let want2 = Point::new(0.0, 0.0, 1.0);

        let got1 = tx_half_quarter * p1;
        let got2 = tx_full_quarter * p2;

        assert_eq!(got1, want1);
        assert_eq!(got2, want2);
    }

    #[test]
    fn inverse_x_rotation_rotates_opposite_direction() {
        let p = Point::new(0.0, 1.0, 0.0);
        let tx = rotate_x(PI / 4.0);
        let inv = tx.inverse();
        let want = Point::new(0.0, (2.0 as f64).sqrt() / 2.0, -((2.0 as f64).sqrt() / 2.0));
        let got = inv * p;
        assert_eq!(got, want);
    }

    #[test]
    fn rotate_point_around_y_axis() {
        let p1 = Point::new(0.0, 0.0, 1.0);
        let p2 = Point::new(0.0, 0.0, 1.0);
        let tx_half_quarter = rotate_y(PI / 4.0);
        let tx_full_quarter = rotate_y(PI / 2.0);

        let want1 = Point::new((2.0 as f64).sqrt() / 2.0, 0.0, (2.0 as f64).sqrt() / 2.0);
        let want2 = Point::new(1.0, 0.0, 0.0);

        let got1 = tx_half_quarter * p1;
        let got2 = tx_full_quarter * p2;

        assert_eq!(got1, want1);
        assert_eq!(got2, want2);
    }

    #[test]
    fn rotate_point_around_z_axis() {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(0.0, 1.0, 0.0);
        let tx_half_quarter = rotate_z(PI / 4.0);
        let tx_full_quarter = rotate_z(PI / 2.0);

        let want1 = Point::new(-((2.0 as f64).sqrt()) / 2.0, (2.0 as f64).sqrt() / 2.0, 0.0);
        let want2 = Point::new(-1.0, 0.0, 0.0);

        let got1 = tx_half_quarter * p1;
        let got2 = tx_full_quarter * p2;

        assert_eq!(got1, want1);
        assert_eq!(got2, want2);
    }

    #[test]
    fn shear_point_x_y() {
        let tx = shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let want = Point::new(5.0, 3.0, 4.0);
        let got = tx * p;
        assert_eq!(got, want);
    }

    #[test]
    fn shear_point_x_z() {
        let tx = shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let want = Point::new(6.0, 3.0, 4.0);
        let got = tx * p;
        assert_eq!(got, want);
    }

    #[test]
    fn shear_point_y_x() {
        let tx = shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let want = Point::new(2.0, 5.0, 4.0);
        let got = tx * p;
        assert_eq!(got, want);
    }

    #[test]
    fn shear_point_y_z() {
        let tx = shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let want = Point::new(2.0, 7.0, 4.0);
        let got = tx * p;
        assert_eq!(got, want);
    }

    #[test]
    fn shear_point_z_x() {
        let tx = shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let want = Point::new(2.0, 3.0, 6.0);
        let got = tx * p;
        assert_eq!(got, want);
    }

    #[test]
    fn shear_point_z_y() {
        let tx = shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        let want = Point::new(2.0, 3.0, 7.0);
        let got = tx * p;
        assert_eq!(got, want);
    }

    #[test]
    fn can_apply_multiple_transforms_in_sequence() {
        let p = Point::new(1.0, 0.0, 1.0);
        let tx_a = rotate_x(PI / 2.0);
        let tx_b = scale(5.0, 5.0, 5.0);
        let tx_c = translate(10.0, 5.0, 7.0);

        let p2 = tx_a * p;
        let want1 = Point::new(1.0, -1.0, 0.0);
        assert_eq!(&want1, &p2);

        let p3 = tx_b * p2;
        let want2 = Point::new(5.0, -5.0, 0.0);
        assert_eq!(&want2, &p3);

        let p4 = tx_c * p3;
        let want3 = Point::new(15.0, 0.0, 7.0);
        assert_eq!(want3, p4);
    }

    #[test]
    fn can_apply_chained_transforms() {
        let p = Point::new(1.0, 0.0, 1.0);
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0))
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        let tx = ray.get_transform();

        let got = tx * p;
        let want = Point::new(15.0, 0.0, 7.0);
        assert_eq!(want, got);
    }

    #[test]
    fn transform_matrix_for_default_view_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let tform = view_transform(&from, &to, &up);
        assert_eq!(tform, Matrix::identity());
    }

    #[test]
    fn transform_view_looking_in_positive_z() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let tform = view_transform(&from, &to, &up);
        let scale_tform = scale(-1.0, 1.0, -1.0);
        assert_eq!(tform, scale_tform);
    }

    #[test]
    fn view_transformation_moves_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let tform = view_transform(&from, &to, &up);
        let translate_tform = translate(0.0, 0.0, -8.0);
        assert_eq!(tform, translate_tform);
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let got = view_transform(&from, &to, &up);
        let want = Matrix::new().with_data([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(got, want);
    }
}
