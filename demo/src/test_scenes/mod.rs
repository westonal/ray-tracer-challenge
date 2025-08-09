pub mod book_cover;
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
pub mod satisfying_conveyor;
pub mod satisfying_pipes;
pub mod satisfying_pipes_raising;
pub mod teapot;
pub mod teapot_animated;
pub mod triangles;

use crate::png_write::PngWrite;
use crate::threaded_canvas::ThreadedCanvas;
use animation::AnimationSpec;
use ray_tracer::RenderWorld;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::{Size, ViewPort};
use ray_tracer::world::World;
use std::ops::Deref;
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct AnimationFrame {
    /// One-based frame number
    pub number: u32,

    /// If nested, what sub-scene is this
    pub sub_scene: u32,

    /// One-based frame number
    pub last_frame_number: u32,

    /// Current time starting at 0 for frame #1
    pub time: Duration,

    /// Frame position in a looping animation, does not reach 1 as that is the same as 0
    pub loop_progress: f32,

    /// Frame position in animation
    pub progress: f32,

    /// How long each frame takes, for motion blur etc
    pub exposure: Duration,
}

impl Default for AnimationFrame {
    fn default() -> Self {
        Self {
            number: 0,
            sub_scene: 0,
            last_frame_number: 0,
            time: Duration::from_secs(0),
            progress: 1.0,
            loop_progress: 0.0,
            exposure: Duration::from_secs(0),
        }
    }
}

pub struct DynamicScene(Box<dyn TestScene + Send + Sync>);

impl DynamicScene {
    pub fn new(scene: Box<dyn TestScene + Send + Sync>) -> Self {
        Self(scene)
    }
}

impl Deref for DynamicScene {
    type Target = dyn TestScene;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

pub trait TestScene {
    fn name(&self) -> &'static str;

    fn default_size(&self) -> Option<Size> {
        None
    }

    fn animation_spec(&self) -> Option<AnimationSpec> {
        None
    }

    fn build_world_for_frame(&self, _frame: &AnimationFrame) -> World {
        self.build_world()
    }

    fn build_world(&self) -> World {
        self.build_world_for_frame(&AnimationFrame::default())
    }

    fn build_camera_for_frame(&self, size: Size, _frame: &AnimationFrame) -> Camera {
        self.build_camera(size)
    }

    fn build_camera(&self, size: Size) -> Camera {
        self.build_camera_for_frame(size, &AnimationFrame::default())
    }

    fn sub_scenes(&self) -> Vec<DynamicScene> {
        vec![]
    }
}

pub trait RenderTestScene<T: ?Sized> {
    fn render_scene(&self, size: Size, allow_animation: bool);
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
    fn render_animation_scene_to(&self, size: Size, path: &str, spec: &AnimationSpec);
}

impl<T: TestScene + ?Sized> RenderTestSceneAnimated<T> for T {
    fn render_animation_scene_to(&self, size: Size, path: &str, spec: &AnimationSpec) {
        let frames = spec.build_frames();
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

pub trait Frames {
    fn build_frames(&self) -> Vec<AnimationFrame>;
}

impl Frames for AnimationSpec {
    fn build_frames(&self) -> Vec<AnimationFrame> {
        let mut result = vec![];
        let time_step_per_frame = self.per_frame_time_step();
        let frame_count = self.frame_count();
        let mut animation_frame = AnimationFrame {
            number: 1,
            sub_scene: 0,
            last_frame_number: frame_count,
            time: Duration::from_secs(0),
            loop_progress: 0.0,
            progress: 0.0,
            exposure: time_step_per_frame,
        };
        while animation_frame.number <= frame_count {
            // Maps [0..frame_count-1) to [0..1]
            animation_frame.loop_progress =
                (animation_frame.number - 1) as f32 / frame_count as f32;
            // Maps [0..frame_count-1] to [0..1]
            animation_frame.progress =
                (animation_frame.number - 1) as f32 / (frame_count - 1) as f32;
            result.push(animation_frame.clone());
            // Advance time and frame only after pushing
            animation_frame.number += 1;
            animation_frame.time += time_step_per_frame;
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
}

impl<T: TestScene + ?Sized> RenderTestScene<T> for T {
    fn render_scene(&self, size: Size, allow_animation: bool) {
        let name = self.name();
        let animation_spec = self.animation_spec();
        if allow_animation && let Some(animation_spec) = animation_spec {
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
