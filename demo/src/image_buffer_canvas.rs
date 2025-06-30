use crate::png_write::PngWrite;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageFormat, Rgba};
use math::tuple::color::Color;
use ray_tracer::canvas::{Canvas, ViewPort};
use std::path::Path;

pub struct ImageBufferCanvas {
    width: u32,
    height: u32,
    image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl ImageBufferCanvas {
    pub(crate) fn draw(&mut self, other: &ImageBufferCanvas, offset: (u32, u32)) {
        self.image_buffer
            .copy_from(&other.image_buffer, offset.0, offset.1)
            .unwrap();
    }
}
impl ViewPort for ImageBufferCanvas {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

impl Canvas<Color> for ImageBufferCanvas {
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
    pub fn new(size: (u32, u32)) -> Self {
        let (width, height) = size;
        Self {
            width,
            height,
            image_buffer: ImageBuffer::new(width, height),
        }
    }
}
