use math::tuple::color::{Color, BLACK};

#[derive(Copy, Clone)]
pub enum BoundingVolumeDebug{
    Off,
    Translucent,
    Solid,
}

#[derive(Copy, Clone)]
pub struct RenderPreferences{
    pub max_ray_generation: u32,
    pub background: Color,
    pub bounding_volume_debug: BoundingVolumeDebug,
}

impl Default for RenderPreferences{
    fn default() -> Self {
        Self{
            background: Default::default(),
            max_ray_generation: 10,
            bounding_volume_debug: BoundingVolumeDebug::Off,
        }
    }
}
