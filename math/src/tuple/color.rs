use crate::tuple::Tuple;
use std::fmt::Formatter;
use std::ops::{Add, Mul, Sub};
use std::sync::LazyLock;

#[derive(PartialEq, Copy, Clone)]
pub struct Color {
    tuple: Tuple,
}

impl Color {
    pub fn cross_blend(&self, other: Color, ratio: f32) -> Color {
        let a = *self;
        a + (other - a) * ratio.clamp(0., 1.)
    }
}

impl Color {
    pub fn red(&self) -> f32 {
        self.tuple.x
    }

    pub fn green(&self) -> f32 {
        self.tuple.y
    }

    pub fn blue(&self) -> f32 {
        self.tuple.z
    }

    pub fn alpha(&self) -> f32 {
        self.tuple.w
    }
}

impl Color {
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            tuple: Tuple::new(r, g, b, a),
        }
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(tuple: (f32, f32, f32)) -> Color {
        Color::rgba(tuple.0, tuple.1, tuple.2, 1.)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(tuple: (f32, f32, f32, f32)) -> Color {
        Color::rgba(tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("C{}", self.tuple))
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

#[cfg(test)]
mod color_display_tests {
    use super::*;
    #[test]
    fn tuple_display_vector() {
        let color = Color::rgba(1., 0.5, 0., 0.7);

        assert_eq!("C(1, 0.5, 0, 0.7)", format!("{color}"));
    }
}

#[cfg(test)]
mod color_rgb_tests {
    use super::*;
    #[test]
    fn color_channel_access() {
        let color = Color::rgba(1., 0.5, 0.2, 0.7);

        assert_eq!(1., color.red());
        assert_eq!(0.5, color.green());
        assert_eq!(0.2, color.blue());
        assert_eq!(0.7, color.alpha());
    }
}

#[cfg(test)]
mod color_into_tests {
    use super::*;
    #[test]
    fn tuple_3_into_color() {
        assert_eq!(Color::rgba(1., 0.5, 0.6, 1.), (1., 0.5, 0.6).into());
    }

    #[test]
    fn tuple_4_into_color() {
        assert_eq!(Color::rgba(1., 0.5, 0.6, 0.3), (1., 0.5, 0.6, 0.3).into());
    }
}

#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        $crate::tuple::color::Color::rgba($r as f32, $g as f32, $b as f32, $a as f32)
    };
    ($r:expr, $g:expr, $b:expr) => {
        color!($r, $g, $b, 1)
    };
}

#[macro_export]
macro_rules! pub_static_color {
    ($name:ident= $color:expr) => {
        pub static $name: LazyLock<Color> = LazyLock::new(|| $color);
    };
}

pub_static_color! {WHITE = color!(1,1,1)}
pub_static_color! {BLACK = color!(0,0,0)}
pub_static_color! {RED = color!(1,0,0)}
pub_static_color! {GREEN = color!(0,1,0)}
pub_static_color! {BLUE = color!(0,0,1)}

#[cfg(test)]
mod color_macro_creation_tests {
    use super::*;
    #[test]
    fn macro_test() {
        assert_eq!(Color::rgba(1., 0.5, 0.7, 1.0), color!(1., 0.5, 0.7));
    }

    #[test]
    fn macro_test_with_alpha() {
        assert_eq!(Color::rgba(1., 0.5, 0.7, 0.2), color!(1., 0.5, 0.7, 0.2));
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::rgba(0., 0., 0., 0.)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            tuple: self.tuple + rhs.tuple,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            tuple: self.tuple - rhs.tuple,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            tuple: self.tuple * rhs,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Self {
            tuple: self.tuple.hadamard_product(rhs.tuple),
        }
    }
}

#[cfg(test)]
mod color_math_tests {
    use super::*;

    #[test]
    fn color_add() {
        let a = Color::rgba(0.9, 0.6, 0.75, 1.);
        let b = Color::rgba(1.7, 0.2, 0.25, 1.);

        assert_eq!(Color::rgba(2.6, 0.8, 1., 2.), a + b)
    }

    #[test]
    fn color_sub() {
        let a = Color::rgba(4., 0.7, 0.8, 0.4);
        let b = Color::rgba(2., 0.2, 0.25, 0.2);

        assert_eq!(Color::rgba(2., 0.5, 0.55, 0.2), a - b)
    }

    #[test]
    fn color_multiply_scalar() {
        let a = Color::rgba(4., 0.7, 0.8, 0.4);

        assert_eq!(Color::rgba(8., 1.4, 1.6, 0.8), a * 2.)
    }

    #[test]
    fn color_pairwise_multiply_by_color() {
        let a = Color::rgba(4., 0.7, 0.8, 0.5);
        let b = Color::rgba(2., 0.2, 0.25, 0.2);

        assert_eq!(Color::rgba(8., 0.14, 0.2, 0.1), a * b)
    }
}

#[cfg(test)]
mod cross_blend_tests {

    #[test]
    fn cross_blend() {
        let c1 = color!(1, 0.5, 0.25);
        let c2 = color!(0, 1., 0.75);
        assert_eq!(c1, c1.cross_blend(c2, -0.1));
        assert_eq!(c1, c1.cross_blend(c2, 0.));
        assert_eq!(c2, c1.cross_blend(c2, 1.));
        assert_eq!(c2, c1.cross_blend(c2, 1.1));
        assert_eq!(color!(0.5, 0.75, 0.5), c1.cross_blend(c2, 0.5));
        assert_eq!(color!(0.75, 0.625, 0.375), c1.cross_blend(c2, 0.25));
    }

    #[test]
    fn cross_blend_with_alpha() {
        let c1 = color!(1, 0.5, 0.25, 0.1);
        let c2 = color!(0, 1., 0.75, 0.6);
        assert_eq!(c1, c1.cross_blend(c2, 0.));
        assert_eq!(c2, c1.cross_blend(c2, 1.));
        assert_eq!(color!(0.5, 0.75, 0.5, 0.35), c1.cross_blend(c2, 0.5));
        assert_eq!(color!(0.25, 0.875, 0.625, 0.475), c1.cross_blend(c2, 0.75));
    }
}
