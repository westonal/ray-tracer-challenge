#[macro_export]
macro_rules! light {
    ($point:expr) => {
        $crate::light!($point, *math::tuple::color::WHITE)
    };

    ($point:expr, $color:expr$(,)?) => {
        $crate::lighting::PointLight::new($point.into(), $color.into())
    };
}

#[cfg(test)]
mod point_light_macro_tests {
    use math::{assert_color, color, point};
    use crate::lighting::PointLight;

    macro_rules! point_light_test {
        ($($name:ident; $actual:expr => $expect:expr)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($actual.position, $expect.position);
                    assert_color!($actual.color, $expect.color);
                }
            )*
        };
    }

    point_light_test!(
        just_point_makes_white;
            light!(point!(1, 2, 3)) => PointLight::new(point!(1, 2, 3), color!(1, 1, 1, 1))

        specify_color;
            light!(point!(4, 5, 6), color!(1, 1, 0)) => PointLight::new(point!(4, 5, 6), color!(1, 1, 0, 1))

        specify_color_trailing_comma;
            light!(point!(4, 5, 6), color!(0, 0, 1),) => PointLight::new(point!(4, 5, 6), color!(0, 0, 1, 1))

        make_use_of_intos;
            light!((4., 5., 6.), (1., 1., 0.)) => PointLight::new(point!(4, 5, 6), color!(1, 1, 0, 1))
    );
}