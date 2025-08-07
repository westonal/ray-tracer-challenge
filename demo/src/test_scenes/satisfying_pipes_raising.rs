use crate::test_scenes::{AnimationFrame, AnimationSpec, TestScene};
use animation::animation_spec;
use animation::interpolation::{Accelerate, Decelerate, Interpolator};
use math::tuple::color::{BLUE, GREEN, RED, WHITE};
use math::{color, degrees, matrix4x4, point, radians, vector};
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
        Some(animation_spec!(2;seconds @25;fps))
    }

    fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
        let mut factory = SatisfyingPipesAnimatedFactory::new(Mode::Efficient);
        let pipe_grid_length = TAU / 2.;
        let gap = 0.0;
        factory.pipe_length = pipe_grid_length - gap;
        factory.floor_height = 10.0;
        let factory = factory;

        let angle_of_pipe = degrees!(10);
        let distance_of_travel = pipe_grid_length * angle_of_pipe.sin_cos().1;
        let world_scene = scene!(
            +plane!(
                matrix: matrix4x4!(
                    translation(
                        frame.loop_progress * distance_of_travel,
                        -factory.floor_height,
                        0.
                    )
                );
                pattern: Pattern::Checker(*WHITE, *BLUE, Transform::new(
                    matrix4x4!(
                        scale_all(distance_of_travel)
                        // two units per pipe
                        scale_all(0.5)
                        translation(1., 0., 0.)
                    )
                ));
            );

            +{
                let mut scene = scene!();
                    for x in -9..3 {
                        let artifical_progress = factory.prog(x, frame.loop_progress);
                        let extra_height = -2. * artifical_progress;
                        scene.add(
                            scene!(
                                matrix: matrix4x4!(
                                            translation(
                                                0.,
                                                extra_height,
                                                0.,
                                            )
                                            rotation_z(angle_of_pipe)
                                            translation(
                                                pipe_grid_length * (frame.loop_progress - 0.5 + -x as f32),
                                                0.,
                                                0.,
                                            )
                                            // TODO, should implement Neg on Angle
                                            rotation_z(degrees!(-angle_of_pipe.to_degrees()))
                                            rotation_y(degrees!(-60.0 * artifical_progress))
                                        );
                                +scene!(
                                    matrix: matrix4x4!(
                                        rotation_z(angle_of_pipe)
                                    );
                                    +factory.pipe_segment();
                                );
                                +factory.pipe_stand(factory.floor_height + extra_height);
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
            +world_scene;
        ));
        world.add_light(PointLight::new(point!(2, 20, 10), color!(1, 1, 1)));
        world
    }

    fn build_camera(&self, size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(30));
        let top_down = ViewMatrix::new_look_at(point!(0, 10, 0), point!(0, 0, 0), vector!(0, 0, 1));
        let side_on = ViewMatrix::new_look_at(point!(0, 0, 10), point!(0, 0, 0), vector!(0, 1, 0));
        let top_down_high = ViewMatrix::new_look_at(
            point!(-8 - 1, 8. + 1.2, 10),
            point!(-1, 1.2, 0),
            vector!(0, 1, 0),
        );
        camera.set_transform(top_down_high);
        // camera.set_transform(side_on);
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
    pub(crate) fn prog(&self, p: i32, progress: f32) -> f32 {
        let p32 = p as f32;
        if p == 0 {
            0.
        } else if p == 1 {
            p32 - Decelerate::interpolate(progress)
        } else if p > 1 {
            p32 - progress
        } else if p == -1 {
            (1. - Accelerate::interpolate(progress)) + p32
        } else {
            (1. - progress) + p32
        }
    }
}

impl SatisfyingPipesAnimatedFactory {
    pub(crate) fn pipe_stand(&self, height: f32) -> SceneTree {
        scene!(
            +cylinder!(matrix: matrix4x4!(
                scale(0.2, height, 0.2)
                translation(0., -1., 0.)
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

    fn pipe_segment(&self) -> SceneTree {
        let length = self.pipe_length / 2.;
        let arc = 40;
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
                    c.material.pattern = Pattern::Solid(*GREEN);
                    c
                }
              - cube!(matrix: matrix4x4!(
                                scale(1.21, 1.01, 1.21 / 2.)
                                translation(0., 0., -1.)
                              )
                     )
              - cube!(matrix: matrix4x4!(
                                rotation_y(degrees!(arc))
                                scale(1.21, 1.01, 1.21 / 2.)
                                translation(0., 0., -1.)
                              );
                    pattern: Pattern::Solid(color!(1.0, 1.0, 0.0));
                     )
               - cube!(matrix: matrix4x4!(
                                rotation_y(degrees!(-arc))
                                scale(1.21, 1.01, 1.21 / 2.)
                                translation(0., 0., -1.)
                              );
                    pattern: Pattern::Solid(color!(1.0, 1.0, 0.0));
                     )) & sphere!(
                        matrix: matrix4x4!(
                            scale(length, 1., length,)
                            translation(0., 0., 0.5)
                        );
                        pattern: Pattern::Solid(color!(1.0, 1.0, 0.0));
                    );
            // +cylinder!(matrix: matrix4x4!(
            //     scale(length, 1., length,)
            //     // Move down to the bed level
            //     // translation(0., 0., 0.5)
            //     rotation_x(degrees!(90))
            // ));
        )
    }
}
