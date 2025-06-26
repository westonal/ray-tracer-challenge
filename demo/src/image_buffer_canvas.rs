use crate::png_write::PngWrite;
use image::{ImageBuffer, ImageFormat, Rgba};
use math::tuple::color::Color;
use ray_tracer::canvas::Canvas;
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

    fn ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
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

impl PngWrite for ImageBufferCanvas {
    fn save_png<Q>(&self, path: Q)
    where
        Q: AsRef<Path>,
    {
        self.image_buffer
            .save_with_format(path, ImageFormat::Png)
            .unwrap();
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
