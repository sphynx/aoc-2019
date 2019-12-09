use aoc_2019::*;
use criterion::*;

fn bench_day3(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 3, part 1");
    group.bench_function("2D array", move |b| b.iter(|| day3::solve_part1()));
    group.bench_function("HashSet", move |b| b.iter(|| day3::solve_part1_with_hashset()));
    group.finish();
}

fn config() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group!{name = benches; config = config(); targets = bench_day3}
criterion_main!(benches);
