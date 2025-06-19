use crate::canvas::Canvas;
use crate::tuple::color::Color;
use image::{ImageBuffer, ImageFormat, Rgba};
use std::path::Path;

pub struct ImageBufferCanvas {
    width: u32,
    height: u32,
    image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Canvas<Color> for ImageBufferCanvas {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn save_png<Q>(&self, path: Q)
    where
        Q: AsRef<Path>,
    {
        self.image_buffer
            .save_with_format(path, ImageFormat::Png)
            .unwrap();
    }

    fn write_color(&mut self, x_offset: u32, y_offset: u32, color: Color) {
        let img = &mut self.image_buffer;
        let c = Rgba::<u8>([
            (color.red() * 255.0).clamp(0.0, 255.0) as u8,
            (color.green() * 255.0).clamp(0.0, 255.0) as u8,
            (color.blue() * 255.0).clamp(0.0, 255.0) as u8,
            (color.alpha() * 255.0).clamp(0.0, 255.0) as u8,
        ]);
        img.put_pixel(x_offset, y_offset, c);
    }
}

impl ImageBufferCanvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            image_buffer: ImageBuffer::new(width, height),
        }
    }
}
