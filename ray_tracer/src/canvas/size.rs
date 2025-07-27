use std::fmt::{Display, Formatter};
use std::ops::{Div, Mul};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width() as f32 / self.height() as f32
    }

    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Size {
    pub const HD_720P: Size = Self {
        width: 1280,
        height: 720,
    };

    pub const HD_1080P: Size = Self {
        width: 1920,
        height: 1080,
    };

    pub const UHD_4K: Size = Self {
        width: 3840,
        height: 2160,
    };
}

impl From<Size> for (u32, u32) {
    fn from(value: Size) -> Self {
        (value.width, value.height)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}×{}", self.width, self.height)
    }
}

impl Mul<u32> for Size {
    type Output = Size;

    fn mul(self, rhs: u32) -> Self::Output {
        Self::new(self.width * rhs, self.height * rhs)
    }
}

impl Div<u32> for Size {
    type Output = Size;

    fn div(self, rhs: u32) -> Self::Output {
        Self::new(self.width / rhs, self.height / rhs)
    }
}

#[cfg(test)]
mod size_math_tests {
    use super::*;
    use Size;

    macro_rules! size_math_tests {
        ($($name:ident; $actual:expr => $expect:expr)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($actual, $expect);
                }
            )*
        };
    }

    size_math_tests!(
        div_whole_number_2; Size::new(200, 100) / 2 => Size::new(100, 50)
        div_whole_number_4; Size::new(200, 100) / 4 => Size::new(50, 25)
        div_round_down;     Size::new(65, 31) / 2   => Size::new(32, 15)
        mul_2;              Size::new(125, 50) * 2  => Size::new(250, 100)
        mul_3;              Size::new(50, 10) * 3   => Size::new(150, 30)
    );
}
