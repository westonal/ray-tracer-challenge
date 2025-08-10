use crate::Material;
use dsl::still;
use math::tuple::color::{BLUE, GREEN, RED, WHITE};
use math::{Angle, color, degrees, matrix4x4, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::Shape;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{gradient_stops, plane, scene, sphere};

still!(
    Chapter7Scene;
    file_name: "Chapter 7 Scene";
    camera: | size | {
        let mut camera = Camera::new(size, degrees!(60));
        camera.set_transform(ViewMatrix::new_look_at(
            point!(0, 1.5, -5),
            point!(0, 1, 0),
            vector!(0, 1, 0),
        ));
        camera
    };
    world: | world: &mut World | {
        world.set_light(PointLight::new(point!(-10, 10, -10), color!(1, 1, 1)));
    };
    scene: {
        scene!(
            +floor();
            +wall(degrees!(-45)); // left wall
            +wall(degrees!(45)); // right wall
            +green_sphere();
            +small_green_sphere();
            +smallest_yellow_sphere();
        )
    };
);

fn floor() -> Shape {
    let mut floor = plane!();
    let mut material = Material::default();
    material.pattern = Pattern::Checker(
        color!(1, 0.9, 0.9),
        color!(1, 0, 1),
        Transform::new(matrix4x4!(
            rotation_y(degrees!(45))
            scale_all(0.25))),
    );
    material.specular = 0.;
    material.reflectivity = 0.6;
    floor.material = material;
    floor
}

fn wall(y: Angle) -> Shape {
    let mut wall = plane!(matrix: matrix4x4!(
        translation(0., 0., 5.)
        rotation_y(y)
        rotation_x(degrees!(90))
    ));
    let mut material = Material::default();
    material.pattern = Pattern::Stripe(color!(1, 0.9, 0.9), color!(1, 0, 1), Transform::identity());
    material.specular = 0.;
    wall.material = material;
    wall
}

fn green_sphere() -> Shape {
    let mut sphere = sphere!(matrix: matrix4x4!(translation(-0.5, 1., 0.5)));
    let mut material = Material::default();
    material.reflectivity = 0.9;
    material.diffuse = 0.0;
    material.specular = 0.9;
    material.shininess = 100.;
    sphere.material = material;
    sphere
}

fn small_green_sphere() -> Shape {
    let mut sphere = sphere!(matrix: matrix4x4!(
        translation(1.5, 0.5, -0.5)
        scale_all(0.5)
        rotation_y(degrees!(30))
        rotation_x(degrees!(30))
    ));
    let mut material = Material::default();
    material.pattern = Pattern::Gradient(
        gradient_stops!(
            0. => *RED,
            0.25 => *RED,
            0.5 => *BLUE,
            0.75 => *GREEN,
            1. => *GREEN),
        Transform::new(matrix4x4!(
            rotation_z(degrees!(90))
            translation(-1., 0., 0.)
            scale_all(2.)
        )),
    );
    material.diffuse = 0.7;
    material.specular = 0.3;
    sphere.material = material;
    sphere
}

fn smallest_yellow_sphere() -> Shape {
    let mut sphere = sphere!(matrix: matrix4x4!(
        translation(-1.5, 0.33, -0.75)
        scale_all(0.33)
        rotation_y(degrees!(30))
        rotation_x(degrees!(-30)))
    );
    let mut material = Material::default();
    material.pattern = Pattern::Checker(color!(1, 0.8, 0.1), *WHITE, Transform::identity());
    material.diffuse = 0.7;
    material.specular = 0.3;
    sphere.material = material;
    sphere
}
