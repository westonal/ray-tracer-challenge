use dsl::still;
use math::{color, degrees, matrix4x4, point, radians, scale, translate, vector};
use ray_tracer::camera::Camera;
use ray_tracer::{cube, light, look_at, material, plane, scene, sphere};

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
    size: (1280, 1280);
    camera: |s| {
        let mut camera = Camera::new(s, radians!(0.785));
        camera.set_transform(look_at!(point!(-6, 6, -10) => point!(6,0,6); up: vector!(-0.45,1,0)));
        camera
    };
    scene: {
        let white = material!(
            color: color!(1, 1, 1);
            diffuse: 0.7;
            ambient: 0.1;
            specular: 0;
            reflectivity: 0.1;
        );
        let blue = material!(
            base: white;
            color: (0.537, 0.831, 0.914);
        );
        let red = material!(
            base: white;
            color: (0.941, 0.322, 0.388);
        );
        let purple = material!(
            base: white;
            color: (0.373, 0.404, 0.550);
        );
        let standard_transform = matrix4x4!(
            translation(1.0, -1.0, 1.0)
            scale_all(0.5)
        );
        let large = standard_transform * scale!(3.5);
        let medium = standard_transform * scale!(3);
        let small = standard_transform * scale!(2);
        scene!(
            +light!(point!(50, 100, -50));
            +light!(point!(-400, 50, -10), color!(0.2, 0.2, 0.2));
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
                material: white;
                size: medium;
                transform: translate!(x: 4;);
            );
            +cover_cube!(
                material: blue;
                size: large;
                transform: translate!(8.5, 1.5, -0.5);
            );
            +cover_cube!(
                material: red;
                size: large;
                transform: translate!(z: 4;);
            );
            +cover_cube!(
                material: white;
                size: small;
                transform: translate!(x: 4; z: 4;);
            );
            +cover_cube!(
                material: purple;
                size: medium;
                transform: translate!(7.5, 0.5, 4);
            );
            +cover_cube!(
                material: white;
                size: medium;
                transform: translate!(-0.25, 0.25 , 8);
            );
            +cover_cube!(
                material: blue;
                size: large;
                transform: translate!(4, 1, 7.5);
            );
            +cover_cube!(
                material: red;
                size: medium;
                transform: translate!(10, 2, 7.5);
            );
            +cover_cube!(
                material: white;
                size: small;
                transform: translate!(8, 2, 12);
            );
            // Cubes on page 253
            +cover_cube!(
                material: white;
                size: small;
                transform: translate!(20, 1, 9);
            );
            +cover_cube!(
                material: blue;
                size: large;
                transform: translate!(-0.5, -5, 0.25);
            );
            +cover_cube!(
                material: red;
                size: large;
                transform: translate!(4, -4, 0);
            );
            +cover_cube!(
                material: white;
                size: large;
                transform: translate!(8.5, -4, 0);
            );
            +cover_cube!(
                material: white;
                size: large;
                transform: translate!(0, -4, 4);
            );
            +cover_cube!(
                material: purple;
                size: large;
                transform: translate!(-0.5, -4.5, 8);
            );
            +cover_cube!(
                material: white;
                size: large;
                transform: translate!(0, -8, 4);
            );
            +cover_cube!(
                material: white;
                size: large;
                transform: translate!(-0.5, -8.5, 8);
            );
        )
    };
);
