use crate::matrix::matrix_4x4::Matrix4x4;
use crate::matrix::{Cofactor, Determinant};

impl Matrix4x4 {
    /// Try and invert the matrix.
    ///
    /// Returns [None] iff there is a zero determinant.
    pub fn invert(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }

        let mut res = Self::identity();
        for r in 0..4 {
            for c in 0..4 {
                res[r][c] = self.cofactor(r, c);
            }
        }
        Some(res.transpose() / det)
    }
}

#[cfg(test)]
mod invert_matrix4x4_tests {
    use crate::matrix::Determinant;
    use crate::matrix::matrix_4x4::Matrix4x4;

    #[test]
    fn non_invertible_matrix() {
        let a = Matrix4x4::new([
            [-4., 2., -2., -3.],
            [9., 6., 2., 6.],
            [0., -5., 1., -5.],
            [0., 0., 0., 0.],
        ]);

        assert_eq!(a.determinant(), 0.);

        assert!(a.invert().is_none());
    }

    #[test]
    fn invert_matrix() {
        let a = Matrix4x4::new([
            [-5., 2., 6., -8.],
            [1., -5., 1., 8.],
            [7., 7., -6., -7.],
            [1., -3., 7., 4.],
        ]);

        assert_eq!(
            a.invert().expect("Matrix is invertible"),
            Matrix4x4::new([
                [0.21804512, 0.45112783, 0.24060151, -0.04511278],
                [-0.8082707, -1.456767, -0.44360903, 0.5206767],
                [-0.078947365, -0.2236842, -0.05263158, 0.19736843],
                [-0.52255636, -0.81390977, -0.30075186, 0.30639097]
            ]),
        )
    }

    #[test]
    fn invert_invert_matrix_same_except_some_loss_of_precision() {
        let a = Matrix4x4::new([
            [-5., 2., 6., -8.],
            [1., -5., 1., 8.],
            [7., 7., -6., -7.],
            [1., -3., 7., 4.],
        ]);

        assert_eq!(
            a.invert()
                .expect("Matrix is invertible")
                .invert()
                .expect("Matrix is invertible"),
            Matrix4x4::new([
                [-4.9999986, 2.0000014, 6.000001, -8.000005],
                [1.0000004, -5.0000005, 0.9999966, 8.000004],
                [6.999998, 7.000001, -5.999993, -7.0000024],
                [1.0000023, -2.9999998, 7.000007, 3.9999993]
            ]),
        )
    }

    #[test]
    fn invert_matrix_2() {
        let a = Matrix4x4::new([
            [8., -5., 9., 2.],
            [7., 5., 6., 1.],
            [-6., 0., 9., 6.],
            [-3., 0., -9., -4.],
        ]);

        assert_eq!(
            a.invert().expect("Matrix is invertible"),
            Matrix4x4::new([
                [-0.15384616, -0.15384616, -0.2820513, -0.53846157],
                [-0.07692308, 0.12307692, 0.025641026, 0.03076923],
                [0.35897437, 0.35897437, 0.43589744, 0.9230769],
                [-0.6923077, -0.6923077, -0.7692308, -1.9230769]
            ]),
        )
    }

    #[test]
    fn invert_matrix_3() {
        let a = Matrix4x4::new([
            [9., 3., 0., 9.],
            [-5., -2., -6., -3.],
            [-4., 9., 6., 4.],
            [-7., 6., 6., 2.],
        ]);

        assert_eq!(
            a.invert().expect("Matrix is invertible"),
            Matrix4x4::new([
                [-0.04074074, -0.07777778, 0.14444445, -0.22222222],
                [-0.07777778, 0.033333335, 0.36666667, -0.33333334],
                [-0.029012345, -0.14629629, -0.10925926, 0.12962963],
                [0.17777778, 0.06666667, -0.26666668, 0.33333334]
            ]),
        )
    }
}
