use crate::intersection::Intersect;
use crate::lighting::PointLight;
use crate::lighting::surface_hit::SurfaceHit;
use crate::ray;
use crate::render::RenderableWorld;
use math::tuple::point::Point;

impl RenderableWorld<'_> {
    /// Which lights are not occluded by objects in the scene
    pub(crate) fn direct_lights(&self, point: Point) -> Vec<&PointLight> {
        self.lights
            .iter()
            .filter_map(|l| {
                let intersections = self.intersect(ray!(point, l.position - point));
                if let Some((hit, _)) = intersections.hit() {
                    if hit.t < 1. { None } else { Some(l) }
                } else {
                    Some(l)
                }
            })
            .collect::<Vec<&PointLight>>()
    }

    /// Which lights are not occluded by objects in the scene, excluding the supplied surface
    /// TODO: This could be an issue for future non-convex surfaces, unable to cast a shadow on themselves
    pub(crate) fn direct_lights_excluding_surface(&self, point: &SurfaceHit) -> Vec<&PointLight> {
        self.lights
            .iter()
            .filter_map(|l| {
                let intersections = self.intersect(ray!(point.point, l.position - point.point));
                if let Some((hit, _)) = intersections.hit_excluding(point.shape_id) {
                    if hit.t < 1. { None } else { Some(l) }
                } else {
                    Some(l)
                }
            })
            .collect::<Vec<&PointLight>>()
    }
}

#[cfg(test)]
mod shadow_tests {

    use crate::lighting::PointLight;
    use crate::primatives::Shape;
    use crate::world::World;
    use math::point;
    use math::tuple::color::Color;

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let mut world = World::default();
        let light = PointLight::new(point!(-10, 10, 0), Color::default());
        world.set_light(light);
        world.add(Shape::new_sphere());
        let point = point!(0, 10, 0);
        let world = world.prepare_for_render();
        let direct_lights = world.direct_lights(point);
        assert_eq!(direct_lights.len(), 1);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_light_and_the_object() {
        let mut world = World::default();
        let light = PointLight::new(point!(-10, 10, 0), Color::default());
        world.set_light(light);
        world.add(Shape::new_sphere());
        let point = point!(5, -5, 0);
        let world = world.prepare_for_render();
        let direct_lights = world.direct_lights(point);
        assert_eq!(direct_lights.len(), 0);
    }

    #[test]
    fn no_shadow_when_point_is_colinear_but_beyond_the_light() {
        let mut world = World::default();
        let light = PointLight::new(point!(-10, 10, 0), Color::default());
        world.set_light(light);
        world.add(Shape::new_sphere());
        let point = point!(-11, 11, 0);
        let world = world.prepare_for_render();
        let direct_lights = world.direct_lights(point);
        assert_eq!(direct_lights.len(), 1);
    }
}
