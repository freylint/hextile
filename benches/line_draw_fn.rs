use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use image::Rgba;

use hextile::api::line_draw_fn;
use hextile::prelude::{Line, Point};

fn gen_bench(c: &mut Criterion) {
    c.bench_function("gen draw fn NODRAW", |b| {
        b.iter(|| {
            line_draw_fn(
                Line::default(),
                [1; 2],
                &Rgba {
                    0: [255, 255, 255, 255],
                },
            )
        })
    });
}

fn run_bench_8x8(c: &mut Criterion) {
    const SIZE: u32 = 8u32;
    c.bench_function("gen draw fn 8x8", |b| {
        b.iter(|| {
            line_draw_fn(
                Line::new(Point::new(0u32, 0u32), Point::new(SIZE, SIZE)),
                [SIZE; 2],
                &Rgba {
                    0: [255, 255, 255, 255],
                },
            )()
            .unwrap()
        })
    });
}

fn run_bench_64x64(c: &mut Criterion) {
    const SIZE: u32 = 64u32;
    c.bench_function("gen draw fn 64x64", |b| {
        b.iter(|| {
            line_draw_fn(
                Line::new(Point::new(0u32, 0u32), Point::new(SIZE, SIZE)),
                [SIZE; 2],
                &Rgba {
                    0: [255, 255, 255, 255],
                },
            )()
            .unwrap()
        })
    });
}

fn run_bench_512x512(c: &mut Criterion) {
    const SIZE: u32 = 512u32;
    c.bench_function("gen draw fn 512x512", |b| {
        b.iter(|| {
            line_draw_fn(
                Line::new(Point::new(0u32, 0u32), Point::new(SIZE, SIZE)),
                [512; 2],
                &Rgba {
                    0: [255, 255, 255, 255],
                },
            )()
            .unwrap()
        })
    });
}

fn run_bench_1024x1024(c: &mut Criterion) {
    const SIZE: u32 = 1024u32;
    c.bench_function("gen draw fn 1024x1024", |b| {
        b.iter(|| {
            line_draw_fn(
                Line::new(Point::new(0u32, 0u32), Point::new(SIZE, SIZE)),
                [1024; 2],
                &Rgba {
                    0: [255, 255, 255, 255],
                },
            )()
            .unwrap()
        })
    });
}

fn run_bench_2048x2048(c: &mut Criterion) {
    const SIZE: u32 = 2048u32;
    c.bench_function("gen draw fn 2048x2048", |b| {
        b.iter(|| {
            line_draw_fn(
                Line::new(Point::new(0u32, 0u32), Point::new(SIZE, SIZE)),
                [SIZE; 2],
                &Rgba {
                    0: [255, 255, 255, 255],
                },
            )()
            .unwrap()
        })
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default()
        //.sample_size(500)
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(10));
    targets = gen_bench, run_bench_8x8, run_bench_64x64, run_bench_512x512, run_bench_1024x1024, run_bench_2048x2048
}
criterion_main!(benches);
