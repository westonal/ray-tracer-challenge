use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
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

impl From<Size> for (u32, u32) {
    fn from(value: Size) -> Self {
        (value.width, value.height)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}
