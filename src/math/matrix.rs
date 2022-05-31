use std::{
    fmt::Display,
    ops::{Index, IndexMut, Mul},
};

use super::{float_equal, tuple::Tuple};

#[derive(Debug)]
pub struct Matrix {
    pub size: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(size: usize, rows: Vec<Vec<f64>>) -> Self {
        Self { size, data: rows }
    }

    pub fn transpose(&self) -> Matrix {
        let mut data: Vec<Vec<f64>> = vec![];

        for i in 0..self.size {
            data.push(vec![]);
            for _ in 0..self.size {
                data[i].push(0.0);
            }
        }
        for col in 0..self.size {
            for row in 0..self.size {
                data[col][row] = self[row][col];
            }
        }
        Matrix {
            size: self.size,
            data,
        }
    }

    pub fn determinant(&self) -> f64 {
        if self.size == 2 {
            return (self[0][0] * self[1][1]) - (self[1][0] * self[0][1]);
        }
        let mut sum = 0.0;
        for index in 0..self.size {
            let col = self[0][index];
            sum += col * self.cofactor(0, index);
        }
        sum
    }

    pub fn sub_matrix(&self, row_sub: usize, col_sub: usize) -> Matrix {
        if self.size < 2 {
            return Matrix {
                size: 0,
                data: vec![],
            };
        }
        let new_size = self.size - 1;
        let mut data: Vec<Vec<f64>> = vec![];

        for i in 0..new_size {
            data.push(vec![]);
            for _ in 0..new_size {
                data[i].push(0.0);
            }
        }

        let mut y = 0;
        let mut x = 0;
        for row in 0..self.size {
            if row == row_sub {
                continue;
            }
            for col in 0..self.size {
                if col == col_sub {
                    continue;
                }
                data[y][x] = self[row][col];
                x += 1;
            }
            x = 0;
            y += 1;
        }

        Matrix {
            size: new_size,
            data,
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.sub_matrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix> {
        if !self.is_invertible() {
            return None;
        }
        let mut data: Vec<Vec<f64>> = vec![];
        for i in 0..self.size {
            data.push(vec![]);
            for _ in 0..self.size {
                data[i].push(0.0);
            }
        }
        for row in 0..self.size {
            for col in 0..self.size {
                let cofactor = self.cofactor(row, col);
                data[col][row] = cofactor / self.determinant();
            }
        }

        Some(Matrix::new(self.size, data))
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[\n")?;
        for row in 0..self.size {
            f.write_str("  [")?;
            for col in 0..self.size {
                write!(f, " {}, ", self[row][col])?;
            }
            f.write_str("]\n")?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Vec<f64> {
        &mut self.data[index]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        let mut eq = true;
        for i in 0..self.size {
            for j in 0..self.size {
                eq = float_equal(self.data[i][j], other.data[i][j]);
            }
        }
        eq
    }

    fn ne(&self, other: &Self) -> bool {
        return !(self == other);
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut data: Vec<Vec<f64>> = vec![];

        for _ in 0..self.size {
            data.push(vec![]);
        }
        for row in 0..self.size {
            for col in 0..self.size {
                let mut val = 0.0;
                for i in 0..self.size {
                    val += self[row][i] * rhs[i][col];
                }
                data[row].push(val);
            }
        }
        Matrix {
            size: self.size,
            data,
        }
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut vals = [0.0; 4];

        for row in 0..self.size {
            for col in 0..self.size {
                vals[row] += self[row][col] * rhs[col];
            }
        }

        Tuple::from(vals[0], vals[1], vals[2], vals[3])
    }
}

impl Mul<Matrix> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod tests {

    use crate::math::float_equal;

    use super::*;

    #[test]
    fn test_can_create_2x2_matrix() {
        let data = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let m = Matrix::new(2, data);

        assert!(float_equal(m[0][0], -3.0));
        assert!(float_equal(m[0][1], 5.0));
        assert!(float_equal(m[1][0], 1.0));
        assert!(float_equal(m[1][1], -2.0));
    }

    #[test]
    fn test_can_create_3x3_matrix() {
        let data = vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -0.7],
            vec![0.0, 1.0, 1.0],
        ];
        let m = Matrix::new(3, data);

        assert!(float_equal(m[0][0], -3.0));
        assert!(float_equal(m[1][1], -2.0));
        assert!(float_equal(m[2][2], 1.0));
    }

    #[test]
    fn test_can_create_4x4_matrix() {
        let data = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ];
        let m = Matrix::new(4, data);

        assert!(float_equal(m[0][0], 1.0));
        assert!(float_equal(m[0][3], 4.0));
        assert!(float_equal(m[1][0], 5.5));
        assert!(float_equal(m[1][2], 7.5));
        assert!(float_equal(m[2][2], 11.0));
        assert!(float_equal(m[3][0], 13.5));
        assert!(float_equal(m[3][2], 15.5));
    }

    #[test]
    fn test_matrix_equality() {
        let m1 = Matrix::new(
            4,
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0, 8.0],
                vec![9.0, 8.0, 7.0, 6.0],
                vec![5.0, 4.0, 3.0, 2.0],
            ],
        );
        let m2 = Matrix::new(
            4,
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0, 8.0],
                vec![9.0, 8.0, 7.0, 6.0],
                vec![5.0, 4.0, 3.0, 2.0],
            ],
        );
        assert!(m1 == m2);
    }

    #[test]
    fn test_matrix_inequality() {
        let m1 = Matrix::new(
            4,
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0, 8.0],
                vec![9.0, 8.0, 7.0, 6.0],
                vec![5.0, 4.0, 3.0, 2.0],
            ],
        );
        let m2 = Matrix::new(
            4,
            vec![
                vec![2.0, 3.0, 4.0, 5.0],
                vec![6.0, 7.0, 8.0, 9.0],
                vec![8.0, 7.0, 6.0, 5.0],
                vec![4.0, 3.0, 2.0, 1.0],
            ],
        );
        assert!(m1 != m2);
    }

    #[test]
    fn test_can_multiply_2_matrices() {
        let m1 = Matrix::new(
            4,
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0, 8.0],
                vec![9.0, 8.0, 7.0, 6.0],
                vec![5.0, 4.0, 3.0, 2.0],
            ],
        );
        let m2 = Matrix::new(
            4,
            vec![
                vec![-2.0, 1.0, 2.0, 3.0],
                vec![3.0, 2.0, 1.0, -1.0],
                vec![4.0, 3.0, 6.0, 5.0],
                vec![1.0, 2.0, 7.0, 8.0],
            ],
        );
        let want = Matrix::new(
            4,
            vec![
                vec![20.0, 22.0, 50.0, 48.0],
                vec![44.0, 54.0, 114.0, 108.0],
                vec![40.0, 58.0, 110.0, 102.0],
                vec![16.0, 26.0, 46.0, 42.0],
            ],
        );
        let got = m1 * m2;
        assert!(got == want);
    }

    #[test]
    fn test_can_multiply_matrix_by_tuple() {
        let m = Matrix::new(
            4,
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![2.0, 4.0, 4.0, 2.0],
                vec![8.0, 6.0, 4.0, 1.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        );
        let t = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let want = Tuple::new(18.0, 24.0, 33.0, 1.0);
        let got = m * t;
        assert!(want == got);
    }

    #[test]
    fn test_multiply_by_identity_matrix_by_matrix() {
        let ident = Matrix::new(
            4,
            vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        );
        let m = Matrix::new(
            4,
            vec![
                vec![0.0, 1.0, 2.0, 4.0],
                vec![1.0, 2.0, 4.0, 8.0],
                vec![2.0, 4.0, 8.0, 16.0],
                vec![4.0, 8.0, 16.0, 32.0],
            ],
        );
        let got = m * ident;
        let want = Matrix::new(
            4,
            vec![
                vec![0.0, 1.0, 2.0, 4.0],
                vec![1.0, 2.0, 4.0, 8.0],
                vec![2.0, 4.0, 8.0, 16.0],
                vec![4.0, 8.0, 16.0, 32.0],
            ],
        );
        assert!(got == want);
    }

    #[test]
    fn test_multiply_by_identity_matrix_by_tuple() {
        let ident = Matrix::new(
            4,
            vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        );
        let t = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let got = t * ident;
        let want = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert!(got == want);
    }

    #[test]
    fn test_can_transpose_a_matrix() {
        let m = Matrix::new(
            4,
            vec![
                vec![0.0, 9.0, 3.0, 0.0],
                vec![9.0, 8.0, 0.0, 8.0],
                vec![1.0, 8.0, 5.0, 3.0],
                vec![0.0, 0.0, 5.0, 8.0],
            ],
        );
        let want = Matrix::new(
            4,
            vec![
                vec![0.0, 9.0, 1.0, 0.0],
                vec![9.0, 8.0, 8.0, 0.0],
                vec![3.0, 0.0, 5.0, 5.0],
                vec![0.0, 8.0, 3.0, 8.0],
            ],
        );
        let got = m.transpose();
        assert!(got == want);
    }

    #[test]
    fn test_can_transpose_ident_matrix() {
        let ident = Matrix::new(
            4,
            vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        );
        let want = Matrix::new(
            4,
            vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        );
        let got = ident.transpose();
        assert!(got == want);
    }

    #[test]
    fn test_can_get_2x2_matrix_determinate() {
        let m = Matrix::new(2, vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        let want = 17.0;
        let got = m.determinant();
        assert!(float_equal(want, got));
    }

    #[test]
    fn test_can_get_3x3_submatrix() {
        let m = Matrix::new(
            4,
            vec![
                vec![-6.0, 1.0, 1.0, 6.0],
                vec![-8.0, 5.0, 8.0, 6.0],
                vec![-1.0, 0.0, 8.0, 2.0],
                vec![-7.0, 1.0, -1.0, 1.0],
            ],
        );
        let want = Matrix::new(
            3,
            vec![
                vec![-6.0, 1.0, 6.0],
                vec![-8.0, 8.0, 6.0],
                vec![-7.0, -1.0, 1.0],
            ],
        );
        let got = m.sub_matrix(2, 1);
        assert!(want == got);
    }

    #[test]
    fn test_can_get_2x2_submatrix() {
        let m = Matrix::new(
            3,
            vec![
                vec![1.0, 5.0, 0.0],
                vec![-3.0, 2.0, 7.0],
                vec![0.0, 6.0, -3.0],
            ],
        );
        let want = Matrix::new(2, vec![vec![-3.0, 2.0], vec![0.0, 6.0]]);
        let got = m.sub_matrix(0, 2);

        assert_eq!(want, got);
    }

    #[test]
    fn test_can_calculate_3x3_matrix_minor() {
        let m = Matrix::new(
            3,
            vec![
                vec![3.0, 5.0, 0.0],
                vec![2.0, -1.0, -7.0],
                vec![6.0, -1.0, 5.0],
            ],
        );
        let m_sub = m.sub_matrix(1, 0);
        let det = m_sub.determinant();
        assert!(float_equal(det, 25.0));
        let minor = m.minor(1, 0);
        assert!(float_equal(minor, 25.0));
    }

    #[test]
    fn test_can_calculate_3x3_matrix_cofactor() {
        let m = Matrix::new(
            3,
            vec![
                vec![3.0, 5.0, 0.0],
                vec![2.0, -1.0, -7.0],
                vec![6.0, -1.0, 5.0],
            ],
        );
        let minor = m.minor(0, 0);
        assert!(float_equal(minor, -12.0));
        let cofactor = m.cofactor(0, 0);
        assert!(float_equal(cofactor, -12.0));

        let minor2 = m.minor(1, 0);
        assert!(float_equal(minor2, 25.0));
        let cofactor2 = m.cofactor(1, 0);
        assert!(float_equal(cofactor2, -25.0));
    }

    #[test]
    fn test_can_get_3x3_matrix_determinate() {
        let m = Matrix::new(
            3,
            vec![
                vec![1.0, 2.0, 6.0],
                vec![-5.0, 8.0, -4.0],
                vec![2.0, 6.0, 4.0],
            ],
        );
        assert!(float_equal(m.cofactor(0, 0), 56.0));
        assert!(float_equal(m.cofactor(0, 1), 12.0));
        assert!(float_equal(m.cofactor(0, 2), -46.0));
        assert!(float_equal(m.determinant(), -196.0));
    }

    #[test]
    fn test_can_get_4x4_matrix_determinate() {
        let m = Matrix::new(
            4,
            vec![
                vec![-2.0, -8.0, 3.0, 5.0],
                vec![-3.0, 1.0, 7.0, 3.0],
                vec![1.0, 2.0, -9.0, 6.0],
                vec![-6.0, 7.0, 7.0, -9.0],
            ],
        );
        assert!(float_equal(m.cofactor(0, 0), 690.0));
        assert!(float_equal(m.cofactor(0, 1), 447.0));
        assert!(float_equal(m.cofactor(0, 2), 210.0));
        assert!(float_equal(m.cofactor(0, 3), 51.0));
        assert!(float_equal(m.determinant(), -4071.0));
    }

    #[test]
    fn test_can_test_for_invertability() {
        let m = Matrix::new(
            4,
            vec![
                vec![6.0, 4.0, 4.0, 4.0],
                vec![5.0, 5.0, 7.0, 6.0],
                vec![4.0, -9.0, 3.0, -7.0],
                vec![9.0, 1.0, 7.0, -6.0],
            ],
        );
        assert!(float_equal(m.determinant(), -2120.0));
        assert!(m.is_invertible());

        let m2 = Matrix::new(
            4,
            vec![
                vec![-4.0, 2.0, -2.0, -3.0],
                vec![9.0, 6.0, 2.0, 6.0],
                vec![0.0, -5.0, 1.0, -5.0],
                vec![0.0, 0.0, 0.0, 0.0],
            ],
        );
        assert!(float_equal(m2.determinant(), 0.0));
        assert!(!m2.is_invertible());
    }

    #[test]
    fn test_can_calculate_inverse_of_a_matrix_1() {
        let m = Matrix::new(
            4,
            vec![
                vec![-5.0, 2.0, 6.0, -8.0],
                vec![1.0, -5.0, 1.0, 8.0],
                vec![7.0, 7.0, -6.0, -7.0],
                vec![1.0, -3.0, 7.0, 4.0],
            ],
        );
        let det = m.determinant();
        let cofactor1 = m.cofactor(2, 3);
        let cofactor2 = m.cofactor(3, 2);
        let m2 = m.inverse().unwrap_or(m);
        assert!(float_equal(det, 532.0));
        assert!(float_equal(cofactor1, -160.0));
        assert!(float_equal(m2[3][2], -(160.0 / 532.0)));
        assert!(float_equal(cofactor2, 105.0));
        assert!(float_equal(m2[2][3], 105.0 / 532.0));

        let want = Matrix::new(
            4,
            vec![
                vec![0.21805, 0.45113, 0.24060, -0.04511],
                vec![-0.80827, -1.45677, -0.44361, 0.52068],
                vec![-0.07895, -0.22368, -0.05263, 0.19737],
                vec![-0.52256, -0.81391, -0.30075, 0.30639],
            ],
        );
        assert_eq!(want, m2);
    }

    #[test]
    fn test_can_calculate_inverse_of_a_matrix_2() {
        let m = Matrix::new(
            4,
            vec![
                vec![8.0, -5.0, 9.0, 2.0],
                vec![7.0, 5.0, 6.0, 1.0],
                vec![-6.0, 0.0, 9.0, 6.0],
                vec![-3.0, 0.0, -9.0, -4.0],
            ],
        );
        let m2 = m.inverse().unwrap_or(m);
        let want = Matrix::new(
            4,
            vec![
                vec![-0.15385, -0.15385, -0.28205, -0.53846],
                vec![-0.07692, 0.12308, 0.02564, 0.03077],
                vec![0.35897, 0.35897, 0.43590, 0.92308],
                vec![-0.69231, -0.69231, -0.76923, -1.92308],
            ],
        );
        assert!(want == m2);
    }

    #[test]
    fn test_can_calculate_inverse_of_a_matrix_3() {
        let m = Matrix::new(
            4,
            vec![
                vec![9.0, 3.0, 0.0, 9.0],
                vec![-5.0, -2.0, -6.0, -3.0],
                vec![-4.0, 9.0, 6.0, 4.0],
                vec![-7.0, 6.0, 6.0, 2.0],
            ],
        );
        let m2 = m.inverse().unwrap_or(m);
        let want = Matrix::new(
            4,
            vec![
                vec![-0.04074, -0.07778, 0.14444, -0.22222],
                vec![-0.07778, 0.03333, 0.36667, -0.33333],
                vec![-0.02901, -0.14630, -0.10926, 0.12963],
                vec![0.17778, 0.06667, -0.26667, 0.33333],
            ],
        );
        assert!(want == m2);
    }

    #[test]
    fn test_multiply_product_by_inverse() {
        let m_a = Matrix::new(
            4,
            vec![
                vec![3.0, -9.0, 7.0, 3.0],
                vec![3.0, -8.0, 2.0, -9.0],
                vec![-4.0, 4.0, 4.0, 1.0],
                vec![-6.0, 5.0, -1.0, 1.0],
            ],
        );
        let m_a_clone = Matrix::new(
            4,
            vec![
                vec![3.0, -9.0, 7.0, 3.0],
                vec![3.0, -8.0, 2.0, -9.0],
                vec![-4.0, 4.0, 4.0, 1.0],
                vec![-6.0, 5.0, -1.0, 1.0],
            ],
        );
        let m_b = Matrix::new(
            4,
            vec![
                vec![8.0, 2.0, 2.0, 2.0],
                vec![3.0, -1.0, 7.0, 0.0],
                vec![7.0, 0.0, 5.0, 4.0],
                vec![6.0, -2.0, 0.0, 5.0],
            ],
        );
        let m_b_clone = Matrix::new(
            4,
            vec![
                vec![8.0, 2.0, 2.0, 2.0],
                vec![3.0, -1.0, 7.0, 0.0],
                vec![7.0, 0.0, 5.0, 4.0],
                vec![6.0, -2.0, 0.0, 5.0],
            ],
        );
        let m_c = m_a * m_b;

        assert_eq!(m_a_clone, m_c * m_b_clone.inverse().unwrap());
    }
}
