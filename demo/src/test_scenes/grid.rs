use dsl::still;
use math::tuple::color::WHITE;
use math::{color, degrees, matrix4x4, point, scale, translate};
use ray_tracer::camera::Camera;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::scene_tree::SceneTree;
use ray_tracer::world::{BoundingVolumeDebug, World};
use ray_tracer::{auto, cube, scene, sphere};

still!(
    Grid;
    file_name: "grid_of_spheres";
    camera: | size| Camera::new(size, degrees!(120));
    world: | world: &mut World | {
        world.add_light(PointLight::new(point!(10, 10, 7), *WHITE));
        world.render_preferences.bounding_volume_debug = BoundingVolumeDebug::TranslucentEmpty;
    };
    scene: {
        let mut root = SceneTree::default();
        let x_count: i32 = 11;
        let y_count: i32 = 7;
        let scale = 0.4;
        for x in -x_count..(x_count + 1) {
            let mut column = scene!(
                matrix: translate!(x: x;);
                bounding_volume: auto!();
            );
            for y in -y_count..(y_count + 1) {
                let pattern = Pattern::Solid(if x == 0 && y == 0 {
                    color!(1, 0.2, 0.2)
                } else if x.abs() == x_count - 1 || y.abs() == y_count - 1 {
                    color!(0, 1.0, 0.1)
                } else {
                    color!(0, 0.5, 0.9)
                });
                column.add(sphere!(
                    matrix: translate!(y: y;) * scale!(scale);
                    pattern: pattern;
                ));
            }
            root.add(column);
        }
        scene!(
            matrix: translate!(z: -5.3;);
            +root;
        )
    };
);
