use dsl::still;
use crate::test_scenes::TestScene;
use math::tuple::color::{BLACK, WHITE};
use math::tuple::point::Point;
use math::{color, degrees, matrix4x4, point, scale, translate, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{cube, plane, scene, sphere};

still!(
    GlassSphereWithAir;
    file_name: "glass_sphere_with_air";
    camera: |s| {
        let mut camera = Camera::new(s, degrees!(20));
        camera.set_transform(*ViewMatrix::new_look_at(
            point!(0, 40, 0),
            Point::origin(),
            vector!(0, 0, 1),
        ));
        camera
    };
    world: | world: &mut World | {
        world.add_light(PointLight::new(point!(-300, 200, 20), *WHITE));
    };
    scene: {
        scene!(
            +plane!(
                matrix: translate!(y: -32;);
                pattern: Pattern::Checker(*BLACK, *WHITE, Transform::new(scale!(3)));
            );
            +scene!(
                matrix: scale!(5);
                bounding_volume: cube!();
                +sphere!(material: Material::glass(););
                +scene!(
                    matrix: scale!(0.5);
                    +sphere!(material: Material::air(););
                );
            );
        )
    };
);
