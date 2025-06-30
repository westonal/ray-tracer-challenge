use crate::image_buffer_canvas::ImageBufferCanvas;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::{Block, BlockIterator, Canvas, PixelIterator, ViewPort};
use ray_tracer::world::World;
use ray_tracer::world::render_world::{RenderPartialWorld, RenderWorld};
use rayon::prelude::*;
use std::ops::{Deref, DerefMut};

pub struct ThreadedCanvas {
    block_size: (u32, u32),
    image_buffer_canvas: ImageBufferCanvas,
}

impl ThreadedCanvas {
    pub fn new(size: (u32, u32), block_size: u32) -> Self {
        Self {
            block_size: (block_size, block_size),
            image_buffer_canvas: ImageBufferCanvas::new(size),
        }
    }
}

impl Deref for ThreadedCanvas {
    type Target = ImageBufferCanvas;

    fn deref(&self) -> &Self::Target {
        &self.image_buffer_canvas
    }
}

impl DerefMut for ThreadedCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.image_buffer_canvas
    }
}

//TODO needed?
impl ViewPort for ThreadedCanvas {
    fn width(&self) -> u32 {
        self.image_buffer_canvas.width()
    }

    fn height(&self) -> u32 {
        self.image_buffer_canvas.height()
    }
}

impl RenderWorld for ImageBufferCanvas {
    fn render(&mut self, world: &World, camera: &Camera) {
        for (x, y) in self.pixels() {
            let color = camera.color_at((x, y), &world);
            if color.alpha() > 0. {
                self.write_color(x, y, color);
            }
        }
    }
}

impl RenderPartialWorld for ImageBufferCanvas {
    fn render_area(&mut self, world: &World, camera: &Camera, block: &Block) {
        for (x, y) in block.pixels() {
            let color = camera.color_at((block.offset.0 + x, block.offset.1 + y), &world);
            if color.alpha() > 0. {
                self.write_color(x, y, color);
            }
        }
    }
}

impl RenderWorld for ThreadedCanvas {
    fn render(&mut self, world: &World, camera: &Camera) {
        let vec: Vec<_> = self
            .blocks(self.block_size)
            .par_bridge()
            .map(|b| {
                let mut temp = ImageBufferCanvas::new(b.size);
                temp.render_area(world, camera, &b);
                (temp, b)
            })
            .collect();

        for (temp, block) in vec {
            self.image_buffer_canvas.draw(&temp, block.offset)
        }
    }
}
