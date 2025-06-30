use crate::canvas::size::Size;

pub trait ViewPort {
    fn size(&self) -> Size;
}
