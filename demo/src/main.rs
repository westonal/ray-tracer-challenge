use animation::AnimationSpec;
use clap::builder::{IntoResettable, OsStr, Resettable};
use clap::{Arg, ArgAction, command, value_parser};
use demo::test_scenes::chapter7_scene::Chapter7Scene;
use demo::test_scenes::chess_pawn::Pawn;
use demo::test_scenes::chess_queen::Queen;
use demo::test_scenes::chess_queen_material_animated::QueenMaterialAnimation;
use demo::test_scenes::csg::Csg;
use demo::test_scenes::cube_of_spheres::CubeOfSpheres;
use demo::test_scenes::cubes::Cubes;
use demo::test_scenes::cylinders::Cylinders;
use demo::test_scenes::glass_sphere_with_air::GlassSphereWithAir;
use demo::test_scenes::grid::Grid;
use demo::test_scenes::satisfying_pipes::SatisfyingPipesAnimated;
use demo::test_scenes::satisfying_pipes_raising::SatisfyingPipesRaisingAnimated;
use demo::test_scenes::teapot::Teapot;
use demo::test_scenes::teapot_animated::TeapotAnimated;
use demo::test_scenes::triangles::Triangles;
use demo::test_scenes::{RenderTestScene, TestScene};
use ray_tracer::canvas::Size;
use std::ops::Deref;
use std::process::exit;
use std::sync::LazyLock;
use demo::test_scenes::satisfying_conveyor::{SatisfyingConveyor, SatisfyingConveyorPt2, SatisfyingConveyorFull};

struct BuiltScene {
    name: &'static str,
    file_name: String,
    test_scene: Box<dyn TestScene + Send + Sync>,
    animation_spec: Option<AnimationSpec>,
}

impl Deref for BuiltScene {
    type Target = dyn TestScene;

    fn deref(&self) -> &Self::Target {
        self.test_scene.as_ref()
    }
}

macro_rules! scenes {
    ($($name:ident)+) => {
        vec!(
            $(
                {
                    let spec = $name.animation_spec();
                    let extension = match (spec) {
                        None => ".png",
                        Some(_) => ".mp4",
                    };
                    BuiltScene {
                        name: stringify!($name),
                        file_name: format!("{}{}", $name.name(), extension),
                        test_scene: Box::new($name),
                        animation_spec: $name.animation_spec(),
                    }
                },
            )+
        )
    };
}

static ALL_SCENES: LazyLock<Vec<BuiltScene>> = LazyLock::new(|| {
    let mut scenes = scenes!(
        Csg
        Teapot
        TeapotAnimated
        Pawn
        Queen
        QueenMaterialAnimation
        Triangles
        Chapter7Scene
        GlassSphereWithAir
        Cubes
        Cylinders
        CubeOfSpheres
        Grid
        SatisfyingPipesAnimated
        SatisfyingPipesRaisingAnimated
        SatisfyingConveyor
        SatisfyingConveyorPt2
        SatisfyingConveyorFull
    );
    scenes.sort_by(|a, b| a.file_name.to_lowercase().cmp(&b.file_name.to_lowercase()));
    scenes
});

/// Size is defined outside this module, so we can't implement [IntoResettable] directly.
struct ClapSizeWrapper(Size);

impl Deref for ClapSizeWrapper {
    type Target = Size;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoResettable<OsStr> for ClapSizeWrapper {
    fn into_resettable(self) -> Resettable<OsStr> {
        Resettable::Value(self.to_string().into())
    }
}

fn main() {
    let mut command = command!()
        .arg(
            Arg::default()
                .id("all")
                .required(false)
                .long("all")
                .short('a')
                .action(ArgAction::SetTrue)
                .help("Render all png scenes, no animations"),
        )
        .arg(
            Arg::default()
                .id("no-anim")
                .required(false)
                .long("no-anim")
                .action(ArgAction::SetTrue)
                .help("Render just first frame of an animation"),
        )
        .arg(
            Arg::default()
                .id("size")
                .value_parser(value_parser!(Size))
                .required(false)
                .long("size")
                .short('s')
                .default_value(ClapSizeWrapper(Size::HD_720P))
                .help("Size of the frames"),
        );

    for (n, scene) in ALL_SCENES.iter().enumerate() {
        let help = match &scene.animation_spec {
            None => format!("{}", scene.file_name),
            Some(spec) => format!(
                "{}, <= {} seconds @ {} FPS --> {} frames",
                scene.file_name,
                spec.duration_limit.as_secs_f32(),
                spec.fps,
                spec.frame_count(),
            ),
        };
        let arg = Arg::default()
            .id(format!("{}", scene.name))
            .required(false)
            .long(format!("{}", scene.name.to_lowercase()))
            .alias(format!("{}", n + 1))
            .action(ArgAction::SetTrue)
            .help(help);
        command = command.arg(arg);
    }

    let matches = command.get_matches();

    let size = *matches.get_one::<Size>("size").unwrap();
    let all = matches.get_flag("all");
    let mut count = 0;
    for scene in ALL_SCENES.iter() {
        if (all && scene.animation_spec.is_none()) || matches.get_flag(scene.name) {
            scene
                .test_scene
                .as_ref()
                .render_scene(size, !matches.get_flag("no-anim"));
            count += 1;
        }
    }

    if count == 0 {
        println!("Specify a test scene to render, --all for all, or --help for more options");
        exit(1);
    } else {
        println!(
            "{} scene{} rendered",
            count,
            if count == 1 { "" } else { "s" }
        );
    }
}
