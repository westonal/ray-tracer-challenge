use crate::chapter7_scene::ray_trace_end_chapter_7_scene;
use crate::image_buffer_canvas::ImageBufferCanvas;
use crate::png_write::PngWrite;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::color::Color;
use math::{color, degrees, point};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Canvas;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::Shape;
use ray_tracer::world::World;
use ray_tracer::world::render_world::RenderWorld;
use std::time::Instant;

mod chapter7_scene;
mod image_buffer_canvas;
mod png_write;

fn main() {
    let mut canvas = ImageBufferCanvas::new(600, 400);
    fill_all_with_gradient(&mut canvas);
    let now = Instant::now();
    ray_trace_end_chapter_7_scene(&mut canvas);
    // ray_trace_with_lighting(&mut canvas);
    let duration = now.elapsed();
    let pixels = canvas.width() * canvas.height();
    println!(
        "Ray trace took: {} ms {} px/sec",
        duration.as_millis(),
        pixels as f32 / duration.as_secs_f32()
    );
    canvas.save_png("demo.png");
    println!("Saved image to `demo.png`");
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

fn ray_trace_with_lighting<C: Canvas<Color>>(canvas: &mut C) {
    let mut material = Material::default();
    material.pattern = Pattern::Solid(color!(1., 0.5, 1.));
    let mut sphere = Shape::new_sphere_transformed(Matrix4x4::translation(0., 0., -2.0_f32.sqrt()));
    sphere.material = material;
    let mut world = World::default();
    world.add(sphere);
    world.set_light(PointLight::new(point!(10, 10, 7), color!(0., 0.9, 1.)));
    let world = world;

    let camera = Camera::new((canvas.width(), canvas.height()), degrees!(90));
    canvas.render(&world, &camera);
}
