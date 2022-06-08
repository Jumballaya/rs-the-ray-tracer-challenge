use std::ops::{Index, IndexMut};

use crate::math::epsilon::ApproxEq;

#[derive(Clone, Copy, Debug)]
pub struct Matrix2 {
    data: [[f64; 2]; 2],
}

impl Matrix2 {
    pub fn new() -> Self {
        Matrix2 {
            data: [[0.0; 2]; 2],
        }
    }

    pub fn with_data(mut self, data: [[f64; 2]; 2]) -> Self {
        self.data = data;
        self
    }

    pub fn determinant(&self) -> f64 {
        (self[0][0] * self[1][1]) - (self[1][0] * self[0][1])
    }
}

impl IndexMut<usize> for Matrix2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<usize> for Matrix2 {
    type Output = [f64; 2];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..2 {
            for j in 0..2 {
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

    use crate::math::epsilon::ApproxEq;

    use super::Matrix2;

    #[test]
    fn create_2x2_matrix() {
        let m = Matrix2::new().with_data([[-3.0, 5.0], [1.0, -2.0]]);
        assert!(m[0][0].approx_eq(-3.0));
        assert!(m[0][1].approx_eq(5.0));
        assert!(m[1][0].approx_eq(1.0));
        assert!(m[1][1].approx_eq(-2.0));
    }

    #[test]
    fn matrix_equality_2x2() {
        let m1 = Matrix2::new().with_data([[1.0, 2.0], [3.0, 4.0]]);
        let m2 = Matrix2::new().with_data([[1.0, 2.0], [3.0, 4.0]]);
        assert_eq!(m1, m2);
    }

    #[test]
    fn determinant_of_2x2_matrix() {
        let m = Matrix2::new().with_data([[1.0, 5.0], [-3.0, 2.0]]);
        let want = 17.0;
        let got = m.determinant();
        assert!(want.approx_eq(got));
    }
}
