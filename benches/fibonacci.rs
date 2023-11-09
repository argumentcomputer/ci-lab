use anyhow::anyhow;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

#[inline]
fn fib_recur(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fib_recur(n - 1) + fib_recur(n - 2),
    }
}

#[inline]
pub fn fib_iter(n: u64) -> u64 {
    if n == 1 {
        1
    } else {
        let mut sum = 0;
        let mut last = 0;
        let mut curr = 1;

        for _ in 1..n {
            sum = last + curr;
            last = curr;
            curr = sum;
        }

        sum
    }
}

#[derive(Clone, Debug, Copy)]
struct ProveParams {
    fib_n: u64,
    _reduction_count: u64,
    date: &'static str,
    sha: &'static str,
}

impl ProveParams {
    fn name_params(&self) -> (String, String) {
        let output_type = bench_parameters_env().unwrap_or("stdout".into());
        match output_type.as_ref() {
            "pr-comment" => ("fib".into(), format!("num-{}", self.fib_n)),
            "commit-comment" => (
                format!("fib-ref={}", env!("VERGEN_GIT_SHA")),
                format!("num-{}", self.fib_n),
            ),
            // TODO: refine "gh-pages",
            _ => (
                "fib".into(),
                format!("num-{}-{}-{}", self.fib_n, self.sha, self.date),
            ),
        }
    }
}

fn bench_parameters_env() -> anyhow::Result<String> {
    std::env::var("LURK_BENCH_OUTPUT")
        .map_err(|e| anyhow!("Noise threshold env var isn't set: {e}"))
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
    let batch_sizes = [100, 200];

    for rc in batch_sizes.iter() {
        let mut group = c.benchmark_group(format!("Fibonacci-rc={}", rc));

        group.noise_threshold(noise_threshold_env().unwrap_or(0.05));
        let nums: Vec<u64> = vec![10, 20];

        for num in nums {
            let prove_params = ProveParams {
                fib_n: num,
                _reduction_count: *rc,
                date: env!("VERGEN_GIT_COMMIT_DATE"),
                sha: env!("VERGEN_GIT_SHA"),
            };
            let (name, params) = prove_params.name_params();
            let id = BenchmarkId::new(format!("Recursive-{}", name), &params);
            group.bench_with_input(id, &num, |b, row| b.iter(|| fib_recur(black_box(*row))));

            let id = BenchmarkId::new(format!("Iterative-{}", name), params);
            group.bench_with_input(id, &num, |b, row| b.iter(|| fib_iter(black_box(*row))));
        }

        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
