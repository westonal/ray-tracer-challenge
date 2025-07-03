use crate::chapter7_scene::ray_trace_end_chapter_7_scene;
use crate::png_write::PngWrite;
use crate::test_scenes::RenderTestScene;
use crate::test_scenes::cube_of_spheres::CubeOfSpheres;
use crate::test_scenes::glass_sphere_with_air::GlassSphereWithAir;
use crate::test_scenes::grid::Grid;
use crate::threaded_canvas::ThreadedCanvas;
use math::tuple::color::Color;
use ray_tracer::canvas::{Canvas, Size, ViewPort};
use ray_tracer::material::Material;
use std::ops::DerefMut;
use std::time::Instant;

mod chapter7_scene;
mod image_buffer_canvas;
mod png_write;
mod test_scenes;
mod threaded_canvas;

fn main() {
    let size_multiplier = 2;
    let size = Size::new(600 * size_multiplier, 400 * size_multiplier);
    render(size, 32);
    let size = Size::HD_720P;
    GlassSphereWithAir::render_scene(size);
    CubeOfSpheres::render_scene(size);
    Grid::render_scene(size);
}

fn render(size: Size, block_size: u32) {
    let mut canvas = ThreadedCanvas::new(size, block_size);
    fill_all_with_gradient(canvas.deref_mut());
    let now = Instant::now();
    // Multithread
    ray_trace_end_chapter_7_scene(&mut canvas);
    // If passed as a regular canvas, that causes single thread rendering
    //ray_trace_end_chapter_7_scene(canvas.deref_mut());
    // ray_trace_with_lighting(&mut canvas);
    let duration = now.elapsed();
    let pixels = canvas.size().width() * canvas.size().height();
    println!(
        "Ray trace, block size {} took: {} ms {} px/sec",
        block_size,
        duration.as_millis(),
        pixels as f32 / duration.as_secs_f32()
    );
    canvas.save_png("demo.png");
    println!("Saved image to `demo.png`");
}

fn fill_all_with_gradient<C: Canvas<Color>>(canvas: &mut C) {
    let size = canvas.size();
    for y in 0..size.height() {
        for x in 0..size.width() {
            let color = (
                (x as f32) / (size.width() as f32),
                (y as f32) / (size.height() as f32),
                0.0,
                1.0,
            )
                .into();
            canvas.write_color(x, y, color);
        }
    }
}
