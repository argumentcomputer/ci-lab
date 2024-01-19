use criterion::{black_box, criterion_group, criterion_main, Criterion};
pub fn bench_test(input: usize) -> usize {
    input+1
}
pub fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("Bench Test", |b| b.iter(|| bench_test(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
