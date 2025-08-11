use dsl::still;
use math::matrix::matrix_4x4::{Matrix4x4, Matrix4x4ScaleAll};
use math::tuple::color::{BLACK, GREEN, WHITE};
use math::{degrees, matrix4x4, point, scale, translate, vector};
use ray_tracer::camera::Camera;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::scene_tree::SceneTree;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{cube, material, plane, scene, sphere};

still!(
    CubeOfSpheres;
    file_name: "cube_of_spheres";
    camera: | size | {
        let mut camera = Camera::new(size, degrees!(30));
        camera.set_transform(ViewMatrix::new_look_at(
            point!(17, 19, 23),
            point!(4, 2, 4),
            vector!(0, 1, 0),
        ));
        camera
    };
    world: | world: &mut World | {
        world.set_light(PointLight::new(point!(40, 40, 20), *WHITE * 0.9));
        world.render_preferences.max_ray_generation = 2;
    };
    scene: {
        scene!(
            +plane!(
                material: material!(
                    reflectivity: 0.5;
                    pattern: Pattern::Checker(
                        *WHITE,
                        *BLACK,
                        Transform::new(matrix4x4!(rotation_y(degrees!(45)) scale_all(2.))),
                    );
                );
            );
            +plane!(
                matrix: translate!(0, 45, 0);
                pattern: Pattern::Checker(
                    *GREEN,
                    *BLACK,
                    Transform::new(matrix4x4!(rotation_y(degrees!(45)) scale_all(10.))),
                );
            );
            +scene!(
                matrix: translate!(3.5, 4, 3.5);
                +scene!(
                    matrix: scale!(2);
                    bounding_volume: cube!(matrix: scale!(2));
                    +double(
                        |matrix| {
                            scene!(
                                matrix: matrix.scale_all(0.5);
                                bounding_volume: cube!(matrix: scale!(2));
                                +double(
                                    |matrix| {
                                        scene!(
                                            matrix: matrix;
                                            +cubes();
                                        )
                                    },
                                );
                            )
                        },
                    );
                );
            );
        )
    };
);

fn cubes() -> SceneTree {
    scene!(
        matrix: scale!(0.5);
        bounding_volume: cube!(matrix: scale!(2));
        +double(
            |matrix| {
                scene!(
                    matrix: matrix;
                    +sphere!(
                        material: material!(
                            color: *BLACK;
                            ambient: 1;
                            specular: 1;
                            reflectivity: 0.9;
                        );
                    );
                )
            },
        );
    )
}

fn double<T: Fn(Matrix4x4) -> SceneTree>(f: T) -> SceneTree {
    let mut tree = SceneTree::default();
    for x in 0..2 {
        for y in 0..2 {
            for z in 0..2 {
                tree.add(f(translate!(
                    x as f32 * 2. - 1.,
                    y as f32 * 2. - 1.,
                    z as f32 * 2. - 1.
                )));
            }
        }
    }
    tree
}
