use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize, BenchmarkId};
use std::time::Duration;

use oorandom::Rand32;

use counting_sort::CountingSort;

use count_sort::{sort_u8, sort_u16};

use core::ops::Range;

use core::convert::TryFrom;

fn create_vector(number_of_elements:usize) -> Vec<u8> {
    let mut vector = Vec::with_capacity(number_of_elements);

    let mut rng = Rand32::new(17);

    for _ in 0..number_of_elements {
        vector.push(rng.rand_range(0..256) as u8);
    }

    vector
}

fn create_vector_t<T:TryFrom<u32>>(number_of_elements:usize, range: Range<u32>) -> Vec<T> {
    let mut vector:Vec<T> = Vec::with_capacity(number_of_elements);
    let mut rng = Rand32::new(7648730752358173238);
    for _ in 0..number_of_elements {
        let random_u32 = rng.rand_range(range.clone());
        let random_value_result = T::try_from(random_u32);
        match random_value_result {
            Ok(v) => vector.push(v),
            Err(e) => println!("Error occurred converting {}", random_u32)
        };
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

fn compare_u8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sort u8");
    let mut number_of_elements = 10000;
    while number_of_elements <= 100000 {
        let vector = create_vector_t::<u8>(number_of_elements, 0..256);
        group.bench_function(BenchmarkId::new("cnt_sort", number_of_elements), |b| b.iter(|| black_box(vector.iter().cnt_sort().unwrap())));
        group.bench_function(BenchmarkId::new("vector.sort", number_of_elements), |b| b.iter_batched(|| vector.clone(), |mut v| black_box(v.sort()), BatchSize::LargeInput));    
        group.bench_function(BenchmarkId::new("sort_u8", number_of_elements), |b| b.iter_batched_ref(|| vector.clone(), |mut v| black_box(sort_u8(& mut v)), BatchSize::LargeInput));    
        number_of_elements += 10000;
    }
    group.finish();
}

fn compare_u16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sort u16");
    let mut number_of_elements = 10000;
    while number_of_elements <= 100000 {
        let vector = create_vector_t::<u16>(number_of_elements, 0..512);
        group.bench_function(BenchmarkId::new("cnt_sort", number_of_elements), |b| b.iter(|| black_box(vector.iter().cnt_sort().unwrap())));
        group.bench_function(BenchmarkId::new("vector.sort", number_of_elements), |b| b.iter_batched(|| vector.clone(), |mut v| black_box(v.sort()), BatchSize::LargeInput));    
        group.bench_function(BenchmarkId::new("sort_u16", number_of_elements), |b| b.iter_batched_ref(|| vector.clone(), |mut v| black_box(sort_u16(& mut v)), BatchSize::LargeInput));    
        number_of_elements += 10000;
    }
    group.finish();
}

fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_slow(n-1) + fibonacci_slow(n-2),
    }
}

fn fibonacci_fast(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}


fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("Recursive", i), i, 
            |b, i| b.iter(|| fibonacci_slow(*i)));
        group.bench_with_input(BenchmarkId::new("Iterative", i), i, 
            |b, i| b.iter(|| fibonacci_fast(*i)));
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = 
    compare_u8,
    compare_u16
    );
criterion_main!(benches);

