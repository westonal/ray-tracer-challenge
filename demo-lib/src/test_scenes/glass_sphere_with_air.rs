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
use ray_tracer::scene_tree::SceneTree;
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
        world.add_light(PointLight::new(point!(-300, 200, 20), *WHITE));
        let mut plane = Shape::new_plane_transformed(Matrix4x4::translation(0., -32., 0.));
        plane.material.pattern =
            Pattern::Checker(*BLACK, *WHITE, Transform::new(Matrix4x4::scale_all(3.)));
        world.add(plane);

        let mut tree = SceneTree::new_bounded(Matrix4x4::scale_all(5.), Some(Shape::new_cube()));

        let mut sphere = Shape::new_sphere();
        sphere.material = Material::glass();
        tree.add(sphere);

        let mut bubble = Shape::new_sphere_transformed(Matrix4x4::scale_all(0.5));
        bubble.material = Material::air();
        tree.add(bubble);

        world.add_tree(tree);

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
