use dsl::still;
use math::tuple::color::{BLACK, WHITE};
use math::tuple::vector::POSITIVE_Z;
use math::{degrees, matrix4x4, point, scale, translate};
use ray_tracer::camera::Camera;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::{cube, plane, scene, sphere};
use ray_tracer::{light, look_at};

still!(
    GlassSphereWithAir;
    file_name: "glass_sphere_with_air";
    camera: |s| {
        let mut camera = Camera::new(s, degrees!(20));
        camera.set_transform(look_at!(point!(0, 40, 0) => point!(); up: *POSITIVE_Z));
        camera
    };
    scene: {
        scene!(
            +light!(point!(-300, 200, 20));
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
