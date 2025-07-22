use crate::scene_tree::{Chain, FlattenTree};
mod default;
pub mod shading;
mod shadows;

#[cfg(test)]
pub use crate::world::default::test_world;

use crate::lighting::PointLight;
use crate::render::RenderableWorld;
use crate::scene_tree::SceneTree;
use math::tuple::color::Color;

pub struct World {
    pub scene_tree: SceneTree,
    pub lights: Vec<PointLight>,
    pub background: Color,
    pub max_ray_generation: u32,
}

impl<'w> World {
    pub fn prepare_for_render(&'w self) -> RenderableWorld<'w> {
        let scene = self.scene_tree.flatten();
        //debug_print(&scene);
        RenderableWorld {
            flat_scene: scene,
            lights: &self.lights,
            background: self.background,
            max_ray_generation: self.max_ray_generation,
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
    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    /// Set lighting to a single light, use add light for multiple lights
    pub fn set_light(&mut self, light: PointLight) {
        self.lights.clear();
        self.add_light(light);
    }
}

impl World {
    pub fn shape_count(&self) -> usize {
        self.scene_tree.shape_count()
    }

    pub fn add<T: Into<SceneTree>>(&mut self, object: T) {
        self.scene_tree.add(object);
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            scene_tree: Default::default(),
            lights: vec![],
            background: Default::default(),
            max_ray_generation: 10,
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
        assert_eq!(10, World::default().max_ray_generation);
    }

    #[test]
    fn setup_world() {
        let mut world = World::default();
        world.add(sphere!());
        world.add(sphere!());
        assert_eq!(2, world.shape_count());
        assert!(world.lights.is_empty());
        world.set_light(PointLight::new(point!(-10, 10, -10), color!(1, 1, 1)));
        assert!(!world.lights.is_empty());
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
