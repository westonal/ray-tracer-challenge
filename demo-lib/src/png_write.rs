use std::path::Path;

pub trait PngWrite {
    fn save_png<Q>(&self, path: Q)
    where
        Q: AsRef<Path>;
}
