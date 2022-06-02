use std::{
    fmt::Display,
    ops::{Index, IndexMut, Mul},
};

use super::{float_equal, tuple::Tuple};

#[derive(Debug, Copy, Clone)]
pub enum Transformation {
    None,                                // Identity Matrix
    Translate(f64, f64, f64),            // Move by (x,y,z)
    Scale(f64, f64, f64),                // Scale by (x,y,z)
    RotateX(f64),                        // Rotate by (radians) in X axis
    RotateY(f64),                        // Rotate by (radians) in X axis
    RotateZ(f64),                        // Rotate by (radians) in X axis
    Shear(f64, f64, f64, f64, f64, f64), // Shear by (x:y, x:z, y:x, y:z, z:x, z:y)
}

#[derive(Debug, Clone)]
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

    pub fn inverse(&self) -> Matrix {
        if !self.is_invertible() {
            return self.clone();
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

        Matrix::new(self.size, data)
    }

    pub fn transform(tform: Transformation) -> Matrix {
        match tform {
            Transformation::None => Matrix::identity_matrix(4),
            Transformation::Translate(x, y, z) => Matrix {
                size: 4,
                data: vec![
                    vec![1.0, 0.0, 0.0, x],
                    vec![0.0, 1.0, 0.0, y],
                    vec![0.0, 0.0, 1.0, z],
                    vec![0.0, 0.0, 0.0, 1.0],
                ],
            },
            Transformation::Scale(x, y, z) => Matrix {
                size: 4,
                data: vec![
                    vec![x, 0.0, 0.0, 0.0],
                    vec![0.0, y, 0.0, 0.0],
                    vec![0.0, 0.0, z, 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ],
            },
            Transformation::RotateX(r) => Matrix {
                size: 4,
                data: vec![
                    vec![1.0, 0.0, 0.0, 0.0],
                    vec![0.0, r.cos(), -r.sin(), 0.0],
                    vec![0.0, r.sin(), r.cos(), 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ],
            },
            Transformation::RotateY(r) => Matrix {
                size: 4,
                data: vec![
                    vec![r.cos(), 0.0, r.sin(), 0.0],
                    vec![0.0, 1.0, 0.0, 0.0],
                    vec![-r.sin(), 0.0, r.cos(), 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ],
            },
            Transformation::RotateZ(r) => Matrix {
                size: 4,
                data: vec![
                    vec![r.cos(), -r.sin(), 0.0, 0.0],
                    vec![r.sin(), r.cos(), 0.0, 0.0],
                    vec![0.0, 0.0, 1.0, 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ],
            },
            Transformation::Shear(x_y, x_z, y_x, y_z, z_x, z_y) => Matrix {
                size: 4,
                data: vec![
                    vec![1.0, x_y, x_z, 0.0],
                    vec![y_x, 1.0, y_z, 0.0],
                    vec![z_x, z_y, 1.0, 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ],
            },
        }
    }

    pub fn transform_chain(tforms: &[Transformation]) -> Matrix {
        let mut m = Matrix::identity_matrix(4);
        for i in 0..tforms.len() {
            let t = tforms[tforms.len() - i - 1];
            m = m * Matrix::transform(t);
        }
        m
    }

    pub fn identity_matrix(size: usize) -> Matrix {
        let mut data: Vec<Vec<f64>> = vec![];
        for i in 0..size {
            data.push(vec![]);
            for j in 0..size {
                if i == j {
                    data[i].push(1.0);
                } else {
                    data[i].push(0.0);
                }
            }
        }
        Matrix { size, data }
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

impl Mul<&Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Self::Output {
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

impl Mul<&Matrix> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut vals = [0.0; 4];

        for row in 0..rhs.size {
            for col in 0..rhs.size {
                vals[row] += rhs[row][col] * self[col];
            }
        }

        Tuple::from(vals[0], vals[1], vals[2], vals[3])
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use crate::math::float_equal;

    use super::*;

    #[test]
    fn matrix_can_create_2x2_matrix() {
        let data = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let m = Matrix::new(2, data);

        assert!(float_equal(m[0][0], -3.0));
        assert!(float_equal(m[0][1], 5.0));
        assert!(float_equal(m[1][0], 1.0));
        assert!(float_equal(m[1][1], -2.0));
    }

    #[test]
    fn matrix_can_create_3x3_matrix() {
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
    fn matrix_can_create_4x4_matrix() {
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
    fn matrix_matrix_equality() {
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
    fn matrix_matrix_inequality() {
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
    fn matrix_can_multiply_2_matrices() {
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
    fn matrix_can_multiply_matrix_by_tuple() {
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
    fn matrix_can_create_idenity_matrix() {
        let ident2 = Matrix::new(2, vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
        let ident3 = Matrix::new(
            3,
            vec![
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0],
            ],
        );
        let ident4 = Matrix::new(
            4,
            vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        );
        assert_eq!(ident2, Matrix::identity_matrix(2));
        assert_eq!(ident3, Matrix::identity_matrix(3));
        assert_eq!(ident4, Matrix::identity_matrix(4));
    }

    #[test]
    fn matrix_multiply_by_identity_matrix_by_matrix() {
        let ident = Matrix::identity_matrix(4);
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
    fn matrix_multiply_by_identity_matrix_by_tuple() {
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
    fn matrix_can_transpose_a_matrix() {
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
    fn matrix_can_transpose_ident_matrix() {
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
    fn matrix_can_get_2x2_matrix_determinate() {
        let m = Matrix::new(2, vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        let want = 17.0;
        let got = m.determinant();
        assert!(float_equal(want, got));
    }

    #[test]
    fn matrix_can_get_3x3_submatrix() {
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
    fn matrix_can_get_2x2_submatrix() {
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
    fn matrix_can_calculate_3x3_matrix_minor() {
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
    fn matrix_can_calculate_3x3_matrix_cofactor() {
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
    fn matrix_can_get_3x3_matrix_determinate() {
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
    fn matrix_can_get_4x4_matrix_determinate() {
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
    fn matrix_can_matrix_for_invertability() {
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
    fn matrix_can_calculate_inverse_of_a_matrix_1() {
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
        let m2 = m.inverse();
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
    fn matrix_can_calculate_inverse_of_a_matrix_2() {
        let m = Matrix::new(
            4,
            vec![
                vec![8.0, -5.0, 9.0, 2.0],
                vec![7.0, 5.0, 6.0, 1.0],
                vec![-6.0, 0.0, 9.0, 6.0],
                vec![-3.0, 0.0, -9.0, -4.0],
            ],
        );
        let m2 = m.inverse();
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
    fn matrix_can_calculate_inverse_of_a_matrix_3() {
        let m = Matrix::new(
            4,
            vec![
                vec![9.0, 3.0, 0.0, 9.0],
                vec![-5.0, -2.0, -6.0, -3.0],
                vec![-4.0, 9.0, 6.0, 4.0],
                vec![-7.0, 6.0, 6.0, 2.0],
            ],
        );
        let m2 = m.inverse();
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
    fn matrix_multiply_product_by_inverse() {
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

        assert_eq!(m_a_clone, m_c * m_b_clone.inverse());
    }

    #[test]
    fn matrix_create_translation_matrix() {
        let tx = Transformation::Translate(5.0, -3.0, 2.0);
        let p = Tuple::new_point(-3.0, 4.0, 5.0);
        let want = Tuple::new_point(2.0, 1.0, 7.0);
        let got = Matrix::transform(tx) * p;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_can_multiply_by_inverse_of_translation() {
        let tx = Transformation::Translate(5.0, -3.0, 2.0);
        let p = Tuple::new_point(-3.0, 4.0, 5.0);
        let want = Tuple::new_point(-8.0, 7.0, 3.0);
        let inverse = Matrix::transform(tx).inverse();
        let got = inverse * p;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_translation_does_not_effect_vectors() {
        let tx = Transformation::Translate(5.0, -3.0, 2.0);
        let v = Tuple::new_vector(-3.0, 4.0, 5.0);
        let want = Tuple::new_vector(-3.0, 4.0, 5.0);
        let got = Matrix::transform(tx) * v;
        assert_eq!(want, got);
    }

    #[test]
    fn matrix_can_apply_scaling_to_point() {
        let tx = Transformation::Scale(2.0, 3.0, 4.0);
        let p = Tuple::new_point(-4.0, 6.0, 8.0);
        let want = Tuple::new_point(-8.0, 18.0, 32.0);
        let got = Matrix::transform(tx) * p;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_can_apply_scaling_to_vector() {
        let tx = Transformation::Scale(2.0, 3.0, 4.0);
        let v = Tuple::new_vector(-4.0, 6.0, 8.0);
        let want = Tuple::new_vector(-8.0, 18.0, 32.0);
        let got = Matrix::transform(tx) * v;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_can_multiply_by_inverse_of_scaling_matrix() {
        let tx = Transformation::Scale(2.0, 3.0, 4.0);
        let v = Tuple::new_vector(-4.0, 6.0, 8.0);
        let want = Tuple::new_vector(-2.0, 2.0, 2.0);

        let inverse = Matrix::transform(tx).inverse();
        let got = inverse * v;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_reflection_is_scaling_by_a_negative_value() {
        let tx = Transformation::Scale(-1.0, 1.0, 1.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let want = Tuple::new_point(-2.0, 3.0, 4.0);
        let got = Matrix::transform(tx) * p;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_rotate_point_around_x_axis() {
        let p1 = Tuple::new_point(0.0, 1.0, 0.0);
        let p2 = Tuple::new_point(0.0, 1.0, 0.0);
        let tx_half_quarter = Transformation::RotateX(PI / 4.0);
        let tx_full_quarter = Transformation::RotateX(PI / 2.0);

        let want1 = Tuple::new_point(0.0, (2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0);
        let want2 = Tuple::new_point(0.0, 0.0, 1.0);

        let got1 = Matrix::transform(tx_half_quarter) * p1;
        let got2 = Matrix::transform(tx_full_quarter) * p2;

        assert_eq!(got1, want1);
        assert_eq!(got2, want2);
    }

    #[test]
    fn matrix_inverse_x_rotation_rotates_opposite_direction() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let tx = Transformation::RotateX(PI / 4.0);
        let inv = Matrix::transform(tx).inverse();
        let want = Tuple::new_point(0.0, (2.0 as f64).sqrt() / 2.0, -((2.0 as f64).sqrt() / 2.0));
        let got = inv * p;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_rotate_point_around_y_axis() {
        let p1 = Tuple::new_point(0.0, 0.0, 1.0);
        let p2 = Tuple::new_point(0.0, 0.0, 1.0);
        let tx_half_quarter = Transformation::RotateY(PI / 4.0);
        let tx_full_quarter = Transformation::RotateY(PI / 2.0);

        let want1 = Tuple::new_point((2.0 as f64).sqrt() / 2.0, 0.0, (2.0 as f64).sqrt() / 2.0);
        let want2 = Tuple::new_point(1.0, 0.0, 0.0);

        let got1 = Matrix::transform(tx_half_quarter) * p1;
        let got2 = Matrix::transform(tx_full_quarter) * p2;

        assert_eq!(got1, want1);
        assert_eq!(got2, want2);
    }

    #[test]
    fn matrix_rotate_point_around_z_axis() {
        let p1 = Tuple::new_point(0.0, 1.0, 0.0);
        let p2 = Tuple::new_point(0.0, 1.0, 0.0);
        let tx_half_quarter = Transformation::RotateZ(PI / 4.0);
        let tx_full_quarter = Transformation::RotateZ(PI / 2.0);

        let want1 = Tuple::new_point(-((2.0 as f64).sqrt()) / 2.0, (2.0 as f64).sqrt() / 2.0, 0.0);
        let want2 = Tuple::new_point(-1.0, 0.0, 0.0);

        let got1 = Matrix::transform(tx_half_quarter) * p1;
        let got2 = Matrix::transform(tx_full_quarter) * p2;

        assert_eq!(got1, want1);
        assert_eq!(got2, want2);
    }

    #[test]
    fn matrix_shear_point_x_y() {
        let tx = Transformation::Shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let want = Tuple::new_point(5.0, 3.0, 4.0);
        let got = Matrix::transform(tx) * p;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_shear_point_x_z() {
        let tx = Transformation::Shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let want = Tuple::new_point(6.0, 3.0, 4.0);
        let got = Matrix::transform(tx) * p;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_shear_point_y_x() {
        let tx = Transformation::Shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let want = Tuple::new_point(2.0, 5.0, 4.0);
        let got = Matrix::transform(tx) * p;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_shear_point_y_z() {
        let tx = Transformation::Shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let want = Tuple::new_point(2.0, 7.0, 4.0);
        let got = Matrix::transform(tx) * p;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_shear_point_z_x() {
        let tx = Transformation::Shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let want = Tuple::new_point(2.0, 3.0, 6.0);
        let got = Matrix::transform(tx) * p;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_shear_point_z_y() {
        let tx = Transformation::Shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let want = Tuple::new_point(2.0, 3.0, 7.0);
        let got = Matrix::transform(tx) * p;
        assert_eq!(got, want);
    }

    #[test]
    fn matrix_can_apply_multiple_transforms_in_sequence() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let tx_a = Matrix::transform(Transformation::RotateX(PI / 2.0));
        let tx_b = Matrix::transform(Transformation::Scale(5.0, 5.0, 5.0));
        let tx_c = Matrix::transform(Transformation::Translate(10.0, 5.0, 7.0));

        let p2 = tx_a * p;
        let want1 = Tuple::new_point(1.0, -1.0, 0.0);
        assert_eq!(&want1, &p2);

        let p3 = tx_b * p2;
        let want2 = Tuple::new_point(5.0, -5.0, 0.0);
        assert_eq!(&want2, &p3);

        let p4 = tx_c * p3;
        let want3 = Tuple::new_point(15.0, 0.0, 7.0);
        assert_eq!(want3, p4);
    }

    #[test]
    fn matrix_can_apply_chained_transforms() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let tx_a = Transformation::RotateX(PI / 2.0);
        let tx_b = Transformation::Scale(5.0, 5.0, 5.0);
        let tx_c = Transformation::Translate(10.0, 5.0, 7.0);

        let tx = Matrix::transform_chain(&[tx_a, tx_b, tx_c]);

        let got = tx * p;
        let want = Tuple::new_point(15.0, 0.0, 7.0);
        assert_eq!(want, got);
    }
}
