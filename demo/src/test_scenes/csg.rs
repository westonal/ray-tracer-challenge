use dsl::still;
use math::tuple::color::{BLACK, BLUE, RED, WHITE};
use math::{degrees, matrix4x4, point, scale, vector};
use ray_tracer::camera::Camera;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{cube, material, plane, scene, sphere};

still!(
    Csg;
    file_name: "csg";
    camera: | size | {
        let mut camera = Camera::new(size, degrees!(30));
        camera.set_transform(ViewMatrix::new_look_at(
            point!(17, 19, 23),
            point!(0, -1, -3),
            vector!(0, 1, 0),
        ));
        camera
    };
    world: | world: &mut World | {
        world.push(PointLight::new(point!(40, 40, 20), *WHITE * 0.9));
    };
    scene: {
        scene!(
            +plane!(
                material: material!(reflectivity: 0.5;);
                pattern: Pattern::Checker(
                    *WHITE,
                    *BLACK,
                    Transform::new(matrix4x4!(rotation_y(degrees!(45)) scale_all(2.))),
                );
            );
            +scene!(
                matrix: matrix4x4!(
                            translation(0., 4.001, 0.)
                            scale_all(4.)
                        );
                bounding_volume: cube!();
                +cube!() - sphere!(matrix: scale!(1.25); pattern: Pattern::Solid(*RED);)
                    &
                 sphere!(matrix: scale!(1.45); pattern: Pattern::Solid(*BLUE););
                // Glass core
                +sphere!(matrix: scale!(1.2499); material: Material::glass();)
                   &
                 cube!(material: Material::glass(););
            );
        )
    };
);
