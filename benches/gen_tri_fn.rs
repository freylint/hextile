use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hextile::api::gen_draw_fn;
use image::Rgba;
use std::time::Duration;


fn gen_bench(c: &mut Criterion) {
    c.bench_function("gen draw fn NODRAW", |b| b.iter(|| gen_draw_fn([0;2], [1;2], &Rgba {0: [255, 255, 255, 255]})));
}

fn run_bench_8x8(c: &mut Criterion) {
    c.bench_function("gen draw fn 8x8", |b| b.iter(|| gen_draw_fn([0;2], [8;2], &Rgba {0: [255, 255, 255, 255]})().unwrap()));
}
fn run_bench_64x64(c: &mut Criterion) {
    c.bench_function("gen draw fn 64x64", |b| b.iter(|| gen_draw_fn([0;2], [64;2], &Rgba {0: [255, 255, 255, 255]})().unwrap()));
}

fn run_bench_512x512(c: &mut Criterion) {
    c.bench_function("gen draw fn 512x512", |b| b.iter(|| gen_draw_fn([0;2], [512;2], &Rgba {0: [255, 255, 255, 255]})().unwrap()));
}

fn run_bench_1024x1024(c: &mut Criterion) {
    c.bench_function("gen draw fn 1024x1024", |b| b.iter(|| gen_draw_fn([0;2], [1024;2], &Rgba {0: [255, 255, 255, 255]})().unwrap()));
}

fn run_bench_2048x2048(c: &mut Criterion) {
    c.bench_function("gen draw fn 2048x2048", |b| b.iter(|| gen_draw_fn([0;2], [2048;2], &Rgba {0: [255, 255, 255, 255]})().unwrap()));
}
criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default()
        //.sample_size(500)
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(10));
    targets = gen_bench, run_bench_8x8, run_bench_64x64, run_bench_512x512, run_bench_1024x1024, run_bench_2048x2048
}
criterion_main!(benches);