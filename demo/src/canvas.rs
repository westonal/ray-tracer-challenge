use std::path::Path;

pub trait Canvas<C> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn ratio(&self) -> f32;

    fn save_png<Q>(&self, path: Q)
    where
        Q: AsRef<Path>;

    fn write_color(&mut self, x_offset: u32, y_offset: u32, color: C);
}
