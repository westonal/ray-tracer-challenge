use crate::test_scenes::{AnimationFrame, AnimationSpec, TestScene};
use animation::animation_spec;
use math::tuple::color::{BLUE, GREEN, RED, WHITE};
use math::{Angle, color, degrees, matrix4x4, point, vector};
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
use std::time::Duration;

pub struct SatisfyingPipesAnimated;

impl TestScene for SatisfyingPipesAnimated {
    fn name(&self) -> &'static str {
        "satisfying-pipes"
    }

    fn animation_spec(&self) -> Option<AnimationSpec> {
        Some(animation_spec!(4;seconds @25;fps))
    }

    fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
        let mut factory = SatisfyingPipesAnimatedFactory::new(Mode::Efficient);
        factory.pipe_length = 6.0;
        factory.floor_height = 2.0;
        let factory = factory;
        let gap = 0.05;
        let pipe_grid_length = factory.pipe_length + gap;
        let grid_size = (5, 3);

        // time stuff
        let rotations_per_loop = 2.;
        let rotational_progress = frame.loop_progress * rotations_per_loop;
        let static_for = 1.0 / (4.0 * rotations_per_loop);
        let angle1 = factory.progress_to_angle(rotational_progress + 0.25);
        let angle2 = factory.progress_to_angle(rotational_progress + 0.5);

        let world_scene = scene!(
            +plane!(
                matrix: matrix4x4!(
                    translation(0., -factory.floor_height, 0.)
                );
                pattern: Pattern::Checker(*WHITE, *BLUE, Transform::identity());
            );

            +{
                let mut scene = scene!();
                for x in -(grid_size.0/2)..(grid_size.0/2 + 1) {
                    for y in -(grid_size.1/2)..(grid_size.1/2 + 1) {
                        let (rot, pipe_motion) = if (x + y) % 2 == 0 {
                            angle1
                        } else {
                            angle2
                        };
                        scene.add(
                            scene!(
                                matrix: matrix4x4!(
                                            translation(
                                                pipe_grid_length * x as f32,
                                                0.,
                                                pipe_grid_length * y as f32
                                            )
                                            rotation_y(rot)
                                        );
                                +factory.pipe_segment(pipe_motion);
                                +factory.pipe_stand();
                            )
                        );
                    }
                }
                scene
            };
            +sphere!(
                matrix: matrix4x4!(translation(0., 1.2,
                    pipe_grid_length*3./2. - pipe_grid_length * frame.loop_progress / static_for
                ));
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
            ViewMatrix::new_look_at(point!(0, 30, 0), point!(0, 0, 0), vector!(0, 0, 1));
        camera.set_transform(top_down_high);
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
            (
                degrees!(90. + (rotate_stage - 3.) * 90.),
                PipeAngle::MovingToX,
            )
        }
    }
}

impl SatisfyingPipesAnimatedFactory {
    pub(crate) fn pipe_stand(&self) -> SceneTree {
        if self.mode == Mode::Efficient {
            return scene!();
        }
        scene!(
            +cylinder!(matrix: matrix4x4!(
                translation(0., -self.floor_height/2., 0.)
                scale(0.2, 1., 0.2)
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
            bounding-volume: cube!(matrix: matrix4x4!(scale(1.21, 1.01, 1.21)));
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
                     )) & cylinder!(matrix: matrix4x4!(
                scale(length, 1., length,)
                // Move down to the bed level
                // translation(0., 0., 0.5)
                rotation_x(degrees!(90))
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
