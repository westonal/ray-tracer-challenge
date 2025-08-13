use math::tuple::color::{BLACK, BLUE, GREEN, RED, WHITE};
use math::{degrees, matrix4x4, point};
use ray_tracer::camera::Camera;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::{Shape, Triangle};
use ray_tracer::transform::Transform;
use ray_tracer::{gradient_stops, material, plane, scene};
use ray_tracer::{light, look_at};

dsl::still!(
    Triangles;
    file_name: "triangles";
    camera: | size | {
        let mut camera = Camera::new(size, degrees!(30));
        camera.set_transform(look_at!(point!(17, 19, 23) => point!(1, 2, -3)));
        camera
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
            +Shape::new_triangle_transformed(
                matrix4x4!(translation(0., 1., 0.)),
                Triangle::new([point!(0, 0, 0), point!(0, 1, 0), point!(1, 1, 0)]),
            );
            +{
                let mut triangle = Shape::new_triangle_transformed(
                    matrix4x4!(
                        scale_all(3.5)
                        translation(2., 1., -3.)
                        rotation_y(degrees!(55))
                    ),
                    Triangle::new([point!(0, 0, 0), point!(0, 1, 0), point!(1, 1, 0)]),
                );
                triangle.material = material!(
                    base:Material::glass();
                    color: (0.5, 0.0, 0.0);
                    ambient: 0.3;
                );
                triangle
            };
            +{
                let mut triangle = Shape::new_triangle_transformed(
                    matrix4x4!(
                        translation(-8., 4., -3.)
                        scale_all(4.)
                    ),
                    Triangle::new([point!(0, 0, 0), point!(0, 1, 0), point!(1, 1, 0)]),
                );
                triangle.material = material!(
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
                            scale_all(2.)
                        )),
                    );
                );
                triangle
            };
        )
    };
);
