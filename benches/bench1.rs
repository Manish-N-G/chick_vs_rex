use criterion::{Criterion, criterion_group, criterion_main};
use criterion::{BenchmarkId};

fn bench_sum(crit: &mut Criterion) {

    let x = (0..1_000_000).collect::<Vec<u128>>();
    crit.bench_function("Normal Iter", |b| {
        b.iter(|| sum_of_sq(&x) )
    });
    crit.bench_function("Rayon Iter", |b| {
        b.iter(|| sum_of_sq_rayon(&x) )
    });
}

criterion_group!(sum_fn, bench_sum);
criterion_main!(sum_fn);




fn sum_of_sq( input: &[u128] ) -> u128 {
    input
        .iter()
        .map( |val| val*val )
        .sum()
}


use rayon::prelude::*;
// Parallel processing for Iterators
fn sum_of_sq_rayon( input: &[u128] ) -> u128 {
    input
        .par_iter()
        .map( |val| val*val )
        .sum()
}
