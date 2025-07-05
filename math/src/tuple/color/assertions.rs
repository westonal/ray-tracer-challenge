#[macro_export]
macro_rules! assert_color {
    ($left:expr, $right:expr) => {
        $crate::assert_tuple!($left, $right, 0.00005);
    };
}

#[cfg(test)]
mod assert_color_tests {
    use crate::color;

    macro_rules! color_assertion_fail_tests {
        ($($name:ident: $left:expr ; $right:expr)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name(){
                assert_color!($left, $right);
            }
        )*
        }
    }

    macro_rules! color_assertion_pass_tests {
        ($($name:ident: $left:expr ; $right:expr)*) => {
        $(
            #[test]
            fn $name(){
                assert_color!($left, $right);
            }
        )*
        }
    }

    color_assertion_fail_tests! {
        significant_red_left: color!(0.1001, 0.2, 0.8); color!(0.1, 0.2, 0.8)
        significant_red_right: color!(0.1, 0.2, 0.8); color!(0.1001, 0.2, 0.8)
        significant_green_left: color!(0.1, 0.2001, 0.8); color!(0.1, 0.2, 0.8)
        significant_green_right: color!(0.1, 0.2, 0.8); color!(0.1, 0.2001, 0.8)
        significant_blue_left: color!(0.1, 0.2, 0.8001); color!(0.1, 0.2, 0.8)
        significant_blue_right: color!(0.1, 0.2, 0.8); color!(0.1, 0.2, 0.8001)
        significant_alpha_left: color!(0.1, 0.2, 0.8, 0.4001); color!(0.1, 0.2, 0.8, 0.4)
        significant_alpha_right: color!(0.1, 0.2, 0.8, 0.4); color!(0.1, 0.2, 0.8, 0.4001)
    }

    color_assertion_pass_tests! {
        insignificant_red_left: color!(0.10001, 0.2, 0.8); color!(0.1, 0.2, 0.8)
        insignificant_red_right: color!(0.1, 0.2, 0.8); color!(0.10001, 0.2, 0.8)
        insignificant_green_left: color!(0.1, 0.20001, 0.8); color!(0.1, 0.2, 0.8)
        insignificant_green_right: color!(0.1, 0.2, 0.8); color!(0.1, 0.20001, 0.8)
        insignificant_blue_left: color!(0.1, 0.2, 0.80001); color!(0.1, 0.2, 0.8)
        insignificant_blue_right: color!(0.1, 0.2, 0.8); color!(0.1, 0.2, 0.80001)
        insignificant_alpha_left: color!(0.1, 0.2, 0.8, 0.40001); color!(0.1, 0.2, 0.8, 0.4)
        insignificant_alpha_right: color!(0.1, 0.2, 0.8, 0.4); color!(0.1, 0.2, 0.8, 0.40001)
    }
}
