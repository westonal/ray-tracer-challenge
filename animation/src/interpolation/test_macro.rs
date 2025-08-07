#[cfg(test)]
#[macro_export]
macro_rules! interpolation_test_mod {
    (
        $interpolator:ident;
        $($name:ident; $t:expr => $expect_output_t:expr)*) => {
        paste::paste! {
            #[cfg(test)]
            mod [<$interpolator:lower _tests>] {
                use super::*;
                $(
                    #[test]
                    fn [<$interpolator:lower _ $name>]() {
                        assert_eq!($expect_output_t, $interpolator::interpolate($t));
                    }
                )*
            }
        }
    };
}
