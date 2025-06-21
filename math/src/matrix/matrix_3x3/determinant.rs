use crate::matrix::matrix_3x3::Matrix3x3;
use crate::matrix::{Cofactor, Determinant};

impl Determinant for Matrix3x3 {
    fn determinant(&self) -> f32 {
        self[0][0] * self.cofactor(0, 0)
            + self[0][1] * self.cofactor(0, 1)
            + self[0][2] * self.cofactor(0, 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::matrix_3x3::Matrix3x3;
    use crate::matrix::{Cofactor, Determinant};

    #[test]
    fn a() {
        let a = Matrix3x3::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }
}
