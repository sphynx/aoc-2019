use aoc_2019::*;
use criterion::*;

fn bench_day9(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 9, part 2");
    group.bench_function("Main impl", move |b| b.iter(|| day9::solve_part2()));
    group.finish();
}

fn config() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group!{name = benches; config = config(); targets = bench_day9}
criterion_main!(benches);
