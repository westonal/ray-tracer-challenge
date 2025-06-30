use crate::camera::Camera;
use crate::canvas::Block;
use crate::world::World;

pub trait RenderWorld {
    fn render(&mut self, world: &World, camera: &Camera);
}

pub trait RenderPartialWorld {
    fn render_area(&mut self, world: &World, camera: &Camera, range: &Block);
}
