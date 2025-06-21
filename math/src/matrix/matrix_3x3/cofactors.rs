use crate::matrix::Minor;
use crate::matrix::matrix_2x2::Matrix2x2;
use crate::matrix::matrix_3x3::Matrix3x3;

impl Matrix3x3 {
    pub fn submatrix(&self, row: usize, column: usize) -> Matrix2x2 {
        let mut result = Matrix2x2::identity();
        let mut row_out = 0;
        for r in 0..3 {
            if r == row {
                continue;
            }
            let mut column_out = 0;
            for c in 0..3 {
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

impl Minor for Matrix3x3 {
    fn minor(&self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }
}

#[cfg(test)]
mod matrix_3x3_sub_matrix_tests {
    use super::*;

    #[test]
    fn submatrix() {
        let a = Matrix3x3::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, 3.0]]);

        assert_eq!(Matrix2x2::new([[-3.0, 2.0], [0.0, 6.0]]), a.submatrix(0, 2));
    }
}

#[cfg(test)]
mod matrix_3x3_minor_and_cofactor_tests {
    use super::*;
    use crate::matrix::Cofactor;

    #[test]
    fn minors() {
        let a = Matrix3x3::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_eq!(25.0, a.minor(1, 0));
        assert_eq!(-12.0, a.minor(0, 0));
    }

    #[test]
    fn cofactors() {
        let a = Matrix3x3::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_eq!(-12.0, a.cofactor(0, 0));
        assert_eq!(-25.0, a.cofactor(1, 0));
    }
}
