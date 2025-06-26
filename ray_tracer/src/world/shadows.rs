use crate::intersection::Intersect;
use crate::lighting::PointLight;
use crate::ray;
use crate::world::World;
use math::tuple::point::Point;

impl World {
    /// Which lights are not occluded by objects in the scene
    pub(crate) fn direct_lights(&self, point: Point) -> Vec<&PointLight> {
        if let Some(l) = &self.light {
            let intersections = self.intersect(ray!(point, l.position - point));
            if let Some(hit) = intersections.hit() {
                if hit.t < 1. { vec![] } else { vec![l] }
            } else {
                vec![l]
            }
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod shadow_tests {

    use crate::lighting::PointLight;
    use crate::primatives::sphere::Sphere;
    use crate::world::World;
    use math::point;
    use math::tuple::color::Color;

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let mut world = World::new();
        let light = PointLight::new(point!(-10, 10, 0), Color::default());
        world.set_light(light);
        world.add(Sphere::new());
        let point = point!(0, 10, 0);
        let direct_lights = world.direct_lights(point);
        assert_eq!(direct_lights.len(), 1);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_light_and_the_object() {
        let mut world = World::new();
        let light = PointLight::new(point!(-10, 10, 0), Color::default());
        world.set_light(light);
        world.add(Sphere::new());
        let point = point!(5, -5, 0);
        let direct_lights = world.direct_lights(point);
        assert_eq!(direct_lights.len(), 0);
    }

    #[test]
    fn no_shadow_when_point_is_colinear_but_beyond_the_light() {
        let mut world = World::new();
        let light = PointLight::new(point!(-10, 10, 0), Color::default());
        world.set_light(light);
        world.add(Sphere::new());
        let point = point!(-11, 11, 0);
        let direct_lights = world.direct_lights(point);
        assert_eq!(direct_lights.len(), 1);
    }
}
