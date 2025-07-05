#[macro_export]
macro_rules! assert_tuple {
    ($left:expr, $right:expr, $epsilon:expr) => {
        let difference: $crate::tuple::Tuple = ($left - $right).into();
        let max_difference = difference.abs().max();
        if max_difference >= $epsilon {
            assert!(
                false,
                "{} != {}, one element differs by {}",
                $left, $right, max_difference
            );
        }
    };
    ($left:expr, $right:expr) => {
        $crate::assert_tuple!($left, $right, 0.00005)
    };
}

#[cfg(test)]
mod assert_tuple_tests {

    macro_rules! tuple_assertion_fail_tests {
    ($($name:ident: $left:expr ; $right:expr)*) => {
    $(
        #[test]
        #[should_panic]
        fn $name(){
            assert_tuple!($left, $right);
        }
    )*
    }
    }
    macro_rules! tuple_assertion_pass_tests {
    ($($name:ident: $left:expr ; $right:expr)*) => {
    $(
        #[test]
        fn $name(){
            assert_tuple!($left, $right);
        }
    )*
    }
    }

    macro_rules! tuple {
        ($x:expr,$y:expr,$z:expr,$w:expr) => {
            $crate::tuple::Tuple::new($x, $y, $z, $w)
        };
    }

    tuple_assertion_fail_tests! {
        significant_red_left: tuple!(0.1001, 0.2, 0.8,0.4); tuple!(0.1, 0.2, 0.8,0.4)
        significant_red_right: tuple!(0.1, 0.2, 0.8,0.4); tuple!(0.1001, 0.2, 0.8,0.4)
        significant_green_left: tuple!(0.1, 0.2001, 0.8,0.4); tuple!(0.1, 0.2, 0.8,0.4)
        significant_green_right: tuple!(0.1, 0.2, 0.8,0.4); tuple!(0.1, 0.2001, 0.8,0.4)
        significant_blue_left: tuple!(0.1, 0.2, 0.8001,0.4); tuple!(0.1, 0.2, 0.8,0.4)
        significant_blue_right: tuple!(0.1, 0.2, 0.8,0.4); tuple!(0.1, 0.2, 0.8001,0.4)
        significant_alpha_left: tuple!(0.1, 0.2, 0.8, 0.4001); tuple!(0.1, 0.2, 0.8, 0.4)
        significant_alpha_right: tuple!(0.1, 0.2, 0.8, 0.4); tuple!(0.1, 0.2, 0.8, 0.4001)
    }

    tuple_assertion_pass_tests! {
        insignificant_red_left: tuple!(0.10001, 0.2, 0.8,0.4); tuple!(0.1, 0.2, 0.8,0.4)
        insignificant_red_right: tuple!(0.1, 0.2, 0.8,0.4); tuple!(0.10001, 0.2, 0.8,0.4)
        insignificant_green_left: tuple!(0.1, 0.20001, 0.8,0.4); tuple!(0.1, 0.2, 0.8,0.4)
        insignificant_green_right: tuple!(0.1, 0.2, 0.8,0.4); tuple!(0.1, 0.20001, 0.8,0.4)
        insignificant_blue_left: tuple!(0.1, 0.2, 0.80001,0.4); tuple!(0.1, 0.2, 0.8,0.4)
        insignificant_blue_right: tuple!(0.1, 0.2, 0.8,0.4); tuple!(0.1, 0.2, 0.80001,0.4)
        insignificant_alpha_left: tuple!(0.1, 0.2, 0.8, 0.40001); tuple!(0.1, 0.2, 0.8, 0.4)
        insignificant_alpha_right: tuple!(0.1, 0.2, 0.8, 0.4); tuple!(0.1, 0.2, 0.8, 0.40001)
    }
}
