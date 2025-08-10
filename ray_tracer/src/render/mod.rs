use crate::intersection::{Intersect, Intersections};
use crate::lighting::PointLight;
use crate::rays::Ray;
use crate::scene_tree::FlatScene;
use math::tuple::color::Color;
use crate::world::RenderPreferences;

pub mod render_world;

pub struct RenderableWorld<'w> {
    pub(crate) flat_scene: FlatScene,
    pub lights: &'w Vec<PointLight>,
    pub render_preferences: &'w RenderPreferences,
}

impl Intersect for RenderableWorld<'_> {
    fn intersect(&self, ray: &Ray) -> Intersections {
        self.flat_scene.intersect(ray)
    }
}
