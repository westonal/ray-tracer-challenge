use crate::intersection::{Intersect, Intersections};
use crate::lighting::PointLight;
use crate::rays::Ray;
use crate::scene_tree::FlatScene;
use math::tuple::color::Color;

pub mod render_world;

pub struct RenderableWorld<'w> {
    pub(crate) flat_scene: FlatScene,
    pub lights: &'w Vec<PointLight>,
    pub background: Color,
    pub max_ray_generation: u32,
}

impl Intersect for RenderableWorld<'_> {
    fn intersect(&self, ray: Ray) -> Intersections {
        self.flat_scene.intersect(ray)
    }
}
