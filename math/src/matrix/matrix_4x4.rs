use crate::matrix::matrix_3x3::Matrix3x3;
use crate::tuple::Tuple;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut, Mul};

///
/// Row-major 4x4 matrix
///
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Matrix4x4([[f32; 4]; 4]);

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

impl Matrix4x4 {
    pub fn transpose(&self) -> Self {
        let mut result = Self::identity();
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
    pub fn identity() -> Self {
        let mut x = [[0.0; 4]; 4];
        for i in 0..4 {
            x[i][i] = 1.0;
        }
        Self(x)
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
        let mut result = Matrix4x4::identity();
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
        assert_eq!(i[0][0], 1.0);
        assert_eq!(i[1][1], 1.0);
        assert_eq!(i[2][2], 1.0);
        assert_eq!(i[3][3], 1.0);
        assert_eq!(i[0][1], 0.0);
        assert_eq!(i[0][2], 0.0);
        assert_eq!(i[0][3], 0.0);
        assert_eq!(i[1][0], 0.0);
        assert_eq!(i[1][2], 0.0);
        assert_eq!(i[1][3], 0.0);
        assert_eq!(i[2][0], 0.0);
        assert_eq!(i[2][1], 0.0);
        assert_eq!(i[2][3], 0.0);
        assert_eq!(i[3][0], 0.0);
        assert_eq!(i[3][1], 0.0);
        assert_eq!(i[3][2], 0.0);
    }

    #[test]
    fn identity_display() {
        assert_eq!(
            "[[1 0 0 0][0 1 0 0][0 0 1 0][0 0 0 1]]",
            format!("{}", Matrix4x4::identity())
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
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4x4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        assert_eq!(
            Matrix4x4::new([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0]
            ]),
            a * b
        );
    }

    #[test]
    fn multiply_matrix_by_identity() {
        let a = Matrix4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(a, a * Matrix4x4::identity());
    }

    #[test]
    fn multiply_matrix_by_tuple() {
        let a = Matrix4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(
            Tuple::new(18.0, 24.0, 33.0, 1.0),
            a * Tuple::new(1.0, 2.0, 3.0, 1.0)
        );
    }

    #[test]
    fn transpose_matrix() {
        let a = Matrix4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(
            Matrix4x4::new([
                [1.0, 5.0, 9.0, 5.0],
                [2.0, 6.0, 8.0, 4.0],
                [3.0, 7.0, 7.0, 3.0],
                [4.0, 8.0, 6.0, 2.0],
            ]),
            a.transpose()
        );
    }

    #[test]
    fn transpose_identity() {
        assert_eq!(Matrix4x4::identity().transpose(), Matrix4x4::identity());
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
