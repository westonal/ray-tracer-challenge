pub mod cube_of_spheres;
pub mod glass_sphere_with_air;
pub mod grid;

use crate::fill_all_with_gradient;
use crate::png_write::PngWrite;
use crate::threaded_canvas::ThreadedCanvas;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::{Size, ViewPort};
use ray_tracer::world::World;
use ray_tracer::world::render_world::RenderWorld;
use std::ops::DerefMut;
use std::time::Instant;

pub trait TestScene {
    fn name() -> &'static str;

    fn build_world() -> World;

    fn build_camera(size: Size) -> Camera;
}

pub trait RenderTestScene<T> {
    fn render_scene(size: Size);
}

impl<T: TestScene> RenderTestScene<T> for T {
    fn render_scene(size: Size) {
        let name = T::name();
        println!("=== Rendering: {} at {} ===", name, size);
        const BLOCK_SIZE: u32 = 32;
        let mut canvas = ThreadedCanvas::new(size, BLOCK_SIZE);
        fill_all_with_gradient(canvas.deref_mut());
        let world = T::build_world();
        let camera = T::build_camera(size);

        let now = Instant::now();
        canvas.render(&world, &camera);
        let duration = now.elapsed();
        let pixels = canvas.size().width() * canvas.size().height();
        println!("Threaded block size: {}", BLOCK_SIZE,);
        println!("Took: {:.3} s", duration.as_secs_f32(),);
        let (quantity, si) = format((pixels as f32 / duration.as_secs_f32()) as u32);
        println!("Rate {:.1} {}px/sec", quantity, si);
        let file_name = format!("test_scenes/{}.png", name);
        canvas.save_png(file_name.clone());
        println!("Saved image: {}", file_name);
        println!();
    }
}

fn format(large: u32) -> (f32, &'static str) {
    let mut l = large as f32;
    let mut prefix_idx = 0;
    let prefixes = vec!["", "k", "M", "G"];
    while l > 1000. && prefix_idx + 1 < prefixes.len() {
        l /= 1000.;
        prefix_idx += 1;
    }
    (l, prefixes.get(prefix_idx).unwrap())
}
