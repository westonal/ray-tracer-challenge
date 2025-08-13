use math::tuple::color::{BLACK, BLUE, GREEN, RED, WHITE};
use math::{degrees, matrix4x4, point, translate, vector};
use ray_tracer::camera::Camera;
use ray_tracer::light;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::{BoundingVolumeDebug, World};
use ray_tracer::{auto, cube, cylinder, gradient_stops, material, plane, scene, sphere};

use crate::obj;
use dsl::still;

still!(
    BoundingVolumes;
    file_name: "bounding_volumes";
    camera: | size | {
        let mut camera = Camera::new(size, degrees!(50));
        camera.set_transform(
            ViewMatrix::new_look_at(point!(17, 19, 23), point!(1, 2, -3), vector!(0, 1, 0)),
        );
        camera
    };
    world: | world: &mut World | {
        world.render_preferences.bounding_volume_debug = BoundingVolumeDebug::Translucent;
    };
    scene: {
        scene!(
            +light!(point!(40, 40, 20), *WHITE * 0.9);
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
            +scene!(
                matrix: translate!(x:9; z:5;);
                +obj!(
                    path: "objs/chess/pawn.obj";
                    material: material!(color: (1., 0., 0.););
                );
            );
            +scene!(
                matrix: translate!(y: 1;);
                bounding_volume: auto!();
                +cube!(matrix: matrix4x4!(
                    rotation_z(degrees!(10))
                    translation(1., 1., 0.)
                ));
                +sphere!(matrix: matrix4x4!(
                    rotation_y(degrees!(270))
                    rotation_z(degrees!(50))
                    translation(1., 4., 0.)
                ));
            );
            +scene!(
                bounding_volume: auto!();
                +sphere!(
                    matrix: matrix4x4!(
                        scale_all(3.5)
                        translation(2., 1., -3.)
                        rotation_y(degrees!(55))
                    );
                    material: material!(
                        base: Material::glass();
                        color: (0.5, 0., 0.);
                        ambient: 0.3;
                    );
                );
            );
            +scene!(
                bounding_volume: auto!();
                +cylinder!(
                    matrix: matrix4x4!(
                        translation(-8., 4., -3.)
                        scale_all(4.)
                        rotation_y(degrees!(0))
                    );
                    material: material!(
                        reflectivity: 0.1;
                        pattern: Pattern::Gradient(
                            gradient_stops!(
                                0. => *RED,
                                0.5 => *BLUE,
                                1. => *GREEN
                            ),
                            Transform::new(matrix4x4!(
                                    rotation_y(degrees!(45))
                                    translation(-1., 0., 0.)
                                    scale_all(2.))),
                        );
                    );
                );
            );
        )
    };
);
