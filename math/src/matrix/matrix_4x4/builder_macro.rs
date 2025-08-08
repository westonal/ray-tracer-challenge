#[macro_export]
macro_rules! matrix4x4 {
    () => {
        $crate::matrix::matrix_4x4::Matrix4x4::identity()
    };
    ($($method:ident($($terms:expr$(,)?)+))+) => {
        {
            use $crate::matrix::matrix_4x4::*;
            let mut r = Matrix4x4::identity();
            $(r = r.$method($($terms, )+);)+
            r
        }
    };
    (
        [$m11:expr, $m12:expr, $m13:expr, $m14:expr]
        [$m21:expr, $m22:expr, $m23:expr, $m24:expr]
        [$m31:expr, $m32:expr, $m33:expr, $m34:expr]
        [$m41:expr, $m42:expr, $m43:expr, $m44:expr]
    ) => {
        $crate::matrix::matrix_4x4::Matrix4x4::new(
            [
                [$m11 as f32, $m12 as f32, $m13 as f32, $m14 as f32],
                [$m21 as f32, $m22 as f32, $m23 as f32, $m24 as f32],
                [$m31 as f32, $m32 as f32, $m33 as f32, $m34 as f32],
                [$m41 as f32, $m42 as f32, $m43 as f32, $m44 as f32],
            ]
        )
    }
}

#[macro_export]
macro_rules! translate {
    ($x:expr, $y:expr, $z:expr) => {
        matrix4x4!(
            translation($x as f32, $y as f32, $z as f32)
        )
    };

    (
        $(x: $x:expr;)?
        $(y: $y:expr;)?
        $(z: $z:expr;)?
    ) => {
        {
            let mut _x = 0_f32;
            let mut _y = 0_f32;
            let mut _z = 0_f32;
            $(_x = $x as f32;)?
            $(_y = $y as f32;)?
            $(_z = $z as f32;)?
            matrix4x4!(
                translation(_x, _y, _z)
            )
        }
    };
}

#[macro_export]
macro_rules! scale {
    ($x:expr, $y:expr, $z:expr) => {
        matrix4x4!(
            scale($x as f32, $y as f32, $z as f32)
        )
    };

    ($all:expr) => {
        matrix4x4!(
            scale_all($all as f32)
        )
    };

    (
        $(x: $x:expr;)?
        $(y: $y:expr;)?
        $(z: $z:expr;)?
    ) => {
        {
            let mut _x = 1_f32;
            let mut _y = 1_f32;
            let mut _z = 1_f32;
            $(_x = $x as f32;)?
            $(_y = $y as f32;)?
            $(_z = $z as f32;)?
            matrix4x4!(
                scale(_x, _y, _z)
            )
        }
    };
}

#[cfg(test)]
mod matrix4x4_builder_tests {
    use crate::degrees;
    use crate::matrix::matrix_4x4::*;

    #[test]
    fn identity() {
        assert_eq!(Matrix4x4::identity(), matrix4x4!());
    }

    #[test]
    fn translation() {
        assert_eq!(
            Matrix4x4::translation(1., 2., 3.5),
            matrix4x4!(translation(1., 2., 3.5))
        );
    }

    #[test]
    fn rotation_x() {
        assert_eq!(
            Matrix4x4::rotation_x(degrees!(45)),
            matrix4x4!(rotation_x(degrees!(45)))
        );
    }

    #[test]
    fn scale_all() {
        assert_eq!(Matrix4x4::scale_all(5.), matrix4x4!(scale_all(5.)));
    }

    #[test]
    fn two_operations() {
        assert_eq!(
            Matrix4x4::translation(1., 2., 3.).scale_all(2.),
            matrix4x4!(
                translation(1., 2., 3.)
                scale_all(2.)
            )
        );
    }
}

#[cfg(test)]
mod direct_build_tests {
    #[test]
    fn create_a_matrix_from_each_cell() {
        let matrix = matrix4x4!(
            [1,   2,  3,  4]
            [5,   6,  7,  8]
            [9,  10, 11, 12]
            [13, 14, 15, 16]
        );
        assert_eq!(
            "[[1 2 3 4][5 6 7 8][9 10 11 12][13 14 15 16]]",
            matrix.to_string()
        )
    }

    #[test]
    fn create_a_matrix_from_each_cell_floating_point() {
        let matrix = matrix4x4!(
            [1.1,   2.2,  3.3,  4.4]
            [5.1,   6.2,  7.3,  8.4]
            [9.1,  10.2, 11.3, 12.4]
            [13.1, 14.2, 15.3, 16.4]
        );
        assert_eq!(
            "[[1.1 2.2 3.3 4.4][5.1 6.2 7.3 8.4][9.1 10.2 11.3 12.4][13.1 14.2 15.3 16.4]]",
            matrix.to_string()
        )
    }
}
