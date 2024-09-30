use benchmark::{sort_algo_1, sort_algo_2};
use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers: Vec<i32> = vec![
        1, 2, 3, 6, 4, 3, 12, 45, 345, 23, 6, 23, 2, 21, 2, 5, 6, 6, 2, 2, 1, 123, 6, 7, 4323, 43,
        3, 2, 566, 7, 5, 3, 3, 355, 3, 2, 1,
    ];

    c.bench_function("Sorting Algorithm", |b| {
        b.iter(|| sort_algo_2(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);
