use std::path::Path;

pub trait Canvas<C> {
    fn save_png<Q>(&self, path: Q)
    where
        Q: AsRef<Path>;

    fn write_color(&mut self, x_offset: u32, y_offset: u32, color: C);
}
