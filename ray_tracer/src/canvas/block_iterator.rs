use crate::canvas::view_port::ViewPort;
use std::cmp::min;

#[derive(Debug)]
pub struct Block {
    pub offset: (u32, u32),
    pub size: (u32, u32),
}

impl ViewPort for Block {
    fn width(&self) -> u32 {
        self.size.0
    }

    fn height(&self) -> u32 {
        self.size.1
    }
}

pub trait BlockIterator<I>
where
    I: Iterator<Item = Block>,
{
    fn blocks(&self, max_block_size: (u32, u32)) -> I;
}

impl<C: ViewPort> BlockIterator<BlockByBlockViewPortIterator> for C {
    fn blocks(&self, max_block_size: (u32, u32)) -> BlockByBlockViewPortIterator {
        BlockByBlockViewPortIterator::new((self.width(), self.height()), max_block_size)
    }
}

pub struct BlockByBlockViewPortIterator {
    x: u32,
    y: u32,
    size: (u32, u32),
    max_block_size: (u32, u32),
}

impl BlockByBlockViewPortIterator {
    fn new(size: (u32, u32), max_block_size: (u32, u32)) -> Self {
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
        let (width, height) = self.size;
        if self.y >= height {
            return None;
        }
        let (mut block_width, mut block_height) = self.max_block_size;
        let (remain_width, remain_height) = (width - self.x, height - self.y);
        block_width = min(block_width, remain_width);
        block_height = min(block_height, remain_height);
        let result = Block {
            offset: (self.x, self.y),
            size: (block_width, block_height),
        };
        self.x += block_width;
        if self.x >= width {
            self.x = 0;
            self.y += block_height;
        }
        Some(result)
    }
}
