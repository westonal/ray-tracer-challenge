use dsl::still;
use math::tuple::color::{BLACK, BLUE, GREEN, RED, WHITE};
use math::{degrees, matrix4x4, point, translate, vector};
use ray_tracer::camera::Camera;
use ray_tracer::light;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::Shape;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{cube, gradient_stops, material, plane, scene};

still!(
    Cubes;
    file_name: "cubes";
    camera: | size | {
        let mut camera = Camera::new(size, degrees!(30));
        camera.set_transform(
            ViewMatrix::new_look_at(point!(17, 19, 23), point!(1, 2, -3), vector!(0, 1, 0)),
        );
        camera
    };
    world: | world: &mut World | {
        world.push(light!(point!(40, 40, 20), *WHITE * 0.9));
    };
    scene: {
        scene!(
            +floor();
            +cube!(matrix: translate!(y: 1;));
            +cube!(matrix: matrix4x4!(
                    scale_all(3.5)
                    translation(2., 1., -3.)
                    rotation_y(degrees!(55))
                );
                material: material!(
                    base: Material::glass();
                    color: (0.5, 0.0, 0.0);
                    ambient: 0.3;
                );
            );
            +cube!(matrix: matrix4x4!(
                translation(-8., 4., -3.)
                scale_all(4.)
                rotation_y(degrees!(0)));
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
                            scale_all(2.))
                        ),
                    );
                );
            );
        )
    };
);

fn floor() -> Shape {
    plane!(
        material: material!(
            reflectivity: 0.5;
            pattern: Pattern::Checker(
                *WHITE,
                *BLACK,
                Transform::new(matrix4x4!(rotation_y(degrees!(45)) scale_all(2.))),
            );
        );
    )
}
