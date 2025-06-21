mod cofactors;
mod determinant;

use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut, Mul};

///
/// Row-major 3x3 matrix
///
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Matrix3x3([[f32; 3]; 3]);

impl Matrix3x3 {
    pub fn transpose(&self) -> Self {
        let mut result = Self::empty();
        for r in 0..3 {
            for c in 0..3 {
                result[c][r] = self[r][c]
            }
        }
        result
    }
}

impl Matrix3x3 {
    pub fn new(data: [[f32; 3]; 3]) -> Self {
        Self(data)
    }

    pub fn empty() -> Self {
        Self([[0.0; 3]; 3])
    }

    pub fn identity() -> Self {
        let mut x = Self::empty();
        for i in 0..3 {
            x[i][i] = 1.0;
        }
        x
    }
}

impl Deref for Matrix3x3 {
    type Target = [[f32; 3]; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Matrix3x3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Matrix3x3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for r in 0..3 {
            write!(f, "[")?;
            for c in 0..3 {
                write!(f, "{}", self[r][c])?;
                if c < 2 {
                    write!(f, " ")?;
                }
            }
            write!(f, "]")?;
        }
        write!(f, "]")
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix3x3::empty();
        for r in 0..3 {
            for c in 0..3 {
                result[r][c] =
                    self[r][0] * rhs[0][c] + self[r][1] * rhs[1][c] + self[r][2] * rhs[2][c];
            }
        }
        result
    }
}

#[cfg(test)]
mod matrix_3x3_tests {
    use super::*;

    #[test]
    fn identity() {
        let i = Matrix3x3::identity();
        assert_eq!(i[0][0], 1.0);
        assert_eq!(i[1][1], 1.0);
        assert_eq!(i[2][2], 1.0);
        assert_eq!(i[0][1], 0.0);
        assert_eq!(i[0][2], 0.0);
        assert_eq!(i[1][0], 0.0);
        assert_eq!(i[1][2], 0.0);
        assert_eq!(i[2][0], 0.0);
        assert_eq!(i[2][1], 0.0);
    }

    #[test]
    fn identity_display() {
        assert_eq!(
            "[[1 0 0][0 1 0][0 0 1]]",
            format!("{}", Matrix3x3::identity())
        );
    }

    #[test]
    fn empty_display() {
        assert_eq!(
            "[[0 0 0][0 0 0][0 0 0]]",
            format!("{}", Matrix3x3::empty())
        );
    }

    #[test]
    fn update_matrix() {
        let mut i = Matrix3x3::identity();
        i[1][0] = 2.0;
        assert_eq!(i[1][0], 2.0);
        assert_eq!("[[1 0 0][2 1 0][0 0 1]]", format!("{}", i));
    }

    #[test]
    fn multiply_matrix() {
        let a = Matrix3x3::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]]);
        let b = Matrix3x3::new([[-2.0, 1.0, 2.0], [3.0, 2.0, 1.0], [3.0, 3.0, 6.0]]);

        assert_eq!(
            Matrix3x3::new([[13.0, 14.0, 22.0], [29.0, 38.0, 58.0], [27.0, 46.0, 68.0]]),
            a * b
        );
    }

    #[test]
    fn multiply_matrix_by_identity() {
        let a = Matrix3x3::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]]);

        assert_eq!(a, a * Matrix3x3::identity());
    }

    #[test]
    fn transpose_matrix() {
        let a = Matrix3x3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);

        assert_eq!(
            Matrix3x3::new([[1.0, 4.0, 7.0], [2.0, 5.0, 8.0], [3.0, 6.0, 9.0],]),
            a.transpose()
        );
    }

    #[test]
    fn transpose_identity() {
        assert_eq!(Matrix3x3::identity().transpose(), Matrix3x3::identity());
    }
}
