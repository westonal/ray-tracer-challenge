#[macro_export]
macro_rules! assert_vector {
    ($left:expr, $right:expr) => {
        $crate::assert_tuple!($left, $right, 0.00005);
    };
}

#[cfg(test)]
mod assert_vector_tests {
    use crate::vector;

    macro_rules! vector_assertion_fail_tests {
    ($($name:ident: $left:expr ; $right:expr)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name(){
                assert_vector!($left, $right);
            }
        )*
        }
    }
    macro_rules! vector_assertion_pass_tests {
        ($($name:ident: $left:expr ; $right:expr)*) => {
        $(
            #[test]
            fn $name(){
                assert_vector!($left, $right);
            }
        )*
        }
    }

    vector_assertion_fail_tests! {
        significant_x_left: vector!(0.1001, 0.2, 0.8); vector!(0.1, 0.2, 0.8)
        significant_x_right: vector!(0.1, 0.2, 0.8); vector!(0.1001, 0.2, 0.8)
        significant_y_left: vector!(0.1, 0.2001, 0.8); vector!(0.1, 0.2, 0.8)
        significant_y_right: vector!(0.1, 0.2, 0.8); vector!(0.1, 0.2001, 0.8)
        significant_z_left: vector!(0.1, 0.2, 0.8001); vector!(0.1, 0.2, 0.8)
        significant_z_right: vector!(0.1, 0.2, 0.8); vector!(0.1, 0.2, 0.8001)
    }

    vector_assertion_pass_tests! {
        insignificant_x_left: vector!(0.10001, 0.2, 0.8); vector!(0.1, 0.2, 0.8)
        insignificant_x_right: vector!(0.1, 0.2, 0.8); vector!(0.10001, 0.2, 0.8)
        insignificant_y_left: vector!(0.1, 0.20001, 0.8); vector!(0.1, 0.2, 0.8)
        insignificant_y_right: vector!(0.1, 0.2, 0.8); vector!(0.1, 0.20001, 0.8)
        insignificant_z_left: vector!(0.1, 0.2, 0.80001); vector!(0.1, 0.2, 0.8)
        insignificant_z_right: vector!(0.1, 0.2, 0.8); vector!(0.1, 0.2, 0.80001)
    }
}
