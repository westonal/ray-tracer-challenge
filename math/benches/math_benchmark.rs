use criterion::{Criterion, criterion_group, criterion_main};
use math::matrix::matrix_4x4::Matrix4x4;
use math::radians;
use std::f32::consts::PI;
use std::hint::black_box;

fn inversion_benchmark(c: &mut Criterion) {
    let a = Matrix4x4::new([
        [-5., 2., 6., -8.],
        [1., -5., 1., 8.],
        [7., 7., -6., -7.],
        [1., -3., 7., 4.],
    ]);
    c.bench_function("inverse", |b| b.iter(|| black_box(a.invert())));
}

fn multiply_benchmark(c: &mut Criterion) {
    let m1 = Matrix4x4::new([
        [-5., 2., 6., -8.],
        [1., -5., 1., 8.],
        [7., 7., -6., -7.],
        [1., -3., 7., 4.],
    ]);
    let m2 = Matrix4x4::new([
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 8., 7., 6.],
        [5., 4., 3., 2.],
    ]);
    c.bench_function("multiply", |b| b.iter(|| black_box(m1 * m2)));
}

fn rotate_x_benchmark(c: &mut Criterion) {
    c.bench_function("rotate x", |b| {
        b.iter(|| black_box(Matrix4x4::rotation_x(radians!(PI / 5.2))))
    });
}

criterion_group!(
    benches,
    inversion_benchmark,
    multiply_benchmark,
    rotate_x_benchmark,
);
criterion_main!(benches);
