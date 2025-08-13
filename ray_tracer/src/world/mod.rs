use crate::scene_tree::{Chain, FlattenScene};
mod default;
mod preferences;
pub mod shading;
mod shadows;

#[cfg(test)]
pub use crate::world::default::test_world;

use crate::render::RenderableWorld;
use crate::scene_tree::SceneTree;
pub use preferences::BoundingVolumeDebug;
pub use preferences::RenderPreferences;

pub struct World {
    pub scene_tree: SceneTree,
    pub render_preferences: RenderPreferences,
}

impl<'w> World {
    pub fn prepare_for_render(&'w self) -> RenderableWorld<'w> {
        let scene = self
            .scene_tree
            .flatten_scene_with_options(self.render_preferences.into());
        //debug_print(&scene);
        RenderableWorld {
            flat_scene: scene,
            render_preferences: &self.render_preferences,
        }
    }
}

fn debug_print(scene: &Vec<Chain>) {
    println!("== SCENE ==");
    for (i, s) in scene.iter().enumerate() {
        println!("{}: {:?}", i, s);
    }
    println!("== END ==");
}

impl World {
    pub fn shape_count(&self) -> usize {
        self.scene_tree.shape_count()
    }

    pub fn push<T: Into<SceneTree>>(&mut self, object: T) {
        self.scene_tree.add(object);
    }

    #[deprecated(note = "use `push`")] // clashes with Add trait for csg, use push
    pub fn add<T: Into<SceneTree>>(&mut self, object: T) {
        self.push(object)
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            scene_tree: Default::default(),
            render_preferences: Default::default(),
        }
    }
}

#[cfg(test)]
mod world_tests {
    use super::*;
    use crate::lighting::PointLight;

    use crate::{ray, sphere};
    use math::matrix::matrix_4x4::Matrix4x4;

    use crate::intersection::Intersect;
    use math::{color, point};

    #[test]
    fn default_world_values() {
        assert_eq!(10, World::default().render_preferences.max_ray_generation);
    }

    #[test]
    fn setup_world() {
        let mut world = World::default();
        world.add(sphere!());
        world.add(sphere!());
        assert_eq!(2, world.shape_count());
        assert!(world.prepare_for_render().flat_scene.lights.is_empty());
        world.push(PointLight::new(point!(-10, 10, -10), color!(1, 1, 1)));
        assert!(!world.prepare_for_render().flat_scene.lights.is_empty());
    }

    #[test]
    fn intersecting_world() {
        let mut world = World::default();
        world.add(sphere!());
        world.add(sphere!(matrix: Matrix4x4::scale(0.5, 0.5, 0.5)));
        let world = world.prepare_for_render();
        let ray = ray!((0., 0., -5.), (0., 0., 1.));
        let intersections = world.intersect(&ray);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].t, 4.);
        assert_eq!(intersections[1].t, 4.5);
        assert_eq!(intersections[2].t, 5.5);
        assert_eq!(intersections[3].t, 6.);
    }
}
