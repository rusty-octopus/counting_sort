use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::time::Duration;

use oorandom::Rand32;

use counting_sort::CountingSort;

use count_sort::{sort_u16, sort_u8};

use core::ops::Range;

use core::convert::TryFrom;

fn create_vector_t<T: TryFrom<u32>>(number_of_elements: usize, range: Range<u32>) -> Vec<T> {
    let mut vector: Vec<T> = Vec::with_capacity(number_of_elements);
    let mut rng = Rand32::new(7648730752358173238);
    for _ in 0..number_of_elements {
        let random_u32 = rng.rand_range(range.clone());
        let random_value_result = T::try_from(random_u32);
        match random_value_result {
            Ok(v) => vector.push(v),
            Err(e) => println!("Error occurred converting {}", random_u32),
        };
    }
    vector
}

fn compare_u8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sort u8");
    let mut number_of_elements = 10000;
    let range_min: u32 = 0;
    let range_max: u32 = 256;
    while number_of_elements <= 100000 {
        let vector = create_vector_t::<u8>(number_of_elements, range_min..range_max);
        group.bench_function(BenchmarkId::new("cnt_sort", number_of_elements), |b| {
            b.iter(|| black_box(vector.iter().cnt_sort().unwrap()))
        });
        group.bench_function(
            BenchmarkId::new("cnt_sort_min_max", number_of_elements),
            |b| {
                b.iter(|| {
                    black_box(
                        vector
                            .iter()
                            .cnt_sort_min_max(&(range_min as u8), &((range_max - 1) as u8))
                            .unwrap(),
                    )
                })
            },
        );
        group.bench_function(BenchmarkId::new("vector.sort", number_of_elements), |b| {
            b.iter_batched(
                || vector.clone(),
                |mut v| black_box(v.sort()),
                BatchSize::LargeInput,
            )
        });
        group.bench_function(BenchmarkId::new("sort_u8", number_of_elements), |b| {
            b.iter_batched_ref(
                || vector.clone(),
                |mut v| black_box(sort_u8(&mut v)),
                BatchSize::LargeInput,
            )
        });
        number_of_elements += 10000;
    }
    group.finish();
}

fn compare_u16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sort u16");
    let mut number_of_elements = 10000;
    let range_min: u32 = 512;
    let range_max: u32 = 1024;
    while number_of_elements <= 100000 {
        let vector = create_vector_t::<u16>(number_of_elements, range_min..range_max);
        group.bench_function(BenchmarkId::new("cnt_sort", number_of_elements), |b| {
            b.iter(|| black_box(vector.iter().cnt_sort().unwrap()))
        });
        group.bench_function(
            BenchmarkId::new("cnt_sort_min_max", number_of_elements),
            |b| {
                b.iter(|| {
                    black_box(
                        vector
                            .iter()
                            .cnt_sort_min_max(&(range_min as u16), &(range_max as u16))
                            .unwrap(),
                    )
                })
            },
        );
        group.bench_function(BenchmarkId::new("vector.sort", number_of_elements), |b| {
            b.iter_batched(
                || vector.clone(),
                |mut v| black_box(v.sort()),
                BatchSize::LargeInput,
            )
        });
        group.bench_function(BenchmarkId::new("sort_u16", number_of_elements), |b| {
            b.iter_batched_ref(
                || vector.clone(),
                |mut v| black_box(sort_u16(&mut v)),
                BatchSize::LargeInput,
            )
        });
        number_of_elements += 10000;
    }
    group.finish();
}

criterion_group!(
        name = benches;
        config = Criterion::default().measurement_time(Duration::from_secs(10));
        targets = compare_u8, compare_u16
    );
criterion_main!(benches);
