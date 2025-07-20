mod builder_macro;
mod cofactors;
mod inversion;
mod rotation;
mod scaling;
mod shearing;
mod transformation_chain_tests;
mod translation;

pub use rotation::Matrix4x4Rotation;
pub use scaling::Matrix4x4Scale;
pub use shearing::Matrix4x4Shear;
pub use translation::Matrix4x4Translate;

use crate::tuple::Tuple;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut, Div, Mul};

///
/// Row-major 4x4 matrix
///
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Matrix4x4([[f32; 4]; 4]);

impl Matrix4x4 {
    pub fn transpose(&self) -> Self {
        let mut result = Self::empty();
        for r in 0..4 {
            for c in 0..4 {
                result[c][r] = self[r][c]
            }
        }
        result
    }
}

impl Matrix4x4 {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Self(data)
    }

    pub fn empty() -> Self {
        Self([[0.0; 4]; 4])
    }

    pub fn identity() -> Self {
        let mut x = Self::empty();
        for i in 0..4 {
            x[i][i] = 1.0;
        }
        x
    }
}

impl Deref for Matrix4x4 {
    type Target = [[f32; 4]; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Matrix4x4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Matrix4x4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for r in 0..4 {
            write!(f, "[")?;
            for c in 0..4 {
                write!(f, "{}", self[r][c])?;
                if c < 3 {
                    write!(f, " ")?;
                }
            }
            write!(f, "]")?;
        }
        write!(f, "]")
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix4x4::empty();
        for r in 0..4 {
            for c in 0..4 {
                result[r][c] = self[r][0] * rhs[0][c]
                    + self[r][1] * rhs[1][c]
                    + self[r][2] * rhs[2][c]
                    + self[r][3] * rhs[3][c];
            }
        }
        result
    }
}

impl Mul<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = Matrix4x4::empty();
        for r in 0..4 {
            for c in 0..4 {
                result[r][c] = self[r][c] * rhs;
            }
        }
        result
    }
}

impl Div<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = Matrix4x4::empty();
        for r in 0..4 {
            for c in 0..4 {
                result[r][c] = self[r][c] / rhs;
            }
        }
        result
    }
}

impl Mul<Tuple> for Matrix4x4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut res = [0.0; 4];
        for r in 0..4 {
            res[r] =
                self[r][0] * rhs.x + self[r][1] * rhs.y + self[r][2] * rhs.z + self[r][3] * rhs.w;
        }
        Tuple::new(res[0], res[1], res[2], res[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        let i = Matrix4x4::identity();
        assert_eq!(i[0][0], 1.);
        assert_eq!(i[1][1], 1.);
        assert_eq!(i[2][2], 1.);
        assert_eq!(i[3][3], 1.);
        assert_eq!(i[0][1], 0.);
        assert_eq!(i[0][2], 0.);
        assert_eq!(i[0][3], 0.);
        assert_eq!(i[1][0], 0.);
        assert_eq!(i[1][2], 0.);
        assert_eq!(i[1][3], 0.);
        assert_eq!(i[2][0], 0.);
        assert_eq!(i[2][1], 0.);
        assert_eq!(i[2][3], 0.);
        assert_eq!(i[3][0], 0.);
        assert_eq!(i[3][1], 0.);
        assert_eq!(i[3][2], 0.);
    }

    #[test]
    fn identity_display() {
        assert_eq!(
            "[[1 0 0 0][0 1 0 0][0 0 1 0][0 0 0 1]]",
            format!("{}", Matrix4x4::identity())
        );
    }

    #[test]
    fn empty_display() {
        assert_eq!(
            "[[0 0 0 0][0 0 0 0][0 0 0 0][0 0 0 0]]",
            format!("{}", Matrix4x4::empty())
        );
    }

    #[test]
    fn update_matrix() {
        let mut i = Matrix4x4::identity();
        i[1][0] = 2.0;
        assert_eq!(i[1][0], 2.0);
        assert_eq!("[[1 0 0 0][2 1 0 0][0 0 1 0][0 0 0 1]]", format!("{}", i));
    }

    #[test]
    fn multiply_matrix() {
        let a = Matrix4x4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        let b = Matrix4x4::new([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);

        assert_eq!(
            Matrix4x4::new([
                [20., 22., 50., 48.],
                [44., 54., 114., 108.],
                [40., 58., 110., 102.],
                [16., 26., 46., 42.]
            ]),
            a * b
        );
    }

    #[test]
    fn multiply_matrix_by_scalar() {
        let a = Matrix4x4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        assert_eq!(
            Matrix4x4::new([
                [5., 10., 15., 20.],
                [25., 30., 35., 40.],
                [45., 40., 35., 30.],
                [25., 20., 15., 10.]
            ]),
            a * 5.
        );
    }

    #[test]
    fn divide_matrix_by_scalar() {
        let a = Matrix4x4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        assert_eq!(
            Matrix4x4::new([
                [0.2, 0.4, 0.6, 0.8],
                [1., 1.2, 1.4, 1.6],
                [1.8, 1.6, 1.4, 1.2],
                [1., 0.8, 0.6, 0.4]
            ]),
            a / 5.
        );
    }

    #[test]
    fn multiply_matrix_by_identity() {
        let a = Matrix4x4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        assert_eq!(a, a * Matrix4x4::identity());
    }

    #[test]
    fn multiply_matrix_by_tuple() {
        let a = Matrix4x4::new([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        ]);

        assert_eq!(
            Tuple::new(18., 24., 33., 1.),
            a * Tuple::new(1., 2., 3., 1.)
        );
    }

    #[test]
    fn transpose_matrix() {
        let a = Matrix4x4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        assert_eq!(
            Matrix4x4::new([
                [1., 5., 9., 5.],
                [2., 6., 8., 4.],
                [3., 7., 7., 3.],
                [4., 8., 6., 2.],
            ]),
            a.transpose()
        );
    }

    #[test]
    fn transpose_identity() {
        assert_eq!(Matrix4x4::identity().transpose(), Matrix4x4::identity());
    }
}
