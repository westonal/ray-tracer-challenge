use crate::canvas::size::Size;
use crate::canvas::view_port::ViewPort;
use std::cmp::min;

#[derive(Debug)]
pub struct Block {
    pub offset: (u32, u32),
    pub size: Size,
}

impl ViewPort for Block {
    fn size(&self) -> Size {
        self.size
    }
}

pub trait BlockIterator<I>
where
    I: Iterator<Item = Block>,
{
    fn blocks(&self, max_block_size: Size) -> I;
}

impl<C: ViewPort> BlockIterator<BlockByBlockViewPortIterator> for C {
    fn blocks(&self, max_block_size: Size) -> BlockByBlockViewPortIterator {
        BlockByBlockViewPortIterator::new(self.size(), max_block_size)
    }
}

pub struct BlockByBlockViewPortIterator {
    x: u32,
    y: u32,
    size: Size,
    max_block_size: Size,
}

impl BlockByBlockViewPortIterator {
    fn new(size: Size, max_block_size: Size) -> Self {
        Self {
            x: 0,
            y: 0,
            size,
            max_block_size,
        }
    }
}

impl Iterator for BlockByBlockViewPortIterator {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let (width, height) = self.size.into();
        if self.y >= height {
            return None;
        }
        let (mut block_width, mut block_height) = self.max_block_size.into();
        let (remain_width, remain_height) = (width - self.x, height - self.y);
        block_width = min(block_width, remain_width);
        block_height = min(block_height, remain_height);
        let result = Block {
            offset: (self.x, self.y),
            size: Size::new(block_width, block_height),
        };
        self.x += block_width;
        if self.x >= width {
            self.x = 0;
            self.y += block_height;
        }
        Some(result)
    }
}
