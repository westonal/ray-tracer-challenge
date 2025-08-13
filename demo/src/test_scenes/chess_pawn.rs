use crate::obj;
use dsl::still;
use math::tuple::color::{RED, WHITE};
use math::{degrees, matrix4x4, point};
use ray_tracer::camera::Camera;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::world::{BoundingVolumeDebug, World};
use ray_tracer::{auto, plane, scene};
use ray_tracer::{light, look_at};
use std::default::Default;

still!(
    Pawn;
    file_name: "chess_pawn";
    camera: | size | {
        let mut camera = Camera::new(size, degrees!(35));
        camera.set_transform(look_at!(point!(4, 6, 8) => point!(0, 1.4, 0)));
        camera
    };
    world: | world: &mut World | {
        world.render_preferences.bounding_volume_debug = BoundingVolumeDebug::Off;
    };
    scene: {
        scene!(
            +light!(point!(2, 20, 10));
            +plane!(pattern: Pattern::Checker(
                                *WHITE,
                                *RED,
                                Transform::new(matrix4x4!(scale_all(2.6) translation(0.5, 0., 0.5)))
                             );
            );
            +scene!(
                matrix: matrix4x4!(
                    //translation(2.6, 0., 0.,)
                    rotation_y(degrees!(-60))
                );
                bounding_volume: auto!();
                +obj!(
                    path: "objs/chess/pawn.obj";
                    material: Material::glass();
                );
            );
        )
    };
);
