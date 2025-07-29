use crate::obj;
use crate::test_scenes::{AnimationFrame, AnimationSpec, TestScene};
use animation::animation_spec;
use math::tuple::color::{BLUE, RED, WHITE};
use math::{color, degrees, matrix4x4, point, vector};
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
        //None
        Some(animation_spec!(4;seconds @10;fps))
    }

    fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
        let pipe_length = 3.0;
        let gap = 0.0;
        let pipe_grid_length = pipe_length + gap;
        let grid_size = (3, 1);
        let floor_height = 2.;
        let world_scene = scene!(
            +plane!(
                matrix: matrix4x4!(
                    translation(0., -floor_height, 0.)
                );
                pattern: Pattern::Checker(*WHITE, *BLUE, Transform::identity());
            );

            +{
                let mut scene = scene!();
                for x in -(grid_size.0/2)..(grid_size.0/2 + 1) {
                    for y in -(grid_size.1/2)..(grid_size.1/2 + 1) {
                        let rot = if (x + y) % 2 == 0 {
                            degrees!(180.0 * frame.loop_progress)
                        }else {
                            degrees!(0.)//360.0 * frame.loop_progress + 90.)
                        };
                        scene.add(
                            scene!(
                                matrix: matrix4x4!(
                                            translation(
                                                pipe_grid_length * 2.0 * x as f32,
                                                0.,
                                                pipe_grid_length * 2.0 * y as f32
                                            )
                                            rotation_y(rot)
                                        );
                                +Self::pipe_segment(pipe_length);
                                +cylinder!(matrix: matrix4x4!(
                                    translation(0., -floor_height/2., 0.)
                                    scale(0.2, 1., 0.2)
                                ));
                            )
                        );
                    }
                }
                scene
            };
            +sphere!(
                matrix: matrix4x4!(translation(0., 1.2, 0.));
                pattern: Pattern::Checker(*WHITE, *RED, Transform::identity());
            );
        );

        let mut world = World::default();
        world.add(
            scene!(
                matrix: matrix4x4!(translation(pipe_length,0.,0.));
            +world_scene;
            ));
        world.add_light(PointLight::new(point!(2, 20, 10), color!(1, 1, 1)));
        world
    }

    fn build_camera(&self, size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(35));
        camera.set_transform(
            ViewMatrix::new_look_at(point!(0, 10, 0), point!(0, 0, 0), vector!(0, 0, 1)).into(),
        );
        camera
    }
}

impl SatisfyingPipesAnimated {
    fn pipe_segment(length: f32) -> SceneTree {
        scene!(
            matrix: matrix4x4!(
                rotation_y(degrees!(90))
                translation(0., 1.2, 0.)
                rotation_x(degrees!(90))
                scale(1., length, 1.,)
            );
            bounding_volume: cube!(matrix: matrix4x4!(scale(1.21, 1.01, 1.21)));
            +(cylinder!(matrix: matrix4x4!(scale(1.2, 1., 1.2)))
              - cylinder!(matrix: matrix4x4!(scale(1., 1.01, 1.)))
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
