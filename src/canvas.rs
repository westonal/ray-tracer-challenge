use image::{ImageBuffer, ImageFormat, Rgba};

pub struct Canvas {
    width: u32,
    height: u32,
    image_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl Canvas {
    pub(crate) fn save(&self) {
        self.image_buffer
            .save_with_format("out.png", ImageFormat::Png)
            .unwrap()
    }
}

impl Canvas {
    pub(crate) fn write_color(&mut self, x_offset: u32, y_offset: u32) {
        let img = &mut self.image_buffer;
        for x in 15..=17 {
            for y in 8..24 {
                img.put_pixel(x_offset + x, y_offset + y, Rgba([255u8, 0u8, 0u8, 255u8]));
                img.put_pixel(x_offset + y, y_offset + x, Rgba([255u8, 0u8, 0u8, 255u8]));
            }
        }
    }
}

impl Canvas {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            image_buffer: ImageBuffer::new(width, height),
        }
    }
}
