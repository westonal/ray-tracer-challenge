use clap::builder::Str;
use clap::{Arg, arg, command};
use demo::test_scenes::chapter7_scene::Chapter7Scene;
use demo::test_scenes::chess_pawn::Pawn;
use demo::test_scenes::chess_queen::Queen;
use demo::test_scenes::csg::Csg;
use demo::test_scenes::cube_of_spheres::CubeOfSpheres;
use demo::test_scenes::cubes::Cubes;
use demo::test_scenes::cylinders::Cylinders;
use demo::test_scenes::glass_sphere_with_air::GlassSphereWithAir;
use demo::test_scenes::grid::Grid;
use demo::test_scenes::teapot::Teapot;
use demo::test_scenes::triangles::Triangles;
use demo::test_scenes::{RenderTestScene, TestScene};
use math::pub_static_color;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::world::World;
use std::sync::LazyLock;

struct BuiltScene {
    name: &'static str,
    file_name: &'static str,
    world_factory: Box<dyn Fn() -> World + Send + Sync>,
    camera_factory: Box<dyn Fn(Size) -> Camera + Send + Sync>,
}

macro_rules! scenes {
    ($($name:ident)+) => {
        vec!(
            $(
                BuiltScene {
                    name: stringify!($name),
                    file_name: $name::name(),
                    world_factory: Box::new(|| $name::build_world()),
                    camera_factory: Box::new(|size| $name::build_camera(size)),
                },
            )+
        )
    };
}

static ALL_SCENES: LazyLock<Vec<BuiltScene>> = LazyLock::new(|| {
    let mut scenes = scenes!(
       Csg
       Teapot
       Pawn
       Queen
       Triangles
       Chapter7Scene
       GlassSphereWithAir
       Cubes
       Cylinders
       CubeOfSpheres
       Grid
    );
    scenes.sort_by(|a, b| a.file_name.to_lowercase().cmp(&b.file_name.to_lowercase()));
    scenes
});

fn main2() {
    let size = Size::HD_720P;
    for scene in ALL_SCENES.iter() {
        println!("{}", scene.file_name);
    }
}

fn main() {
    let mut matches = command!().arg(arg!([x] "s"));

    for (n, scene) in ALL_SCENES.iter().enumerate() {
        // TODO: HOW MAKE OPTION?
        let arg = Arg::default()
            .id(format!("{}", scene.name))
            .long(format!("{}", scene.name.to_lowercase()))
            .alias(format!("{}", n + 1))
            .help(format!("{}", scene.file_name));
        matches = matches.arg(arg);
    }

    let matches = matches.get_matches();

    if let Some(x) = matches.get_one::<String>("x") {
        println!("Value for name: {x}");
    }
    for scene in ALL_SCENES.iter() {
        println!("{}", scene.file_name);
    }
}
