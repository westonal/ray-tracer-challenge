use crate::obj;
use dsl::still;
use math::tuple::color::{RED, WHITE};
use math::{color, degrees, matrix4x4, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{plane, scene};
use std::default::Default;

still!(
    Teapot;
    file_name: "utah_teapot";
    camera: | size| {
        let mut camera = Camera::new(size, degrees!(35));
        camera.set_transform(
            ViewMatrix::new_look_at(point!(8, 6, 4), point!(0, 0.8, 0), vector!(0, 1, 0)),
        );
        camera
    };
    world: | world: &mut World | {
        world.add_light(PointLight::new(
            point!(2, 20, 10),
            color!(1, 0.5, 0.5) * 0.5,
        ));
        world.add_light(PointLight::new(
            point!(-2, 20, -10),
            color!(0.5, 1, 0.5) * 0.5,
        ));
        world.add_light(PointLight::new(
            point!(-10, 20, -2),
            color!(0.5, 0.5, 1) * 0.5,
        ));
    };
    scene: scene!(
            +plane!(pattern: Pattern::Checker(*WHITE, *RED, Transform::identity()););
            +scene!(
                matrix: matrix4x4!(rotation_y(degrees!(-60)));
                +obj!(path: "objs/teapot.obj";);
            );
        );
);
