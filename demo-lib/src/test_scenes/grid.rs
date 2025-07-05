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
use ray_tracer::world::World;

pub struct Grid {}

impl TestScene for Grid {
    fn name() -> &'static str {
        "grid_of_spheres"
    }

    fn build_world() -> World {
        let mut world = World::default();
        let x_count: i32 = 11;
        let y_count: i32 = 7;
        for y in -y_count..(y_count + 1) {
            for x in -x_count..(x_count + 1) {
                let mut material = Material::default();
                material.pattern = Pattern::Solid(if x == 0 && y == 0 {
                    color!(1., 0.2, 0.2)
                } else if x.abs() == x_count - 1 || y.abs() == y_count - 1 {
                    color!(0., 1.0, 0.1)
                } else {
                    color!(0., 0.5, 0.9)
                });
                let scale = 0.4;
                let mut sphere = Shape::new_sphere_transformed(
                    Matrix4x4::translation(x as f32, y as f32, -5.3).pre_scale(scale, scale, scale),
                );
                sphere.material = material;
                world.add(sphere);
            }
        }
        world.add_light(PointLight::new(point!(10, 10, 7), *WHITE));
        world
    }

    fn build_camera(size: Size) -> Camera {
        Camera::new(size, degrees!(120))
    }
}
