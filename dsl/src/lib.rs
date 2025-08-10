mod still_scene_macro;

use animation::AnimationSpec;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::world::World;
use std::ops::Deref;
use std::time::Duration;

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
