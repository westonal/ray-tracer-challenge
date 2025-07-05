use criterion::{Criterion, criterion_group, criterion_main};
use demo_lib::test_scenes::RenderTestScene;
use demo_lib::test_scenes::cube_of_spheres::CubeOfSpheres;
use ray_tracer::canvas::Size;
use std::hint::black_box;

fn cube_of_spheres_benchmark(c: &mut Criterion) {
    c.bench_function("CubeOfSpheres", |b| {
        b.iter(|| black_box(CubeOfSpheres::render_scene_to(Size::new(100, 100), None)))
    });
}

criterion_group!(benches, cube_of_spheres_benchmark,);
criterion_main!(benches);
