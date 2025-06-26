use crate::camera::Camera;
use crate::world::World;
use crate::world::render_world::RenderWorld;
use math::tuple::color::Color;

pub trait Canvas<C> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn ratio(&self) -> f32;

    fn write_color(&mut self, x_offset: u32, y_offset: u32, color: C);
}

impl<C: Canvas<Color>> RenderWorld for C {
    fn render(&mut self, world: &World, camera: &Camera) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let color = camera.color_at((x, y), &world);
                if color.alpha() > 0. {
                    self.write_color(x, y, color);
                }
            }
        }
    }
}
