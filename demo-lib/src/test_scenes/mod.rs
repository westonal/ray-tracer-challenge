pub mod chapter7_scene;
pub mod chess_pawn;
pub mod chess_queen;
pub mod cube_of_spheres;
pub mod cubes;
pub mod cylinders;
pub mod glass_sphere_with_air;
pub mod grid;
pub mod teapot;
pub mod triangles;

use crate::png_write::PngWrite;
use crate::threaded_canvas::ThreadedCanvas;
use ray_tracer::RenderWorld;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::{Size, ViewPort};
use ray_tracer::world::World;
use std::time::Instant;

pub trait TestScene {
    fn name() -> &'static str;

    fn build_world() -> World;

    fn build_camera(size: Size) -> Camera;
}

pub trait RenderTestScene<T> {
    fn render_scene(size: Size);
    fn render_scene_to(size: Size, path: Option<&str>);
}

impl<T: TestScene> RenderTestScene<T> for T {
    fn render_scene(size: Size) {
        let name = T::name();
        let file_name = format!("test_scenes/{}.png", name);
        Self::render_scene_to(size, Some(&file_name));
    }

    fn render_scene_to(size: Size, path: Option<&str>) {
        let name = T::name();
        if path.is_some() {
            println!("=== Rendering: {} at {} ===", name, size);
        }
        const BLOCK_SIZE: u32 = 32;
        let mut canvas = ThreadedCanvas::new(size, BLOCK_SIZE);
        let world = T::build_world();
        let camera = T::build_camera(size);

        let now = Instant::now();

        let world = world.prepare_for_render();
        canvas.render(&world, &camera);
        let duration = now.elapsed();
        let pixels = canvas.size().width() * canvas.size().height();
        if path.is_some() {
            println!("Threaded block size: {}", BLOCK_SIZE,);
            println!("Took: {:.3} s", duration.as_secs_f32(),);
            let (quantity, si) = format((pixels as f32 / duration.as_secs_f32()) as u32);
            println!("Rate {:.1} {}px/sec", quantity, si);
        }
        if let Some(file_name) = path {
            canvas.save_png(file_name);
            println!("Saved image: {}", file_name);
            println!();
        }
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
