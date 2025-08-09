#[macro_export]
macro_rules! still {
    (
        $name:tt;
        file_name: $file_name:expr;
        $(size: $size:expr;)?
        camera: $camera:expr;
        world: $world:expr;
        scene: $scene:expr;
    ) => {
        pub struct $name;

        impl $crate::TestScene for $name {
            fn name(&self) -> &'static str {
                $file_name
            }

            $(
            fn default_size(&self) -> Option<ray_tracer::canvas::Size> {
                Some($size.into())
            }
            )?

            fn build_world(&self) -> ray_tracer::world::World {
                let mut world = ray_tracer::world::World::default();
                $world(&mut world);
                world.add($scene);
                world
            }

            fn build_camera(&self, size: ray_tracer::canvas::Size) -> Camera {
                $camera(size)
            }
        }
    };
}
