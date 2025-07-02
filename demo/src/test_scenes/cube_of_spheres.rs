use crate::test_scenes::TestScene;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::color::{BLACK, GREEN, WHITE};
use math::{color, degrees, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::Shape;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;

pub struct CubeOfSpheres {}

impl TestScene for CubeOfSpheres {
    fn name() -> &'static str {
        "cube_of_spheres"
    }

    fn build_world() -> World {
        let mut world = World::default();
        world.max_ray_generation = 3;
        world.set_light(PointLight::new(point!(40, 40, 20), *WHITE * 0.9));
        let mut shape = Shape::new_plane();
        shape.material.pattern = Pattern::Checker(
            *WHITE,
            *BLACK,
            Transform::new(Matrix4x4::rotation_y(degrees!(45)).pre_scale_all(2.)),
        );
        shape.material.reflectivity = 0.5;
        world.add(shape);
        let mut shape = Shape::new_plane_transformed(Matrix4x4::translation(0., 45., 0.));
        shape.material.pattern = Pattern::Checker(
            *GREEN,
            *BLACK,
            Transform::new(Matrix4x4::rotation_y(degrees!(45)).pre_scale_all(10.)),
        );
        world.add(shape);
        for x in 0..8 {
            for y in 0..8 {
                for z in 0..8 {
                    let mut sphere = Shape::new_sphere_transformed(
                        Matrix4x4::translation(x as f32, y as f32 + 0.5, z as f32)
                            .pre_scale_all(0.5),
                    );
                    let mut material = Material::default();
                    material.reflectivity = 0.9;
                    material.specular = 1.;
                    material.ambient = 1.;
                    material.pattern = Pattern::Solid(color!(0, 0, 0));
                    sphere.material = material;

                    world.add(sphere)
                }
            }
        }
        world
    }

    fn build_camera(size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(30));
        camera.set_transform(
            ViewMatrix::new_look_at(point!(17, 19, 23), point!(4, 2, 4), vector!(0, 1, 0)).into(),
        );
        camera
    }
}
