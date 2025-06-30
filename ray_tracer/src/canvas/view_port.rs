pub trait ViewPort {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

pub trait ViewPortRatio {
    fn ratio(&self) -> f32;
}

impl<V: ViewPort> ViewPortRatio for V {
    fn ratio(&self) -> f32 {
        self.width() as f32 / self.height() as f32
    }
}
