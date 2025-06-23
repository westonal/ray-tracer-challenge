use crate::canvas::Canvas;
use crate::image_buffer_canvas::ImageBufferCanvas;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::Tuple;
use math::tuple::color::Color;
use math::tuple::point::Point;
use math::{color, point};
use ray_tracer::intersection::Intersect;
use ray_tracer::lighting::{Material, PointLight};
use ray_tracer::primatives::sphere::Sphere;
use ray_tracer::ray;
use ray_tracer::rays::Ray;
use ray_tracer::world::World;
use std::cmp::min;
use std::f32::consts::TAU;
use std::time::Instant;

mod canvas;
mod image_buffer_canvas;

fn main() {
    let mut canvas = ImageBufferCanvas::new(600, 400);
    fill_all_with_gradient(&mut canvas);
    //example(&mut canvas);
    //ray_trace_silhouette(&mut canvas);
    let now = Instant::now();
    ray_trace_with_lighting(&mut canvas);
    let duration = now.elapsed();
    let pixels = canvas.width() * canvas.height();
    println!(
        "Ray trace took: {} ms {} px/sec",
        duration.as_millis(),
        pixels as f32 / duration.as_secs_f32()
    );
    //draw_clock(&mut canvas);
    canvas.save_png("demo.png");
    println!("Saved image to `demo.png`");
}

fn draw_clock<C: Canvas<Color>>(canvas: &mut C) {
    for hour in 0..12 {
        let point = Point::origin();
        let radius = min(canvas.width(), canvas.height()) as f32 * 3. / 8.;
        let m = Matrix4x4::identity()
            .pre_translation(canvas.width() as f32 / 2., canvas.height() as f32 / 2., 0.)
            .pre_scale(radius, radius, 1.)
            .pre_rotation_z(TAU * hour as f32 / 12.)
            .pre_translation(0., -1., 0.);

        let tuple = m * point;
        canvas.write_color(tuple.x as u32, tuple.y as u32, Color::rgba(1., 1., 1., 1.));
    }
}

fn fill_all_with_gradient<C: Canvas<Color>>(canvas: &mut C) {
    for y in 0..canvas.height() {
        for x in 0..canvas.width() {
            let color = (
                (x as f32) / (canvas.width() as f32),
                (y as f32) / (canvas.height() as f32),
                0.0,
                1.0,
            )
                .into();
            canvas.write_color(x, y, color);
        }
    }
}

fn example<C: Canvas<Color>>(canvas: &mut C) {
    let vector = Tuple::point(10.0, 20.0, 3.0);
    let mut speed = Tuple::vector(2.0, -2.0, -0.1);
    let acceleration = Tuple::vector(0.0, 0.05, 0.0);
    for i in 0..50 {
        let point = vector + speed * i as f32;
        let color = (0.5, (i as f32) / 50.0, 1.0 - (i as f32) / 25.0, 1.0).into();
        canvas.write_color(point.x as u32, point.y as u32, color);
        speed = speed + acceleration;
    }
}

fn ray_trace_silhouette<C: Canvas<Color>>(canvas: &mut C) {
    let fov_y = TAU / 4.; // 90°
    let fov_x = apply_ratio(fov_y, canvas.ratio());
    let sphere = Sphere::new_transformed(Matrix4x4::translation(0., 0., -2.0_f32.sqrt()));
    let color = color!(1., 1., 0.);
    let ray = ray!(Point::origin(), (0., 0., 1.));
    for y in 0..canvas.height() {
        let y_norm = y as f32 / canvas.height() as f32 - 0.5;
        for x in 0..canvas.width() {
            let x_norm = x as f32 / canvas.width() as f32 - 0.5;
            let tube = Matrix4x4::identity()
                .pre_rotation_x(apply_ratio(fov_x, x_norm * 2.)) //TODO NOT RIGHT, not giving 45° as max I think
                .pre_rotation_y(apply_ratio(fov_y, y_norm * 2.));
            let ray2 = tube * ray;

            if !sphere.intersect(ray2).is_empty() {
                canvas.write_color(x, y, color);
            }
        }
    }
}

fn ray_trace_with_lighting<C: Canvas<Color>>(canvas: &mut C) {
    let mut material = Material::default();
    material.color = color!(1., 0.5, 1.);
    let mut sphere = Sphere::new_transformed(Matrix4x4::translation(0., 0., 2.0_f32.sqrt()));
    sphere.material = material;
    let mut world = World::default();
    world.add(sphere);
    let world = world;

    let light = PointLight::new(point!(-10, -10, -7), color!(1., 0.9, 1.));
    let light2 = PointLight::new(point!(0, -10, -7), color!(0., 0.9, 1.));

    let fov_y = TAU / 4.; // 90°
    let z = 0.5 / (fov_y / 2.0).tan();
    let fov_x = apply_ratio(fov_y, canvas.ratio());
    let ray = ray!(Point::origin(), (0., 0., 1.));
    for y in 0..canvas.height() {
        let y_norm = y as f32 / canvas.height() as f32 - 0.5;
        for x in 0..canvas.width() {
            let x_norm = x as f32 / canvas.width() as f32 - 0.5;
            let ray = ray!((0., 0., 0.), (x_norm * canvas.ratio(), y_norm, z));

            let intersections = world.intersect(ray);
            if let Some(hit) = intersections.hit() {
                let point = ray.position(hit.t);
                let color1 = hit.sphere.material.light(
                    &light,
                    ray.origin,
                    (-ray.direction).normalize(),
                    hit.sphere.normal_at(point),
                );
                let color2 = hit.sphere.material.light(
                    &light2,
                    ray.origin,
                    (-ray.direction).normalize(),
                    hit.sphere.normal_at(point),
                );

                canvas.write_color(x, y, color1 + color2);
            }
        }
    }
}

fn apply_ratio(angle_radians: f32, ratio: f32) -> f32 {
    let a = (angle_radians / 2.).tan().recip();
    let a2 = a * ratio;
    a2.recip().atan() * 2.
}
