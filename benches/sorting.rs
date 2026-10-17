use criterion::{Criterion, criterion_group, criterion_main};
use dsa::algorithms::sorting::bubble_sort;

fn bench(c: &mut Criterion) {
    let data: Vec<i32> = (0..1000).rev().collect();
    c.bench_function("bubble 1000", |b| {
        b.iter(|| {
            let mut v = data.clone();
            bubble_sort(&mut v);
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
