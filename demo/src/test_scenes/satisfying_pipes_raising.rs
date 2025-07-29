use crate::obj;
use crate::test_scenes::{AnimationFrame, AnimationSpec, TestScene};
use animation::animation_spec;
use math::tuple::color::{BLUE, GREEN, RED, WHITE};
use math::{color, degrees, matrix4x4, point, radians, vector, Angle};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::scene_tree::SceneTree;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{cube, cylinder, plane, scene, sphere};
use std::default::Default;
use std::f32::consts::TAU;
use std::time::Duration;

pub struct SatisfyingPipesRaisingAnimated;

impl TestScene for SatisfyingPipesRaisingAnimated {
    fn name(&self) -> &'static str {
        "satisfying-pipes-raising"
    }

    fn animation_spec(&self) -> Option<AnimationSpec> {
        Some(animation_spec!(4;seconds @25;fps))
    }

    fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
        let mut factory = SatisfyingPipesAnimatedFactory::new(Mode::Efficient);
        let pipe_grid_length = TAU;
        let gap = 0.05;
        factory.pipe_length = pipe_grid_length - gap;
        factory.floor_height = 10.0;
        let factory = factory;

        // time stuff

        let world_scene = scene!(
            +plane!(
                matrix: matrix4x4!(
                    translation(0., -factory.floor_height, 0.)
                );
                pattern: Pattern::Checker(*WHITE, *BLUE, Transform::identity());
            );

            +{
                let mut scene = scene!();
                    for x in -2..2 {
                        scene.add(
                            scene!(
                                matrix: matrix4x4!(
                                            rotation_z(degrees!(15))
                                            translation(
                                                pipe_grid_length * (frame.loop_progress - 0.5 + -x as f32),
                                                factory.pipe_height(x, frame.loop_progress),
                                                0.,//pipe_grid_length * y as f32
                                            )
                                            rotation_z(degrees!(-15))
                                            //rotation_y(rot)
                                        );
                                +scene!(
                                    matrix: matrix4x4!(
                                        rotation_z(degrees!(15))
                                    );
                                    +factory.pipe_segment(PipeAngle::StaticZ);
                                );
                                +factory.pipe_stand();
                            )
                        );
                    }
                scene
            };
            +sphere!(
                matrix: matrix4x4!(
                    translation(0., 1.2, 0.)
                    rotation_z(radians!(pipe_grid_length * frame.loop_progress))
                );
                pattern: Pattern::Checker(*WHITE, *RED, Transform::identity());
            );
        );

        let mut world = World::default();
        world.add(scene!(
            //matrix: matrix4x4!(translation(pipe_grid_length,0.,0.));
            +world_scene;
        ));
        world.add_light(PointLight::new(point!(2, 20, 10), color!(1, 1, 1)));
        world
    }

    fn build_camera(&self, size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(35));
        let top_down = ViewMatrix::new_look_at(point!(0, 10, 0), point!(0, 0, 0), vector!(0, 0, 1));
        let top_down_high =
            ViewMatrix::new_look_at(point!(-5, 2, 10), point!(0., 1.2, 0.), vector!(0, 1, 0));
        camera.set_transform(top_down_high.into());
        camera
    }
}

#[derive(PartialEq)]
enum Mode {
    Efficient,
    Final,
}

struct SatisfyingPipesAnimatedFactory {
    mode: Mode,
    pipe_length: f32,
    floor_height: f32,
}

impl SatisfyingPipesAnimatedFactory {
    pub(crate) fn pipe_height(&self, p: i32, progress: f32) -> f32 {
        let p32 = p as f32;
        if p == 0 {
            0.
        } else if p > 0 {
          -2. * ((1. - progress) * p32 + p32 - 1.)
        } else {
          -2. * (progress * p32 + p32 + 1.)
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum PipeAngle {
    StaticX,
    MovingToZ,
    StaticZ,
    MovingToX,
}

impl SatisfyingPipesAnimatedFactory {
    pub(crate) fn progress_to_angle(&self, loop_progress: f32) -> (Angle, PipeAngle) {
        let rotate_stage = (loop_progress * 4.0) % 4.0;
        if rotate_stage < 1. {
            (degrees!(0.), PipeAngle::StaticX)
        } else if rotate_stage < 2. {
            (degrees!((rotate_stage - 1.) * 90.), PipeAngle::MovingToZ)
        } else if rotate_stage < 3. {
            (degrees!(90.), PipeAngle::StaticZ)
        } else {
            (degrees!(90. + (rotate_stage - 3.) * 90.), PipeAngle::MovingToX)
        }
    }
}

impl SatisfyingPipesAnimatedFactory {
    pub(crate) fn pipe_stand(&self) -> SceneTree {
        // if self.mode == Mode::Efficient {
        //     return scene!();
        // }
        scene!(
            +cylinder!(matrix: matrix4x4!(
                translation(0., -self.floor_height, 0.)
                scale(0.2, self.floor_height / 2., 0.2)
                translation(0., 1., 0.)
            ));
        )
    }
}

impl SatisfyingPipesAnimatedFactory {
    fn new(mode: Mode) -> Self {
        Self {
            mode,
            pipe_length: 1.,
            floor_height: 1.,
        }
    }

    fn pipe_segment(&self, pipe_motion: PipeAngle) -> SceneTree {
        let length = self.pipe_length / 2.;
        scene!(
            matrix: matrix4x4!(
                rotation_y(degrees!(90))
                translation(0., 1.2, 0.)
                rotation_x(degrees!(90))
                scale(1., length, 1.,)
            );
            bounding_volume: cube!(matrix: matrix4x4!(scale(1.21, 1.01, 1.21)));
            +(cylinder!(matrix: matrix4x4!(scale(1.2, 1., 1.2)))
              - {
                    let mut c = cylinder!(matrix: matrix4x4!(scale(1., 1.01, 1.)));
                    c.material.pattern = match pipe_motion {
                        PipeAngle::StaticX => {Pattern::Solid(*GREEN)}
                        PipeAngle::MovingToZ => {Pattern::Solid(*BLUE)}
                        PipeAngle::StaticZ => {Pattern::Solid(*GREEN)}
                        PipeAngle::MovingToX => {Pattern::Solid(*BLUE)}
                    };
                    c
                }
              - cube!(matrix: matrix4x4!(
                                scale(1.21, 1.01, 1.21 / 2.)
                                translation(0., 0., -1.)
                              )
                     )) & sphere!(matrix: matrix4x4!(
                scale(length, 1., length,)
                translation(0., 0., 0.5)
            ));
            // +cylinder!(matrix: matrix4x4!(
            //     scale(length, 1., length,)
            //     // Move down to the bed level
            //     // translation(0., 0., 0.5)
            //     rotation_x(degrees!(90))
            // ));
        )
    }
}
