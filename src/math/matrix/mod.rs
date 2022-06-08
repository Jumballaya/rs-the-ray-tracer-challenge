mod matrix2;
mod matrix3;

use std::ops::{Index, IndexMut, Mul};

use crate::math::epsilon::ApproxEq;
use crate::math::matrix::matrix3::Matrix3;

use super::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    data: [[f64; 4]; 4],
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn identity() -> Matrix {
        Matrix {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn with_data(mut self, data: [[f64; 4]; 4]) -> Self {
        self.data = data;
        self
    }

    pub fn transpose(&self) -> Matrix {
        let mut m = Matrix::new();
        for col in 0..4 {
            for row in 0..4 {
                m[col][row] = self[row][col];
            }
        }
        m
    }

    pub fn determinant(&self) -> f64 {
        let mut sum = 0.0;
        for index in 0..4 {
            let col = self[0][index];
            sum += col * self.cofactor(0, index);
        }
        sum
    }

    fn sub_matrix(&self, row_sub: usize, col_sub: usize) -> Matrix3 {
        let mut m = Matrix3::new();

        let mut y = 0;
        let mut x = 0;
        for row in 0..4 {
            if row != row_sub {
                for col in 0..4 {
                    if col != col_sub {
                        m[y][x] = self[row][col];
                        x += 1;
                    }
                }
                x = 0;
                y += 1;
            }
        }
        m
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.sub_matrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn is_invertible(&self) -> bool {
        !(0.0).approx_eq(self.determinant())
    }

    pub fn inverse(&self) -> Matrix {
        if !self.is_invertible() {
            panic!("Non invertible matrix")
        }
        let mut m = Matrix::new();
        for row in 0..4 {
            for col in 0..4 {
                let cofactor = self.cofactor(row, col);
                m[col][row] = cofactor / self.determinant();
            }
        }
        m
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if !self[i][j].approx_eq(other[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Index<usize> for Matrix {
    type Output = [f64; 4];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::new();

        for i in 0..4 {
            for j in 0..4 {
                res[i][j] = self[i][0] * rhs[0][j]
                    + self[i][1] * rhs[1][j]
                    + self[i][2] * rhs[2][j]
                    + self[i][3] * rhs[3][j];
            }
        }
        res
    }
}

impl<T> Mul<T> for Matrix
where
    T: Tuple,
{
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        let x = self[0][0] * rhs.x()
            + self[0][1] * rhs.y()
            + self[0][2] * rhs.z()
            + self[0][3] * rhs.w();
        let y = self[1][0] * rhs.x()
            + self[1][1] * rhs.y()
            + self[1][2] * rhs.z()
            + self[1][3] * rhs.w();
        let z = self[2][0] * rhs.x()
            + self[2][1] * rhs.y()
            + self[2][2] * rhs.z()
            + self[2][3] * rhs.w();
        Self::Output::new(x, y, z)
    }
}

#[cfg(test)]
mod test {
    use crate::math::{
        epsilon::ApproxEq,
        matrix::{matrix3::Matrix3, Matrix},
        point::Point,
        tuple::Tuple,
        vector::Vector,
    };

    #[test]
    fn create_4x4_matrix() {
        let m = Matrix::new().with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert!(m[0][0].approx_eq(1.0));
        assert!(m[0][3].approx_eq(4.0));
        assert!(m[1][0].approx_eq(5.5));
        assert!(m[1][2].approx_eq(7.5));
        assert!(m[2][2].approx_eq(11.0));
        assert!(m[3][0].approx_eq(13.5));
        assert!(m[3][2].approx_eq(15.5));
    }

    #[test]
    fn matrix_matrix_equality() {
        let m1 = Matrix::new().with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new().with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert!(m1 == m2);
    }

    #[test]
    fn matrix_matrix_inequality() {
        let m1 = Matrix::new().with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new().with_data([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert!(m1 != m2);
    }

    #[test]
    fn matrix_can_multiply_2_matrices() {
        let m1 = Matrix::new().with_data([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new().with_data([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let want = Matrix::new().with_data([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        let got = m1 * m2;
        assert!(got == want);
    }

    #[test]
    fn matrix_can_multiply_matrix_by_point() {
        let m = Matrix::new().with_data([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let t = Point::new(1.0, 2.0, 3.0);
        let want = Point::new(18.0, 24.0, 33.0);
        let got = m * t;
        assert!(want == got);
    }

    #[test]
    fn matrix_can_multiply_matrix_by_vector() {
        let m = Matrix::new().with_data([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let t = Vector::new(1.0, 2.0, 3.0);
        let want = Vector::new(14.0, 22.0, 32.0);
        let got = m * t;
        assert!(want == got);
    }

    #[test]
    fn matrix_can_create_idenity_matrix() {
        let ident4 = Matrix::new().with_data([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(ident4, Matrix::identity());
    }

    #[test]
    fn matrix_multiply_by_identity_matrix_by_matrix() {
        let ident = Matrix::identity();
        let m = Matrix::new().with_data([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        let got = m * ident;
        let want = Matrix::new().with_data([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        assert!(got == want);
    }

    #[test]
    fn matrix_multiply_by_identity_matrix_by_tuple() {
        let ident = Matrix::new().with_data([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let t = Vector::new(1.0, 2.0, 3.0);
        let got = ident * t;
        let want = Vector::new(1.0, 2.0, 3.0);
        assert!(got == want);
    }

    #[test]
    fn matrix_can_transpose_a_matrix() {
        let m = Matrix::new().with_data([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let want = Matrix::new().with_data([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        let got = m.transpose();
        assert!(got == want);
    }

    #[test]
    fn matrix_can_transpose_ident_matrix() {
        let ident = Matrix::new().with_data([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let want = Matrix::new().with_data([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let got = ident.transpose();
        assert!(got == want);
    }

    #[test]
    fn matrix_can_get_3x3_submatrix() {
        let m = Matrix::new().with_data([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let want =
            Matrix3::new().with_data([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
        let got = m.sub_matrix(2, 1);
        assert!(want == got);
    }

    #[test]
    fn matrix_can_get_4x4_matrix_determinate() {
        let m = Matrix::new().with_data([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert!(m.cofactor(0, 0).approx_eq(690.0));
        assert!(m.cofactor(0, 1).approx_eq(447.0));
        assert!(m.cofactor(0, 2).approx_eq(210.0));
        assert!(m.cofactor(0, 3).approx_eq(51.0));
        assert!(m.determinant().approx_eq(-4071.0));
    }

    #[test]
    fn matrix_can_matrix_for_invertability() {
        let m = Matrix::new().with_data([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert!(m.determinant().approx_eq(-2120.0));
        assert!(m.is_invertible());

        let m2 = Matrix::new().with_data([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert!(m2.determinant().approx_eq(0.0));
        assert!(!m2.is_invertible());
    }

    #[test]
    fn matrix_can_calculate_inverse_of_a_matrix_1() {
        let m = Matrix::new().with_data([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let det = m.determinant();
        let cofactor1 = m.cofactor(2, 3);
        let cofactor2 = m.cofactor(3, 2);
        let m2 = m.inverse();
        assert!(det.approx_eq(532.0));
        assert!(cofactor1.approx_eq(-160.0));
        assert!(m2[3][2].approx_eq(-(160.0 / 532.0)));
        assert!(cofactor2.approx_eq(105.0));
        assert!(m2[2][3].approx_eq(105.0 / 532.0));

        let want = Matrix::new().with_data([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert_eq!(want, m2);
    }

    #[test]
    fn matrix_can_calculate_inverse_of_a_matrix_2() {
        let m = Matrix::new().with_data([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let m2 = m.inverse();
        let want = Matrix::new().with_data([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert!(want == m2);
    }

    #[test]
    fn matrix_can_calculate_inverse_of_a_matrix_3() {
        let m = Matrix::new().with_data([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let m2 = m.inverse();
        let want = Matrix::new().with_data([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert!(want == m2);
    }

    #[test]
    fn matrix_multiply_product_by_inverse() {
        let m_a = Matrix::new().with_data([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let m_b = Matrix::new().with_data([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let m_c = m_a * m_b;

        assert_eq!(m_a, m_c * m_b.inverse());
    }
}
