use crate::test_scenes::TestScene;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::color::{BLACK, WHITE};
use math::tuple::point::Point;
use math::{degrees, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::Shape;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;

pub struct GlassSphereWithAir {}

impl TestScene for GlassSphereWithAir {
    fn name() -> &'static str {
        "glass_sphere_with_air"
    }

    fn build_world() -> World {
        let mut world = World::default();
        world.add_light(PointLight::new(point!(-30, 50, 10), *WHITE));
        let mut plane = Shape::new_plane();
        plane.material.pattern = Pattern::Checker(*BLACK, *WHITE, Transform::identity());
        world.add(plane);

        let mut sphere =
            Shape::new_sphere_transformed(Matrix4x4::scale_all(5.).pre_translation(0., 1., 0.));
        sphere.material = Material::glass();
        world.add(sphere);

        let mut bubble =
            Shape::new_sphere_transformed(Matrix4x4::scale_all(2.).pre_translation(0., 1., 0.));
        bubble.material = Material::air();
        world.add(bubble);

        world
    }

    fn build_camera(size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(20));
        camera.set_transform(*ViewMatrix::new_look_at(
            point!(0, 40, 0),
            Point::origin(),
            vector!(0, 0, 1),
        ));
        camera
    }
}
