pub mod glass_sphere_with_air;
mod cube_of_spheres;

use crate::chapter7_scene::ray_trace_end_chapter_7_scene;
use crate::fill_all_with_gradient;
use crate::png_write::PngWrite;
use crate::threaded_canvas::ThreadedCanvas;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::{Size, ViewPort};
use ray_tracer::world::World;
use ray_tracer::world::render_world::RenderWorld;
use std::fmt::Display;
use std::ops::DerefMut;
use std::path::Path;
use std::time::Instant;

pub trait TestScene {
    fn build_world() -> World;

    fn build_camera(size: Size)->Camera;
}

pub trait RenderTestScene<T> {
    fn render_scene<Q>(size: Size, name: Q)
    where
        Q: AsRef<Path> + Display;
}

impl<T:TestScene> RenderTestScene<T> for T {
    fn render_scene<Q>(size: Size, name: Q)
    where
        Q: AsRef<Path> + Display,
    {
        const BLOCK_SIZE: u32 = 32;
        let mut canvas = ThreadedCanvas::new(size, BLOCK_SIZE);
        fill_all_with_gradient(canvas.deref_mut());
        let world = T::build_world();
        let camera = T::build_camera(size);

        let now = Instant::now();
        canvas.render(&world, &camera);
        let duration = now.elapsed();
        let pixels = canvas.size().width() * canvas.size().height();
        println!(
            "Ray trace, block size {} took: {} ms {} px/sec",
            BLOCK_SIZE,
            duration.as_millis(),
            pixels as f32 / duration.as_secs_f32()
        );
        canvas.save_png(name.as_ref());
        println!("Saved image to {}", name);
    }

}