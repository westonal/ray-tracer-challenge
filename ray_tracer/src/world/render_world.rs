use crate::camera::Camera;
use crate::world::World;

pub trait RenderWorld {
    fn render(&mut self, world: &World, camera: &Camera);
}
