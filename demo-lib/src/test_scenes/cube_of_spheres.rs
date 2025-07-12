use crate::test_scenes::TestScene;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::color::{BLACK, GREEN, WHITE};
use math::tuple::point::Point;
use math::{color, degrees, point, vector};
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

pub struct CubeOfSpheres {}

impl TestScene for CubeOfSpheres {
    fn name() -> &'static str {
        "cube_of_spheres"
    }

    fn build_world() -> World {
        let mut world = World::default();
        world.max_ray_generation = 2;
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
        world.add_tree(Self::double(
            Matrix4x4::translation(0., 0.5, 0.),
            Some(Shape::new_cube_transformed(Matrix4x4::scale_all(8.))),
            |point| {
                Self::double(
                    Matrix4x4::translation(point.x * 4., point.y * 4., point.z * 4.),
                    Some(Shape::new_cube_transformed(
                        Matrix4x4::translation(1., 1., 1.).pre_scale_all(3.),
                    )),
                    |point| {
                        Self::cubes(Matrix4x4::translation(
                            point.x * 2.,
                            point.y * 2.,
                            point.z * 2.,
                        ))
                    },
                )
            },
        ));
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

impl CubeOfSpheres {
    fn cubes(matrix: Matrix4x4) -> SceneTree {
        Self::double(
            matrix,
            Some(Shape::new_cube_transformed(
                Matrix4x4::translation(0.5, 0.5, 0.5).pre_scale_all(2.),
            )),
            |point| {
                let mut sphere = Shape::new_sphere_transformed(
                    Matrix4x4::translation(point.x, point.y, point.z).pre_scale_all(0.5),
                );
                let mut material = Material::default();
                material.reflectivity = 0.9;
                material.specular = 1.;
                material.ambient = 1.;
                material.pattern = Pattern::Solid(color!(0, 0, 0));
                sphere.material = material;

                SceneTree::Leaf(sphere)
            },
        )
    }

    fn double<T: Fn(Point) -> SceneTree>(
        matrix: Matrix4x4,
        bound: Option<Shape>,
        f: T,
    ) -> SceneTree {
        let mut tree = SceneTree::new_bounded(matrix, bound);
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    tree.add_tree(f(point!(x, y, z)));
                }
            }
        }
        tree
    }
}
