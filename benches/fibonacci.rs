use anyhow::anyhow;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

#[inline]
fn fib_recur(n: u64) -> u64 {
    //match n {
    //    0 => 1,
    //    1 => 1,
    //    n => fib_recur(n - 1) + fib_recur(n - 2),
    //}
    std::thread::sleep(std::time::Duration::from_millis(1));
    1
}

#[inline]
pub fn fib_iter(n: u64) -> u64 {
    //if n == 1 {
    //        1
    //    } else {
    //        let mut sum = 0;
    //        let mut last = 0;
    //        let mut curr = 1;

    //        for _ in 1..n {
    //            sum = last + curr;
    //            last = curr;
    //            curr = sum;
    //        }

    //        sum
    //    }
    std::thread::sleep(std::time::Duration::from_millis(1));
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
