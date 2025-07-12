use crate::test_scenes::TestScene;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::color::WHITE;
use math::{color, degrees, point};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::Shape;
use ray_tracer::scene_tree::SceneTree;
use ray_tracer::world::World;

pub struct Grid {}

impl TestScene for Grid {
    fn name() -> &'static str {
        "grid_of_spheres"
    }

    fn build_world() -> World {
        let mut root = SceneTree::default();
        let x_count: i32 = 11;
        let y_count: i32 = 7;
        let scale = 0.4;
        for x in -x_count..(x_count + 1) {
            let mut column = SceneTree::new_bounded(
                Matrix4x4::translation(x as f32, 0., 0.),
                Some(Shape::new_cube_transformed(Matrix4x4::scale(
                    scale,
                    y_count as f32 + 0.5,
                    scale,
                ))),
            );
            for y in -y_count..(y_count + 1) {
                let mut material = Material::default();
                material.pattern = Pattern::Solid(if x == 0 && y == 0 {
                    color!(1., 0.2, 0.2)
                } else if x.abs() == x_count - 1 || y.abs() == y_count - 1 {
                    color!(0., 1.0, 0.1)
                } else {
                    color!(0., 0.5, 0.9)
                });
                let mut sphere = Shape::new_sphere_transformed(
                    Matrix4x4::translation(0., y as f32, 0.).pre_scale_all(scale),
                );
                sphere.material = material;
                column.add(sphere);
            }
            root.add_tree(column);
        }

        let mut world = World::default();
        world.add_tree(SceneTree::new_single(
            Matrix4x4::translation(0., 0., -5.3),
            root,
        ));
        world.add_light(PointLight::new(point!(10, 10, 7), *WHITE));
        world
    }

    fn build_camera(size: Size) -> Camera {
        Camera::new(size, degrees!(120))
    }
}
