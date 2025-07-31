use crate::obj;
use crate::test_scenes::{AnimationFrame, AnimationSpec, TestScene};
use animation::animation_spec;
use math::tuple::color::{BLACK, RED, YELLOW};
use math::{color, degrees, matrix4x4, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::material::Material;
use ray_tracer::scene_tree::SceneTree;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{cube, cylinder, plane, scene, sphere};
use std::default::Default;
use std::f32::consts::PI;
use std::time::Duration;

pub struct SatisfyingConveyor;

impl TestScene for SatisfyingConveyor {
    fn name(&self) -> &'static str {
        "satisfying-conveyor"
    }

    fn animation_spec(&self) -> Option<AnimationSpec> {
        Some(animation_spec!(4;seconds @25;fps))
    }

    fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
        let mut factory = SatisfyingPipesAnimatedFactory::new(Mode::Efficient);
        factory.floor_height = 1.0;
        let factory = factory;
        let progress = frame.loop_progress;

        let world_scene = scene!(
            +plane!(
                matrix: matrix4x4!(
                    translation(0., -factory.floor_height, 0.)
                );
                pattern: Pattern::Checker(color!(0.3, 0.3, 0.3), color!(0.7, 0.7, 0.7), Transform::identity());
            );
            +factory.pawn.clone();
            +factory.half_world();
            +scene!(
                matrix: matrix4x4!(
                    scale(-1., 1., 1.)
                );
                +factory.half_world();
            );
        );

        let mut world = World::default();
        world.add(scene!(
            +world_scene;
        ));
        world.add_light(PointLight::new(point!(-5, 20, 10), color!(1, 1, 1)));
        world
    }

    fn build_camera(&self, size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(30));
        let top_down = ViewMatrix::new_look_at(point!(0, 10, 0), point!(0, 0, 0), vector!(0, 0, 1));
        let top_down_high =
            ViewMatrix::new_look_at(point!(0, 30, 0), point!(0, 0, 0), vector!(0, 0, 1));
        let front_on =
            ViewMatrix::new_look_at(point!(-10, 10, 20), point!(0, 0.5, 0), vector!(0, 1, 0));
        camera.set_transform(front_on.into());
        camera
    }
}

//

#[derive(PartialEq)]
enum Mode {
    Efficient,
    Final,
}

struct SatisfyingPipesAnimatedFactory {
    mode: Mode,
    pipe_length: f32,
    floor_height: f32,
    pawn: SceneTree,
}

impl SatisfyingPipesAnimatedFactory {

    fn half_world(&self) -> SceneTree {
        scene!(
        matrix: matrix4x4!(
            //scale(1., 0.5, 2.)
            translation(1., 0., 0.)
        );
        // bounding_volume: cube!(
        //
        // );
        +cube!(
            matrix: matrix4x4!(
                scale(1., 0.5, 2.)
                translation(0., -1., 0.)
            );
            pattern: Pattern::Stripe(*YELLOW, *BLACK, Transform::new(
                matrix4x4!(
                    rotation_y(degrees!(-60))
                    scale_all(0.5)
                    // scale_all(0.5)
                    // scale(1., 1., 1. / 10.)
                )
            ))
        );
            // die stamp block
        +scene!(
                // todo motion
            matrix: matrix4x4!(translation(1., 0., 0.));
            material_override: Material::glass();
            +cube!(
                matrix: matrix4x4!(
                        scale(1., 1.6, 1.)
                        translation(0., 1., 0.)
                    );
                    //pattern: Pattern::Solid(*RED);
            ) - scene!(
                    matrix: matrix4x4!(translation(-1., 0., 0.));
                    +self.silver_pawn();
            );
            );
            // TODO IDEA - inject mould
            // +self.silver_pawn() & sphere!(
            //     matrix: matrix4x4!(scale(1., 1.9, 1.));
            //     pattern: Pattern::Solid(*RED));
    )
    }

    fn silver_pawn(&self) -> SceneTree {
        let mut pawn = self.pawn.clone();
        // TODO MUST HAVE A MATERIAL OVERRIDE NODE
        pawn
    }
}

impl SatisfyingPipesAnimatedFactory {
    /// origin is cutting point
    pub(crate) fn chisel(&self) -> SceneTree {
        let thickness = 0.01;
        let thickness_2 = 0.04;
        scene!(
            matrix: matrix4x4!(
                translation(-1., 0., 0.)
            );
            +cube!(
                matrix: matrix4x4!(
                    scale(1., thickness, thickness_2)
                );
                pattern: Pattern::Solid(color!(0.8, 0.8, 0.4));
            ) + cylinder!(
                matrix: matrix4x4!(
                    translation(-0.5, 0., 0.)
                    scale(4., 0.2, 0.2)
                    translation(-1., 0., 0.)
                    rotation_z(degrees!(90))
                );
                pattern: Pattern::Solid(color!(0.8, 0.8, 0.4));
            ) + sphere!(
                matrix: matrix4x4!(
                    translation(-0.5, 0., 0.)
                    scale_all(0.2)
                );
                pattern: Pattern::Solid(color!(0.8, 0.8, 0.4));
            );
        )
    }
}
impl SatisfyingPipesAnimatedFactory {
    fn new(mode: Mode) -> Self {
        let pawn = scene!(
            matrix: matrix4x4!(
                scale_all(0.8)
            );
            +obj!(path: "objs/chess/pawn.obj";);
        );
        Self {
            mode,
            pipe_length: 1.,
            floor_height: 1.,
            pawn,
        }
    }
}

fn accelerate(input: f32) -> f32 {
    input * input
}

fn decelerate(input: f32) -> f32 {
    1.0 - accelerate(1.0 - input)
}

fn accelerate_decelerate(input: f32) -> f32 {
    ((input + 1.) * PI).cos() / 2. + 0.5
}