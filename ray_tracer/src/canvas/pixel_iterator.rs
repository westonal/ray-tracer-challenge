use crate::canvas::size::Size;
use crate::canvas::view_port::ViewPort;

pub trait PixelIterator<I>
where
    I: Iterator<Item = (u32, u32)>,
{
    fn pixels(&self) -> I;
}

impl<C: ViewPort> PixelIterator<RowByRowViewPortPixelIterator> for C {
    fn pixels(&self) -> RowByRowViewPortPixelIterator {
        RowByRowViewPortPixelIterator::new(self.size())
    }
}

pub struct RowByRowViewPortPixelIterator {
    size: Size,
    x: u32,
    y: u32,
}

impl RowByRowViewPortPixelIterator {
    fn new(size: Size) -> Self {
        RowByRowViewPortPixelIterator { size, x: 0, y: 0 }
    }
}

impl Iterator for RowByRowViewPortPixelIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let (width, height) = self.size.into();
        if self.y >= height {
            return None;
        }
        let result = (self.x, self.y);
        self.x += 1;
        if self.x >= width {
            self.x = 0;
            self.y += 1;
        }
        Some(result)
    }
}
