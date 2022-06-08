use std::ops::{Index, IndexMut};

use crate::math::epsilon::ApproxEq;

use super::matrix2::Matrix2;

#[derive(Clone, Copy, Debug)]
pub struct Matrix3 {
    data: [[f64; 3]; 3],
}

impl Matrix3 {
    pub fn new() -> Matrix3 {
        Matrix3 {
            data: [[0.0; 3], [0.0; 3], [0.0; 3]],
        }
    }

    pub fn with_data(mut self, data: [[f64; 3]; 3]) -> Self {
        self.data = data;
        self
    }

    pub fn determinant(&self) -> f64 {
        let mut sum = 0.0;
        for index in 0..3 {
            let col = self[0][index];
            sum += col * self.cofactor(0, index);
        }
        sum
    }

    fn sub_matrix(&self, row_sub: usize, col_sub: usize) -> Matrix2 {
        let mut m = Matrix2::new();

        let mut y = 0;
        let mut x = 0;
        for row in 0..3 {
            if row != row_sub {
                for col in 0..3 {
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
}

impl Index<usize> for Matrix3 {
    type Output = [f64; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !self[i][j].approx_eq(other[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {

    use super::Matrix2;
    use super::Matrix3;
    use crate::math::epsilon::ApproxEq;

    #[test]
    fn create_3x3_matrix() {
        let m = Matrix3::new().with_data([[-3.0, 5.0, 0.0], [1.0, -2.0, -0.7], [0.0, 1.0, 1.0]]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn matrix_equality_3x3() {
        let m1 = Matrix3::new().with_data([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let m2 = Matrix3::new().with_data([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_can_get_2x2_submatrix() {
        let m = Matrix3::new().with_data([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let want = Matrix2::new().with_data([[-3.0, 2.0], [0.0, 6.0]]);
        let got = m.sub_matrix(0, 2);
        assert_eq!(want, got);
    }

    #[test]
    fn calculate_3x3_matrix_minor() {
        let m = Matrix3::new().with_data([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let m_sub = m.sub_matrix(1, 0);
        let det = m_sub.determinant();
        assert!(det.approx_eq(25.0));
        let minor = m.minor(1, 0);
        assert!(minor.approx_eq(25.0));
    }

    #[test]
    fn calculate_3x3_matrix_cofactor() {
        let m = Matrix3::new().with_data([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let minor = m.minor(0, 0);
        assert!(minor.approx_eq(-12.0));
        let cofactor = m.cofactor(0, 0);
        assert!(cofactor.approx_eq(-12.0));

        let minor2 = m.minor(1, 0);
        assert!(minor2.approx_eq(25.0));
        let cofactor2 = m.cofactor(1, 0);
        assert!(cofactor2.approx_eq(-25.0));
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let m = Matrix3::new().with_data([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert!(m.cofactor(0, 0).approx_eq(56.0));
        assert!(m.cofactor(0, 1).approx_eq(12.0));
        assert!(m.cofactor(0, 2).approx_eq(-46.0));
        assert!(m.determinant().approx_eq(-196.0));
    }
}
