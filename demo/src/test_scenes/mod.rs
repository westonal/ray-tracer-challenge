pub mod chapter7_scene;
pub mod chess_pawn;
pub mod chess_queen;
pub mod chess_queen_material_animated;
pub mod csg;
pub mod cube_of_spheres;
pub mod cubes;
pub mod cylinders;
pub mod glass_sphere_with_air;
pub mod grid;
pub mod teapot;
pub mod teapot_animated;
pub mod triangles;

use crate::png_write::PngWrite;
use crate::threaded_canvas::ThreadedCanvas;
use ray_tracer::RenderWorld;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::{Size, ViewPort};
use ray_tracer::world::World;
use std::process::Command;
use std::time::{Duration, Instant};

pub struct SceneTiming {
    duration: Duration,
    fps: f32,
}

#[derive(Clone)]
pub struct AnimationFrame {
    /// One-based frame number
    pub number: usize,

    /// One-based frame number
    pub last_frame_number: usize,

    /// Current time starting at 0 for frame #1
    pub time: Duration,

    /// Frame position in animation
    pub progress: f32,

    /// How long each frame takes, for motion blur etc
    pub exposure: Duration,
}

impl Default for AnimationFrame {
    fn default() -> Self {
        Self {
            number: 0,
            last_frame_number: 0,
            time: Duration::from_secs(0),
            progress: 1.0,
            exposure: Duration::from_secs(0),
        }
    }
}

pub trait TestScene {
    fn name(&self) -> &'static str;

    fn animation(&self) -> Option<SceneTiming> {
        None
    }

    fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
        self.build_world()
    }

    fn build_world(&self) -> World {
        self.build_world_for_frame(&AnimationFrame::default())
    }

    fn build_camera_for_frame(&self, size: Size, frame: &AnimationFrame) -> Camera {
        self.build_camera(size)
    }

    fn build_camera(&self, size: Size) -> Camera {
        self.build_camera_for_frame(size, &AnimationFrame::default())
    }
}

pub trait RenderTestScene<T: ?Sized> {
    fn render_scene(&self, size: Size);
    fn render_scene_to_at_time(
        &self,
        size: Size,
        path: Option<&str>,
        frame: Option<&AnimationFrame>,
    );
    fn render_scene_to(&self, size: Size, path: Option<&str>) {
        self.render_scene_to_at_time(size, path, None);
    }
}

pub trait RenderTestSceneAnimated<T: ?Sized> {
    fn render_animation_scene_to(&self, size: Size, path: &str, spec: &SceneTiming);
}

impl<T: TestScene + ?Sized> RenderTestSceneAnimated<T> for T {
    fn render_animation_scene_to(&self, size: Size, path: &str, spec: &SceneTiming) {
        let frames = build_frames(spec);
        let name = self.name();
        let frame_path = format!("{}{}_frames/", path, name);
        let path_prefix = format!("{}{}_", frame_path, name);
        let _ = std::fs::create_dir_all(&frame_path);
        for animation_frame in frames {
            let file_name = format!("{}{:04}.png", path_prefix, animation_frame.number);
            self.render_scene_to_at_time(size, Some(&file_name), Some(&animation_frame));
        }
        println!("Creating animation");
        let filename = format!("{}{}.mp4", path, name);
        let command = format!(
            "ffmpeg -y -f image2 -framerate \"{}\" -i \"{}%04d.png\" -vcodec libx264 -crf 23 -pix_fmt yuv420p -f mp4 \"{}\"",
            spec.fps, path_prefix, filename,
        );
        println!("{}", command);
        let result = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process");
        if result.status.success() {
            println!("Created animation file {}", filename);
        } else {
            panic!("Failed to create animation from frames")
        }
    }
}

fn build_frames(spec: &SceneTiming) -> Vec<AnimationFrame> {
    let mut result = vec![];
    let time_step_per_frame = Duration::from_millis((1000.0 / spec.fps) as u64);
    let mut animation_frame = AnimationFrame {
        number: 1,
        last_frame_number: 0,
        time: Duration::from_secs(0),
        progress: 0.0,
        exposure: time_step_per_frame,
    };
    while animation_frame.time <= spec.duration {
        result.push(animation_frame.clone());
        animation_frame.time += time_step_per_frame;
        animation_frame.progress = animation_frame.time.as_secs_f32() / spec.duration.as_secs_f32();
        animation_frame.number += 1;
    }
    let last_frame = result.last();
    if let Some(AnimationFrame { number, .. }) = last_frame {
        let number = *number;
        for animation_frame in result.iter_mut() {
            animation_frame.last_frame_number = number;
        }
    }
    result
}

impl<T: TestScene + ?Sized> RenderTestScene<T> for T {
    fn render_scene(&self, size: Size) {
        let name = self.name();
        let animation_spec = self.animation();
        if let Some(animation_spec) = animation_spec {
            self.render_animation_scene_to(size, "test_scenes/", &animation_spec);
        } else {
            let file_name = format!("test_scenes/{}.png", name);
            self.render_scene_to(size, Some(&file_name));
        }
    }

    fn render_scene_to_at_time(
        &self,
        size: Size,
        path: Option<&str>,
        frame: Option<&AnimationFrame>,
    ) {
        let name = self.name();
        if path.is_some() {
            match frame {
                None => println!("=== Rendering: {} at {} ===", name, size),
                Some(time) => println!(
                    "=== Rendering: {} at {} : Frame #{}/{} ===",
                    name, size, time.number, time.last_frame_number,
                ),
            };
        }
        const BLOCK_SIZE: u32 = 32;
        let mut canvas = ThreadedCanvas::new(size, BLOCK_SIZE);
        let (world, camera) = match frame {
            None => (self.build_world(), self.build_camera(size)),
            Some(frame) => (
                self.build_world_for_frame(frame),
                self.build_camera_for_frame(size, frame),
            ),
        };

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
