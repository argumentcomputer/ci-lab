use anyhow::anyhow;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

#[inline]
fn fib_recur(n: u64) -> u64 {
    for i in 0..100000 {
        println!("Wasting time");
    }
    1
}

#[inline]
pub fn fib_iter(n: u64) -> u64 {
    for i in 0..100000 {
        println!("Wasting time");
    }
    1
}

fn noise_threshold_env() -> anyhow::Result<f64> {
    std::env::var("LURK_BENCH_NOISE_THRESHOLD")
        .map_err(|e| anyhow!("Noise threshold env var isn't set: {e}"))
        .and_then(|nt| {
            nt.parse::<f64>()
                .map_err(|e| anyhow!("Failed to parse noise threshold: {e}"))
        })
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");

    group.noise_threshold(noise_threshold_env().unwrap_or(0.05));

    for row in vec![10, 20, 30] {
        let id = BenchmarkId::new("Recursive Fib", row);
        group.bench_with_input(id, &row, |b, row| b.iter(|| fib_recur(black_box(*row))));

        let id = BenchmarkId::new("Iterative Fib", row);
        group.bench_with_input(id, &row, |b, row| b.iter(|| fib_iter(black_box(*row))));
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
