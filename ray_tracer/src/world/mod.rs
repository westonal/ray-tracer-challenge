pub mod render_world;
pub mod shading;
mod shadows;

use crate::intersection::{Intersect, Intersections};
use crate::lighting::PointLight;
use crate::primatives::Shape;
use crate::rays::Ray;
use math::tuple::color::Color;

#[derive(Default)]
pub struct World {
    shapes: Vec<Shape>,
    pub light: Option<PointLight>,
    pub background: Color,
}

impl World {
    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }
}

impl World {
    pub fn object_count(&self) -> usize {
        self.shapes.len()
    }
}

impl World {
    pub fn add(&mut self, object: Shape) {
        self.shapes.push(object);
    }
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Intersect for World {
    fn intersect(&self, ray: Ray) -> Intersections {
        let mut results = Intersections::default();
        for object in &self.shapes {
            results += object.intersect(ray);
        }
        results
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
    fn setup_world() {
        let mut world = World::new();
        world.add(Shape::new_sphere());
        world.add(Shape::new_sphere());
        assert_eq!(2, world.object_count());
        assert!(world.light.is_none());
        world.set_light(PointLight::new(point!(-10, 10, -10), color!(1, 1, 1)));
        assert!(world.light.is_some());
    }

    #[test]
    fn intersecting_world() {
        let mut world = World::new();
        world.add(Shape::new_sphere());
        world.add(Shape::new_sphere_transformed(Matrix4x4::scale(
            0.5, 0.5, 0.5,
        )));
        let world = world;
        let ray = ray!((0., 0., -5.), (0., 0., 1.));
        let intersections = world.intersect(ray);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].t, 4.);
        assert_eq!(intersections[1].t, 4.5);
        assert_eq!(intersections[2].t, 5.5);
        assert_eq!(intersections[3].t, 6.);
    }
}
