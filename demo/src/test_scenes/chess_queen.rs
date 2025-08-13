use crate::obj;
use math::tuple::color::{BLACK, WHITE};
use math::{degrees, matrix4x4, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{material, plane, scene};
use std::default::Default;

dsl::still!(
    Queen;
    file_name: "chess_queen";
    camera: | size | {
        let mut camera = Camera::new(size, degrees!(35));
        camera.set_transform(
            ViewMatrix::new_look_at(point!(4, 6, 8), point!(0, 1.8, 0), vector!(0, 1, 0)),
        );
        camera
    };
    world: | world: &mut World | {
        world.push(PointLight::new(point!(2, 20, 10), *WHITE));
    };
    scene: {
        scene!(
            +plane!(pattern: Pattern::Checker(
                                *WHITE,
                                *BLACK,
                                Transform::new(matrix4x4!(scale_all(2.6) translation(0.5, 0., 0.5)))
                             );
            );
            +scene!(
                matrix: matrix4x4!(
                    translation(0.75, 0., 0.)
                    scale_all(0.22)
                );
                +obj!(
                    path: "objs/chess/queen.obj";
                    material: material!(
                            base: Material::glass();
                            color: (0.5, 0.5, 0.5);
                            ambient: 0.2;
                            transparency: 0.7;
                            reflectivity: 0.9;
                    );
                );
            );
        )
    };
);
