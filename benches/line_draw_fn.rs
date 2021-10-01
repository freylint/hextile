use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use image::{ImageBuffer, Rgba};

use hextile::api::draw_line;
use hextile::prelude::{Line, Point};

fn gen_bench(c: &mut Criterion) {
    c.bench_function("gen draw fn NODRAW", |b| {
        b.iter(|| {
            // Create buffer
            let mut buf = ImageBuffer::new(0, 0);

            // Exec on empty image buffer
            draw_line(
                &mut buf,
                Line::default(),
                &Rgba {
                    0: [255, 255, 255, 255],
                },
            )
        })
    });
}

fn draw_line_bench(c: &mut Criterion) {
    const SIZES: [u32; 5] = [8, 64, 512, 1024, 2048];
    const NAMES: [&str; 5] = [
        "draw_line 8x8",
        "draw_line 64x64",
        "draw_line 512x512",
        "draw_line 1024x1024",
        "draw_line 2048x2048",
    ];
    const WHITE: Rgba<u8> = Rgba {
        0: [255u8, 255u8, 255u8, 255u8],
    };

    let mut group = c.benchmark_group("line draw fns");

    for i in 0..SIZES.len() {
        let name = NAMES[i];
        let mut buf = ImageBuffer::new(SIZES[i], SIZES[i]);
        let size = SIZES[i];
        group.bench_function(name, |b| {
            b.iter(|| {
                draw_line(
                    &mut buf,
                    Line::new(Point::new(0u32, 0u32), Point::new(size, size)),
                    &WHITE,
                )
                    .unwrap()
            })
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default()
        //.sample_size(500)
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(10));
    targets = gen_bench, draw_line_bench
}
criterion_main!(benches);
