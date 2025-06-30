use crate::canvas::view_port::ViewPort;

pub trait PixelIterator<I>
where
    I: Iterator<Item = (u32, u32)>,
{
    fn pixels(&self) -> I;
}

impl<C: ViewPort> PixelIterator<RowByRowViewPortPixelIterator> for C {
    fn pixels(&self) -> RowByRowViewPortPixelIterator {
        RowByRowViewPortPixelIterator::new(self.width(), self.height())
    }
}

pub struct RowByRowViewPortPixelIterator {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}

impl RowByRowViewPortPixelIterator {
    fn new(width: u32, height: u32) -> Self {
        RowByRowViewPortPixelIterator {
            width,
            height,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for RowByRowViewPortPixelIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.height {
            return None;
        }
        let result = (self.x, self.y);
        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        Some(result)
    }
}
