use math::{color, degrees, matrix4x4, point, radians, translate, vector};
use ray_tracer::camera::Camera;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern::Solid;
use ray_tracer::{cube, plane, scene, sphere};
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;

macro_rules! still {
    (
        $name:tt;
        file_name: $file_name:expr;
        camera: $camera:expr;
        world: $world:expr;
        scene: $scene:expr;
    ) => {
        pub struct $name;

        impl $crate::test_scenes::TestScene for $name {
            fn name(&self) -> &'static str {
                $file_name
            }

            fn build_world(&self) -> ray_tracer::world::World {
                let mut world = ray_tracer::world::World::default();
                $world(&mut world);
                world.add($scene);
                world
            }

            fn build_camera(&self, size: ray_tracer::canvas::Size) -> Camera {
                $camera(size)
            }
        }
    };
}

macro_rules! material {
    (
        $(base: $base:expr;)?
        $(color: $color:expr;)?
        $(diffuse: $diffuse:expr;)?
        $(ambient: $ambient:expr;)?
        $(specular: $specular:expr;)?
        $(shininess: $shininess:expr;)?
        $(transparency: $transparency:expr;)?
        $(reflectivity: $reflectivity:expr;)?
        $(refractive-index: $refractive_index:expr;)?
    ) => {
        {
            let mut _material = Material::default();
            $(_material = $base.clone();)?
            $(_material.pattern = ray_tracer::material::pattern::Pattern::Solid($color.into());)?
            $(_material.diffuse = $diffuse as f32;)?
            $(_material.ambient = $ambient as f32;)?
            $(_material.specular = $specular as f32;)?
            $(_material.shininess = $shininess as f32;)?
            $(_material.transparency = $transparency as f32;)?
            $(_material.reflectivity = $reflectivity as f32;)?
            $(_material.refractive_index = $refractive_index as f32;)?
            _material
        }
    };
}

macro_rules! cover_cube {
    (
        material: $material:expr;
        size: $size:expr;
        transform: $transform:expr;
    ) => {
        cube!(
            matrix: $transform * $size;
            material: $material;
        )
    };
}

still!(
    BookCover;
    file_name: "book-cover";
    camera: |s| {
        let mut camera = Camera::new(s, radians!(0.785));
        camera.set_transform(
            ViewMatrix::new_look_at(
                point!(-6, 6, -10),
                point!(6,0,6),
                vector!(-0.45,1,0),
            ).into());
        camera
    };
    world: | world: &mut World |{
        world.add_light(PointLight::new(point!(50, 100, -50), color!(1, 1, 1)));
        world.add_light(PointLight::new(point!(-400, 50, -10), color!(0.2, 0.2, 0.2)));
    };
    scene: {
        let white_material = material!(
            color: color!(1, 1, 1);
            diffuse: 0.7;
            ambient: 0.1;
            specular: 0;
            reflectivity: 0.1;
        );
        let blue_material = material!(
            base: white_material;
            color: (0.537, 0.831, 0.914);
        );
        let red_material = material!(
            base: white_material;
            color: (0.941, 0.322, 0.388);
        );
        let purple_material = material!(
            base: white_material;
            color: (0.373, 0.404, 0.550);
        );
        let standard_transform = matrix4x4!(
            translation(1.0, -1.0, 1.0)
            scale_all(0.5)
        );
        let large = standard_transform * matrix4x4!(scale_all(3.5));
        let medium = standard_transform * matrix4x4!(scale_all(3.0));
        let small = standard_transform * matrix4x4!(scale_all(2.0));
        scene!(
            +plane!(
                matrix: matrix4x4!(
                    translation(0., 0., 500.)
                    rotation_x(degrees!(90))
                );
                material: material!(
                    color: (1.0, 1.0, 1.0);
                    diffuse: 0;
                    ambient: 1;
                    specular: 0;
                );
            );
            +sphere!(
                matrix: large;
                material: material!(
                    color: (0.373, 0.404, 0.550);
                    diffuse: 0.2;
                    ambient: 0;
                    specular: 1;
                    shininess: 200;
                    transparency: 0.7;
                    reflectivity: 0.7;
                    refractive-index: 1.5;
                );
            );
            // Cubes on page 252
            +cover_cube!(
                material: white_material;
                size: medium;
                transform: translate!(x: 4;);
            );
            +cover_cube!(
                material: blue_material;
                size: large;
                transform: translate!(8.5, 1.5, -0.5);
            );
            +cover_cube!(
                material: red_material;
                size: large;
                transform: translate!(z: 4;);
            );
            +cover_cube!(
                material: white_material;
                size: small;
                transform: translate!(x: 4; z: 4;);
            );
            +cover_cube!(
                material: purple_material;
                size: medium;
                transform: translate!(7.5, 0.5, 4);
            );
            +cover_cube!(
                material: white_material;
                size: medium;
                transform: translate!(-0.25, 0.25 , 8);
            );
            +cover_cube!(
                material: blue_material;
                size: large;
                transform: translate!(4, 1, 7.5);
            );
            +cover_cube!(
                material: red_material;
                size: medium;
                transform: translate!(10, 2, 7.5);
            );
            +cover_cube!(
                material: white_material;
                size: small;
                transform: translate!(8, 2, 12);
            );
            // Cubes on page 253
            +cover_cube!(
                material: white_material;
                size: small;
                transform: translate!(20, 1, 9);
            );
            +cover_cube!(
                material: blue_material;
                size: large;
                transform: translate!(-0.5, -5, 0.25);
            );
            +cover_cube!(
                material: red_material;
                size: large;
                transform: translate!(4, -4, 0);
            );
            +cover_cube!(
                material: white_material;
                size: large;
                transform: translate!(8.5, -4, 0);
            );
            +cover_cube!(
                material: white_material;
                size: large;
                transform: translate!(0, -4, 4);
            );
            +cover_cube!(
                material: purple_material;
                size: large;
                transform: translate!(-0.5, -4.5, 8);
            );
            +cover_cube!(
                material: white_material;
                size: large;
                transform: translate!(0, -8, 4);
            );
            +cover_cube!(
                material: white_material;
                size: large;
                transform: translate!(-0.5, -8.5, 8);
            );
        )
    };
);
