use crate::Material;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::color::Color;
use math::{Angle, color, degrees, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Canvas;
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
    let mut floor = Shape::new_plane_transformed(Matrix4x4::scale(10., 0.01, 10.));
    let mut material = Material::default();
    material.pattern = Pattern::Solid(color!(1, 0.9, 0.9));
    material.specular = 0.;
    floor.material = material;
    floor
}

fn wall(y: Angle) -> Shape {
    let mut wall = Shape::new_plane_transformed(
        Matrix4x4::translation(0., 0., 5.)
            .pre_rotation_y(y)
            .pre_rotation_x(degrees!(90))
            .pre_scale(10., 0.01, 10.),
    );
    let mut material = Material::default();
    material.pattern = Pattern::Solid(color!(1, 0.9, 0.9));
    material.specular = 0.;
    wall.material = material;
    wall
}

fn green_sphere() -> Shape {
    let mut sphere = Shape::new_sphere_transformed(Matrix4x4::translation(-0.5, 1., 0.5));
    let mut material = Material::default();
    material.pattern = Pattern::Solid(color!(0.1, 1, 0.5));
    material.diffuse = 0.7;
    material.specular = 0.3;
    sphere.material = material;
    sphere
}

fn small_green_sphere() -> Shape {
    let mut sphere = Shape::new_sphere_transformed(
        Matrix4x4::translation(1.5, 0.5, -0.5).pre_scale(0.5, 0.5, 0.5),
    );
    let mut material = Material::default();
    material.pattern = Pattern::Solid(color!(0.5, 1, 0.1));
    material.diffuse = 0.7;
    material.specular = 0.3;
    sphere.material = material;
    sphere
}

fn smallest_yellow_sphere() -> Shape {
    let mut sphere = Shape::new_sphere_transformed(
        Matrix4x4::translation(-1.5, 0.33, -0.75).pre_scale(0.33, 0.33, 0.33),
    );
    let mut material = Material::default();
    material.pattern = Pattern::Solid(color!(1, 0.8, 0.1));
    material.diffuse = 0.7;
    material.specular = 0.3;
    sphere.material = material;
    sphere
}
