use crate::Material;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::color::{BLUE, Color, GREEN, RED, WHITE};
use math::{Angle, color, degrees, point, vector};
use ray_tracer::Transform::Transform;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Canvas;
use ray_tracer::gradient_stops;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::Shape;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::world::render_world::RenderWorld;

pub fn ray_trace_end_chapter_7_scene<C: Canvas<Color>>(canvas: &mut C) {
    let mut world = World::default();
    world.add(floor());
    world.add(wall(degrees!(-45))); // left wall
    world.add(wall(degrees!(45))); // right wall
    world.add(green_sphere());
    world.add(small_green_sphere());
    world.add(smallest_yellow_sphere());
    world.set_light(PointLight::new(point!(-10, 10, -10), color!(1, 1, 1)));
    let world = world;

    let mut camera = Camera::new((canvas.width(), canvas.height()), degrees!(60));
    camera.set_transform(
        ViewMatrix::new_look_at(point!(0, 1.5, -5), point!(0, 1, 0), vector!(0, 1, 0)).into(),
    );

    canvas.render(&world, &camera);
}

fn floor() -> Shape {
    let mut floor = Shape::new_plane();
    let mut material = Material::default();
    material.pattern = Pattern::Checker(
        color!(1, 0.9, 0.9),
        color!(1, 0, 1),
        Transform::new(Matrix4x4::rotation_y(degrees!(45)).pre_scale_all(0.25)),
    );
    material.specular = 0.;
    floor.material = material;
    floor
}

fn wall(y: Angle) -> Shape {
    let mut wall = Shape::new_plane_transformed(
        Matrix4x4::translation(0., 0., 5.)
            .pre_rotation_y(y)
            .pre_rotation_x(degrees!(90)),
    );
    let mut material = Material::default();
    material.pattern = Pattern::Stripe(color!(1, 0.9, 0.9), color!(1, 0, 1), Transform::identity());
    material.specular = 0.;
    wall.material = material;
    wall
}

fn green_sphere() -> Shape {
    let mut sphere = Shape::new_sphere_transformed(Matrix4x4::translation(-0.5, 1., 0.5));
    let mut material = Material::default();
    material.pattern = Pattern::Stripe(
        color!(0.1, 1, 0.5),
        *RED,
        Transform::new(Matrix4x4::scale(0.1, 0.1, 0.1).pre_rotation_z(degrees!(45))),
    );
    material.diffuse = 0.7;
    material.specular = 0.3;
    sphere.material = material;
    sphere
}

fn small_green_sphere() -> Shape {
    let mut sphere = Shape::new_sphere_transformed(
        Matrix4x4::translation(1.5, 0.5, -0.5)
            .pre_scale_all(0.5)
            .pre_rotation_y(degrees!(30))
            .pre_rotation_x(degrees!(30)),
    );
    let mut material = Material::default();
    material.pattern = Pattern::Gradient(
        gradient_stops!(
            0. => *RED,
            0.25 => *RED,
            0.5 => *BLUE,
            0.75 => *GREEN,
            1. => *GREEN),
        Transform::new(
            Matrix4x4::rotation_z(degrees!(90))
                .pre_translation(-1., 0., 0.)
                .pre_scale_all(2.),
        ),
    );
    material.diffuse = 0.7;
    material.specular = 0.3;
    sphere.material = material;
    sphere
}

fn smallest_yellow_sphere() -> Shape {
    let mut sphere = Shape::new_sphere_transformed(
        Matrix4x4::translation(-1.5, 0.33, -0.75)
            .pre_scale_all(0.33)
            .pre_rotation_y(degrees!(30))
            .pre_rotation_x(degrees!(-30)),
    );
    let mut material = Material::default();
    material.pattern = Pattern::Checker(
        color!(1, 0.8, 0.1),
        *WHITE,
        // TODO: BUG, without some scale, the pattern does not align exactly
        //  Even though it's a unit sphere and unit repeating pattern.
        Transform::new(Matrix4x4::scale_all(1.2)),
    );
    material.diffuse = 0.7;
    material.specular = 0.3;
    sphere.material = material;
    sphere
}
