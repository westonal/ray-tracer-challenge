use crate::camera::Camera;
use crate::canvas::Block;
pub(crate) use crate::render::RenderableWorld;

pub trait RenderWorld {
    fn render(&mut self, world: &RenderableWorld, camera: &Camera);
}

pub trait RenderPartialWorld {
    fn render_area(&mut self, world: &RenderableWorld, camera: &Camera, range: &Block);
}
