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
