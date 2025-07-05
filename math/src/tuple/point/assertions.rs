#[macro_export]
macro_rules! assert_point {
    ($left:expr, $right:expr) => {
        $crate::assert_tuple!($left, $right, 0.00005);
    };
}

#[cfg(test)]
mod assert_point_tests {
    use crate::point;

    macro_rules! point_assertion_fail_tests {
    ($($name:ident: $left:expr ; $right:expr)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name(){
                assert_point!($left, $right);
            }
        )*
        }
    }
    macro_rules! point_assertion_pass_tests {
        ($($name:ident: $left:expr ; $right:expr)*) => {
        $(
            #[test]
            fn $name(){
                assert_point!($left, $right);
            }
        )*
        }
    }

    point_assertion_fail_tests! {
        significant_x_left: point!(0.1001, 0.2, 0.8); point!(0.1, 0.2, 0.8)
        significant_x_right: point!(0.1, 0.2, 0.8); point!(0.1001, 0.2, 0.8)
        significant_y_left: point!(0.1, 0.2001, 0.8); point!(0.1, 0.2, 0.8)
        significant_y_right: point!(0.1, 0.2, 0.8); point!(0.1, 0.2001, 0.8)
        significant_z_left: point!(0.1, 0.2, 0.8001); point!(0.1, 0.2, 0.8)
        significant_z_right: point!(0.1, 0.2, 0.8); point!(0.1, 0.2, 0.8001)
    }

    point_assertion_pass_tests! {
        insignificant_x_left: point!(0.10001, 0.2, 0.8); point!(0.1, 0.2, 0.8)
        insignificant_x_right: point!(0.1, 0.2, 0.8); point!(0.10001, 0.2, 0.8)
        insignificant_y_left: point!(0.1, 0.20001, 0.8); point!(0.1, 0.2, 0.8)
        insignificant_y_right: point!(0.1, 0.2, 0.8); point!(0.1, 0.20001, 0.8)
        insignificant_z_left: point!(0.1, 0.2, 0.80001); point!(0.1, 0.2, 0.8)
        insignificant_z_right: point!(0.1, 0.2, 0.8); point!(0.1, 0.2, 0.80001)
    }
}
