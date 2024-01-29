use anyhow::anyhow;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

//#[inline]
//fn fib_recur(n: u64) -> u64 {
//    match n {
//        0 => 1,
//        1 => 1,
//        n => fib_recur(n - 1) + fib_recur(n - 2),
//    }
//}

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
    //std::thread::sleep(std::time::Duration::from_millis(1));
    //1
}

#[derive(Clone, Debug)]
struct ProveParams {
    fib_n: u64,
    date: String,
    sha: String,
}

impl ProveParams {
    fn new(fib_n: u64) -> Self {
        let date = env!("VERGEN_GIT_COMMIT_DATE").to_owned();
        let mut sha = env!("VERGEN_GIT_SHA").to_owned();
        sha.truncate(7);
        Self { fib_n, date, sha }
    }
    fn group_name_params(&self) -> (String, String) {
        let output_type = bench_parameters_env().unwrap_or("stdout".into());

        let mut short_sha = self.sha.to_owned();
        short_sha.truncate(7);
        match output_type.as_ref() {
            "pr-comment" => ("fib".into(), format!("num-{}", self.fib_n)),
            "commit-comment" => (
                format!("fib-ref={}", self.sha),
                format!("num-{}", self.fib_n),
            ),
            "gh-pages" => (
                format!("{}-{}", self.sha, self.date),
                format!("num-{}", self.fib_n),
            ),
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

fn rc_env() -> anyhow::Result<Vec<usize>> {
    std::env::var("LURK_RC")
        .map_err(|e| anyhow!("Reduction count env var isn't set: {e}"))
        .and_then(|rc| {
            let vec: anyhow::Result<Vec<usize>> = rc
                .split(',')
                .map(|rc| {
                    rc.parse::<usize>()
                        .map_err(|e| anyhow!("Failed to parse RC: {e}"))
                })
                .collect();
            vec
        })
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
    let nums: Vec<u64> = vec![10, 20];
    for num in nums {
        let mut group = c.benchmark_group(format!("Fibonacci-num={}", num));
        group.noise_threshold(noise_threshold_env().unwrap_or(0.05));

        let reduction_counts = rc_env().unwrap_or_else(|_| vec![100]);
        for rc in reduction_counts.iter() {
            let prove_params = ProveParams::new(num);
            let id = BenchmarkId::new(
                format!("{}-{}", prove_params.sha, prove_params.date),
                format!("rc={}", rc),
            );
            group.bench_with_input(id, &num, |b, row| b.iter(|| fib_iter(black_box(*row))));
        }
        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
