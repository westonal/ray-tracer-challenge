mod block_iterator;
mod pixel_iterator;
mod size;
mod view_port;

pub use crate::canvas::block_iterator::*;
pub use crate::canvas::pixel_iterator::*;
pub use crate::canvas::view_port::ViewPort;
pub use size::Size;

pub trait Canvas<C>: ViewPort {
    fn write_color(&mut self, x_offset: u32, y_offset: u32, color: C);
}
