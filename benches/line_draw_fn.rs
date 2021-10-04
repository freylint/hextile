use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::{ImageBuffer, Rgba};

use hextile::prelude::{Line, Point};

fn gen_bench(c: &mut Criterion) {
    c.bench_function("gen draw fn NODRAW", |b| {
        b.iter(|| {
            // Create buffer
            let mut buf = ImageBuffer::new(0, 0);

            // Exec on empty image buffer
            Line::default().draw_over_buf(
                &mut buf,
                &Rgba {
                    0: [255, 255, 255, 255],
                },
            )
        })
    });
}

fn draw_line_bench(c: &mut Criterion) {
    const SIZES: [u32; 5] = [8, 64, 512, 1024, 2048];
    const NAMES: [&str; 5] = ["8x8", "64x64", "512x512", "1024x1024", "2048x2048"];
    const WHITE: Rgba<u8> = Rgba {
        0: [255u8, 255u8, 255u8, 255u8],
    };

    let mut group = c.benchmark_group("line draw");
    group.significance_level(0.1);

    for i in 0..SIZES.len() {
        let name = NAMES[i];
        let mut buf = ImageBuffer::new(SIZES[i], SIZES[i]);
        let size = SIZES[i];
        let num_iters = (0..(2048 / size)).len();

        let l = black_box(Line::new(Point::default(), Point::new(size, size)));
        group.bench_function(name, |b| {
            b.iter(|| {
                for _ in 0..num_iters {
                    l.draw_over_buf(&mut buf, &WHITE);
                }
            })
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3));
    targets = gen_bench, draw_line_bench
}
criterion_main!(benches);
