use crate::intersection::Intersect;
use crate::lighting::PointLight;
use crate::lighting::surface_hit::SurfaceHit;
use crate::ray;
use crate::render::RenderableWorld;

impl RenderableWorld<'_> {
    pub(crate) fn how_much_light_let_blocked(&self, point: &SurfaceHit, l: &PointLight) -> f32 {
        let intersections = self.intersect(&ray!(point.point, l.position - point.point));
        if let Some((hit, _)) = intersections.hit_excluding(point.shape_id)
            && hit.t < 1.
        {
            // TODO: One limitation of this is it only considers the first blocking object.
            //  What about the rest of the ray's path to the light?
            // TODO: Secondly, what about refraction/reflection around inside the transparent object.
            // TODO: Thirdly, the light let through an object should change color.
            hit.shape.material.shadow_opacity
        } else {
            // No hit, all light let though
            0.
        }
    }
}

#[cfg(test)]
mod shadow_tests {

    use crate::lighting::PointLight;
    use crate::lighting::surface_hit::SurfaceHit;
    use crate::primatives::ShapeId;
    use crate::sphere;
    use crate::world::World;
    use math::point;
    use math::tuple::color::Color;

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let mut world = World::default();
        let light = PointLight::new(point!(-10, 10, 0), Color::default());
        world.push(sphere!());
        let point = point!(0, 10, 0);
        let id = ShapeId::default();
        let surface_hit = SurfaceHit::new(&id, point);
        let world = world.prepare_for_render();
        let blocked = world.how_much_light_let_blocked(&surface_hit, &light);
        assert_eq!(blocked, 0.);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_light_and_the_object() {
        let mut world = World::default();
        let light = PointLight::new(point!(-10, 10, 0), Color::default());
        world.push(sphere!());
        let point = point!(5, -5, 0);
        let id = ShapeId::default();
        let surface_hit = SurfaceHit::new(&id, point);
        let world = world.prepare_for_render();
        let blocked = world.how_much_light_let_blocked(&surface_hit, &light);
        assert_eq!(blocked, 1.);
    }

    #[test]
    fn less_than_total_shadow_by_changing_material_shadow_opacity() {
        let mut world = World::default();
        let light = PointLight::new(point!(-10, 10, 0), Color::default());
        let mut shape = sphere!();
        shape.material.shadow_opacity = 0.4;
        world.push(shape);
        let point = point!(5, -5, 0);
        let id = ShapeId::default();
        let surface_hit = SurfaceHit::new(&id, point);
        let world = world.prepare_for_render();
        let blocked = world.how_much_light_let_blocked(&surface_hit, &light);
        assert_eq!(blocked, 0.4);
    }

    #[test]
    fn no_shadow_when_point_is_colinear_but_beyond_the_light() {
        let mut world = World::default();
        let light = PointLight::new(point!(-10, 10, 0), Color::default());
        world.push(sphere!());
        let point = point!(-11, 11, 0);
        let id = ShapeId::default();
        let surface_hit = SurfaceHit::new(&id, point);
        let world = world.prepare_for_render();
        let blocked = world.how_much_light_let_blocked(&surface_hit, &light);
        assert_eq!(blocked, 0.);
    }

    /// Note that all primatives are convex and so cannot self-shadow. This means we can exclude
    /// primitives from shadows by id and not rely on arbitrary over/under point offsets.
    #[test]
    fn a_primitive_shape_does_not_cast_a_shadow_on_self() {
        let mut world = World::default();
        let light = PointLight::new(point!(-10, 10, 0), Color::default());
        let shape = sphere!();
        let id = shape.id.clone();
        world.push(shape);
        let point = point!(5, -5, 0);
        let surface_hit = SurfaceHit::new(&id, point);
        let world = world.prepare_for_render();
        let blocked = world.how_much_light_let_blocked(&surface_hit, &light);
        assert_eq!(blocked, 0.);
    }
}
