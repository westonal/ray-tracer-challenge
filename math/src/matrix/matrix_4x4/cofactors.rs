use crate::matrix::matrix_3x3::Matrix3x3;
use crate::matrix::matrix_4x4::Matrix4x4;
use crate::matrix::{Cofactor, Determinant, Minor};

impl Matrix4x4 {
    pub fn submatrix(&self, row: usize, column: usize) -> Matrix3x3 {
        let mut result = Matrix3x3::identity();
        let mut row_out = 0;
        for r in 0..4 {
            if r == row {
                continue;
            }
            let mut column_out = 0;
            for c in 0..4 {
                if c == column {
                    continue;
                }
                result[row_out][column_out] = self[r][c];
                column_out += 1;
            }
            row_out += 1;
        }
        result
    }
}

impl Minor for Matrix4x4 {
    fn minor(&self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }
}

impl Determinant for Matrix4x4 {
    fn determinant(&self) -> f32 {
        self[0][0] * self.cofactor(0, 0)
            + self[0][1] * self.cofactor(0, 1)
            + self[0][2] * self.cofactor(0, 2)
            + self[0][3] * self.cofactor(0, 3)
    }
}

#[cfg(test)]
mod matrix_4x4_sub_matrix_tests {
    use super::*;

    #[test]
    fn submatrix() {
        let a = Matrix4x4::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        assert_eq!(
            Matrix3x3::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0],]),
            a.submatrix(2, 1)
        );
    }
}

#[cfg(test)]
mod matrix_4x4_cofactors_tests {
    use super::*;

    fn matrix_under_test() -> Matrix4x4 {
        Matrix4x4::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ])
    }

    #[test]
    fn minors() {
        assert_eq!(matrix_under_test().minor(0, 0), 690.0);
        assert_eq!(matrix_under_test().minor(0, 1), -447.0);
        assert_eq!(matrix_under_test().minor(0, 2), 210.0);
        assert_eq!(matrix_under_test().minor(0, 3), -51.0);
    }

    #[test]
    fn cofactors() {
        assert_eq!(matrix_under_test().cofactor(0, 0), 690.0);
        assert_eq!(matrix_under_test().cofactor(0, 1), 447.0);
        assert_eq!(matrix_under_test().cofactor(0, 2), 210.0);
        assert_eq!(matrix_under_test().cofactor(0, 3), 51.0);
    }

    #[test]
    fn determinant() {
        assert_eq!(matrix_under_test().determinant(), -4071.0);
    }
}
