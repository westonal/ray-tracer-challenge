use crate::intersection::{Intersect, Intersections};
use crate::lighting::PointLight;
use crate::primatives::sphere::Sphere;
use crate::rays::Ray;

#[derive(Default)]
pub struct World {
    objects: Vec<Sphere>,
    pub light: Option<PointLight>,
}

impl World {
    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
    }
}

impl World {
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}

impl World {
    pub fn add(&mut self, object: Sphere) {
        self.objects.push(object);
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
        for object in &self.objects {
            results += object.intersect(ray);
        }
        results
    }
}

#[cfg(test)]
mod world_tests {
    use super::*;
    use crate::lighting::PointLight;
    use crate::primatives::sphere::Sphere;
    use crate::ray;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::tuple::color::Color;
    use math::tuple::point::Point;
    use math::{color, point};

    #[test]
    fn setup_world() {
        let mut world = World::new();
        world.add(Sphere::new());
        world.add(Sphere::new());
        assert_eq!(2, world.object_count());
        assert!(world.light.is_none());
        world.set_light(PointLight::new(point!(-10, 10, -10), color!(1., 1., 1.)));
        assert!(world.light.is_some());
    }

    #[test]
    fn intersecting_world() {
        let mut world = World::new();
        world.add(Sphere::new());
        world.add(Sphere::new_transformed(Matrix4x4::scale(0.5, 0.5, 0.5)));
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
