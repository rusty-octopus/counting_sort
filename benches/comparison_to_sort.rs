use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use std::time::Duration;

use oorandom::Rand32;

use counting_sort::CountingSort;

use count_sort::sort_u8;

fn create_vector(number_of_elements:usize) -> Vec<u8> {
    let mut vector = Vec::with_capacity(number_of_elements);

    let mut rng = Rand32::new(17);

    for _ in 0..number_of_elements {
        vector.push(rng.rand_range(0..256) as u8);
    }

    vector
}

fn count_sort_vector_u8_65k(c: &mut Criterion) {
    let vector = create_vector(65536);
    c.bench_function("count sort vector<u8> 65536", |b| b.iter(|| black_box(vector.iter().cnt_sort().unwrap())));
}

fn count_sort_vector_u8_01k(c: &mut Criterion) {
    let vector = create_vector(1000);
    c.bench_function("count sort vector<u8> 1000", |b| b.iter(|| black_box(vector.iter().cnt_sort().unwrap())));
}

fn vector_sort_u8_01k(c: &mut Criterion) {
    let vector = create_vector(1000);
    c.bench_function("vector.sort <u8> 1000", move |b| 
        b.iter_batched(|| vector.clone(), |mut vector| black_box(vector.sort()), BatchSize::LargeInput)
    );
}

fn vector_sort_u8_65k(c: &mut Criterion) {
    let vector = create_vector(65536);
    c.bench_function("vector.sort <u8> 65536", move |b| 
        b.iter_batched(|| vector.clone(), |mut vector| black_box(vector.sort()), BatchSize::LargeInput)
    );
}

fn count_sort_u8_65k(c: &mut Criterion) {
    let vector = create_vector(65536);
    c.bench_function("count_sort <u8> 65536", move |b| 
        b.iter_batched_ref(|| vector.clone(), |mut v| black_box(sort_u8(& mut v)), BatchSize::LargeInput)
    );
}

//pub fn vector_sort_u8_01k_unbatched(c: &mut Criterion) {
//    let mut vector = create_vector(1000);
//    c.bench_function("vector.sort <u8> 1000 unbatched", |b| b.iter(|| black_box(vector.sort())));
//}
//
//pub fn vector_sort_u8_65k_unbatched(c: &mut Criterion) {
//    let mut vector = create_vector(65536);
//    c.bench_function("vector.sort <u8> 65536 unbatched", |b| b.iter(|| black_box(vector.sort())));
//}

fn compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sort");
    let vector = create_vector(65536);
    group.bench_function("Compare: count sort vector<u8> 65536", |b| b.iter(|| black_box(vector.iter().cnt_sort().unwrap())));
    group.bench_function("Compare: vector.sort <u8> 65536", |b| b.iter_batched(|| vector.clone(), |mut v| black_box(v.sort()), BatchSize::LargeInput));    
    group.bench_function("Compare: count_sort <u8> 65536", |b| b.iter_batched_ref(|| vector.clone(), |mut v| black_box(sort_u8(& mut v)), BatchSize::LargeInput));
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(60));
    targets = 
    count_sort_vector_u8_01k,
    vector_sort_u8_01k,
    count_sort_vector_u8_65k,
    vector_sort_u8_65k,
    count_sort_u8_65k,
    compare);
criterion_main!(benches);