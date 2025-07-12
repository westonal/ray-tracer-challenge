mod default;
pub mod render_world;
pub mod shading;
mod shadows;

#[cfg(test)]
pub use crate::world::default::test_world;

use crate::intersection::{Intersect, Intersections};
use crate::lighting::PointLight;
use crate::primatives::Shape;
use crate::rays::Ray;
use crate::scene_tree::{FlatScene, SceneTree};
use math::tuple::color::Color;

pub struct World {
    pub scene_tree: SceneTree,
    pub lights: Vec<PointLight>,
    pub background: Color,
    pub max_ray_generation: u32,
}

impl<'w> World {
    pub fn prepare_for_render(&'w self) -> RenderableWorld<'w> {
        RenderableWorld {
            shapes: self.scene_tree.flatten(),
            lights: &self.lights,
            background: self.background,
            max_ray_generation: self.max_ray_generation,
        }
    }
}

pub struct RenderableWorld<'w> {
    pub(crate) shapes: FlatScene,
    pub lights: &'w Vec<PointLight>,
    pub background: Color,
    pub max_ray_generation: u32,
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

    pub fn add_tree(&mut self, object: SceneTree) {
        self.scene_tree.add_tree(object);
    }

    pub fn add(&mut self, object: Shape) {
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

impl Intersect for RenderableWorld<'_> {
    fn intersect(&self, ray: Ray) -> Intersections {
        self.shapes.intersect(ray)
    }
}

#[cfg(test)]
mod world_tests {
    use super::*;
    use crate::lighting::PointLight;
    use crate::primatives::Shape;
    use crate::ray;
    use math::matrix::matrix_4x4::Matrix4x4;

    use math::{color, point};

    #[test]
    fn default_world_values() {
        assert_eq!(10, World::default().max_ray_generation);
    }

    #[test]
    fn setup_world() {
        let mut world = World::default();
        world.add(Shape::new_sphere());
        world.add(Shape::new_sphere());
        assert_eq!(2, world.shape_count());
        assert!(world.lights.is_empty());
        world.set_light(PointLight::new(point!(-10, 10, -10), color!(1, 1, 1)));
        assert!(!world.lights.is_empty());
    }

    #[test]
    fn intersecting_world() {
        let mut world = World::default();
        world.add(Shape::new_sphere());
        world.add(Shape::new_sphere_transformed(Matrix4x4::scale(
            0.5, 0.5, 0.5,
        )));
        let world = world.prepare_for_render();
        let ray = ray!((0., 0., -5.), (0., 0., 1.));
        let intersections = world.intersect(ray);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].t, 4.);
        assert_eq!(intersections[1].t, 4.5);
        assert_eq!(intersections[2].t, 5.5);
        assert_eq!(intersections[3].t, 6.);
    }
}
