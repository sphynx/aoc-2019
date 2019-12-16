use aoc_2019::*;
use criterion::*;
use indoc::indoc;

fn bench_day12(c: &mut Criterion) {

    let input = indoc!(
        "<x=-8, y=-10, z=0>
         <x=5, y=5, z=10>
         <x=2, y=-7, z=3>
         <x=9, y=-8, z=-3>"
    );

    let mut system = day12::System::from_str(input);

    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("Gravitation");
    group.plot_config(plot_config);

    for size in [1, 10, 100, 1_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| system.run(size));
        });
    }

    group.finish();
}

fn config() -> Criterion {
    Criterion::default()
}

criterion_group! {name = benches; config = config(); targets = bench_day12}
criterion_main!(benches);
