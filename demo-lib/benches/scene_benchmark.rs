use criterion::{Criterion, criterion_group, criterion_main};
use demo_lib::test_scenes::RenderTestScene;
use demo_lib::test_scenes::cube_of_spheres::CubeOfSpheres;
use ray_tracer::canvas::Size;
use std::hint::black_box;
use std::time::Duration;

fn scenes_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Scenes");
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("CubeOfSpheres", |b| {
        b.iter(|| black_box(CubeOfSpheres::render_scene_to(Size::new(100, 100), None)))
    });
    group.finish()
}

criterion_group!(benches, scenes_benchmark,);
criterion_main!(benches);
