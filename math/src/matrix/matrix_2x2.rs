use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut, Mul};

///
/// Row-major 2x2 matrix
///
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Matrix2x2([[f32; 2]; 2]);

impl Matrix2x2 {
    pub fn determinant(&self) -> f32 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Matrix2x2 {
    pub fn transpose(&self) -> Self {
        let mut result = Self::identity();
        for r in 0..2 {
            for c in 0..2 {
                result[c][r] = self[r][c]
            }
        }
        result
    }
}

impl Matrix2x2 {
    pub fn new(data: [[f32; 2]; 2]) -> Self {
        Self(data)
    }
    pub fn identity() -> Self {
        let mut x = [[0.0; 2]; 2];
        for i in 0..2 {
            x[i][i] = 1.0;
        }
        Self(x)
    }
}

impl Deref for Matrix2x2 {
    type Target = [[f32; 2]; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Matrix2x2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Matrix2x2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for r in 0..2 {
            write!(f, "[")?;
            for c in 0..2 {
                write!(f, "{}", self[r][c])?;
                if c < 1 {
                    write!(f, " ")?;
                }
            }
            write!(f, "]")?;
        }
        write!(f, "]")
    }
}

impl Mul for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix2x2::identity();
        for r in 0..2 {
            for c in 0..2 {
                result[r][c] = self[r][0] * rhs[0][c]
                    + self[r][1] * rhs[1][c];
            }
        }
        result
    }
}

#[cfg(test)]
mod matrix_2x2_tests {
    use super::*;

    #[test]
    fn identity() {
        let i = Matrix2x2::identity();
        assert_eq!(i[0][0], 1.0);
        assert_eq!(i[1][1], 1.0);
        assert_eq!(i[0][1], 0.0);
        assert_eq!(i[1][0], 0.0);
    }

    #[test]
    fn identity_display() {
        assert_eq!("[[1 0][0 1]]", format!("{}", Matrix2x2::identity()));
    }

    #[test]
    fn update_matrix() {
        let mut i = Matrix2x2::identity();
        i[1][0] = 2.0;
        assert_eq!(i[1][0], 2.0);
        assert_eq!("[[1 0][2 1]]", format!("{}", i));
    }

    #[test]
    fn multiply_matrix() {
        let a = Matrix2x2::new([
            [1.0, 2.0],
            [5.0, 6.0]
        ]);
        let b = Matrix2x2::new([
            [-2.0, 1.0],
            [3.0, 2.0]
        ]);

        assert_eq!(
            Matrix2x2::new([
                [4.0, 5.0],
                [8.0, 17.0]
            ]),
            a * b
        );
    }

    #[test]
    fn multiply_matrix_by_identity() {
        let a = Matrix2x2::new([[1.0, 2.0], [3.0, 4.0]]);

        assert_eq!(a, a * Matrix2x2::identity());
    }

    #[test]
    fn transpose_matrix() {
        let a = Matrix2x2::new([[1.0, 2.0], [3.0, 4.0]]);

        assert_eq!(Matrix2x2::new([[1.0, 3.0], [2.0, 4.0],]), a.transpose());
    }

    #[test]
    fn transpose_identity() {
        assert_eq!(Matrix2x2::identity().transpose(), Matrix2x2::identity());
    }

    #[test]
    fn determinant() {
        assert_eq!(17.0, Matrix2x2::new([[1.0,5.0],[-3.0,2.0]]).determinant());
    }
}
