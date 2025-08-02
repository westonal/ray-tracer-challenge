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
use demo::test_scenes::{DynamicScene, RenderTestScene, TestScene};
use ray_tracer::canvas::Size;
use std::ops::Deref;
use std::process::exit;
use std::sync::LazyLock;
use demo::test_scenes::satisfying_conveyor::SatisfyingConveyor;

struct BuiltScene {
    name: String,
    file_name: String,
    cli_argument_name: String,
    test_scene: DynamicScene,
    animation_spec: Option<AnimationSpec>,
}

impl Deref for BuiltScene {
    type Target = dyn TestScene;

    fn deref(&self) -> &Self::Target {
        self.test_scene.deref()
    }
}

macro_rules! scenes {
    ($($name:ident)+) => {
        {
            let mut all_scenes = vec![];
            $(
                let animation_spec = $name.animation_spec();
                let extension = match animation_spec {
                    None => ".png",
                    Some(_) => ".mp4",
                };

                let name = stringify!($name);

                all_scenes.push(BuiltScene {
                            name: name.to_string(),
                            file_name: format!("{}{}", $name.name(), extension),
                            cli_argument_name: name.to_string(),
                            test_scene: DynamicScene::new(Box::new($name)),
                            animation_spec: animation_spec,
                        });

                for (i, sub) in $name.sub_scenes().into_iter().enumerate() {
                    let sub_scene = sub.deref();
                    let animation_spec = sub_scene.animation_spec();
                    let extension = match (animation_spec) {
                        None => ".png",
                        Some(_) => ".mp4",
                    };
                    all_scenes.push(BuiltScene {
                            name: format!("{}.part_{}", name, i + 1),
                            file_name: format!("{}{}", sub_scene.name(), extension),
                            cli_argument_name: format!("{}.{}", name, i + 1),
                            test_scene: sub,
                            animation_spec: animation_spec,
                        });
                }
            )+
            all_scenes
        }
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
    );
    scenes.sort_by(|a, b| a.cli_argument_name.cmp(&b.cli_argument_name));
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
            .long(format!("{}", scene.cli_argument_name.to_lowercase()))
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
        if (all && scene.animation_spec.is_none()) || matches.get_flag(&scene.name) {
            scene
                .test_scene
                .deref()
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
