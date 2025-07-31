use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Div, Mul};
use std::str::FromStr;
use std::sync::LazyLock;

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

static SIZE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?<width>\d+)([x×]?(?<height>\d+))?$").unwrap());

static SPECIAL_SIZE_MAP: LazyLock<HashMap<String, Size>> = LazyLock::new(|| {
    HashMap::from(
        [
            ("720p", Size::HD_720P),
            ("1080p", Size::HD_1080P),
            ("4k", Size::UHD_4K),
        ]
        .map(|(str, size)| (str.to_lowercase(), size)),
    )
});

impl FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(special) = SPECIAL_SIZE_MAP.get(&s.to_lowercase()) {
            return Ok(*special);
        }
        let option = SIZE_REGEX
            .captures(s)
            .ok_or_else(|| "Invalid Size String")?;
        let width = option
            .name("width")
            .map(|m| m.as_str().parse::<u32>())
            .unwrap()
            .map_err(|_| "Invalid width")?;
        let height = option
            .name("height")
            .map(|m| m.as_str().parse::<u32>())
            .unwrap_or(Ok(width))
            .map_err(|_| "Invalid width")?;
        Ok(Size::new(width, height))
    }
}

#[cfg(test)]
mod size_parsing_tests {
    use super::*;
    use Size;

    macro_rules! ok_parse_tests {
        ($($name:ident; $input:expr => $expect:expr)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($input.parse(), Ok($expect));
                }
            )*
        };
    }

    macro_rules! error_parse_tests {
        ($($name:ident; $input:expr => $expect:expr)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($input.parse::<Size>(), Err($expect.to_string()));
                }
            )*
        };
    }

    ok_parse_tests!(
        square_assumed; "100"     => Size::new(100, 100)
        width_x_height; "200x100" => Size::new(200, 100)

        // Special names
        seven_twenty;       "720p"  => Size::HD_720P
        seven_twenty_upper; "720P"  => Size::HD_720P
        ten_eighty;         "1080p" => Size::HD_1080P
        ten_eighty_upper;   "1080P" => Size::HD_1080P
        four_kay;           "4k"    => Size::UHD_4K
        four_kay_upper;     "4K"    => Size::UHD_4K

        reciprocal;         Size::UHD_4K.to_string() => Size::UHD_4K
    );

    error_parse_tests!(
        empty;         "" => "Invalid Size String"
        not_a_number; "x" => "Invalid Size String"
    );
}
