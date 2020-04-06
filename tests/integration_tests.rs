use counting_sort::CountingSort;

use std::collections::LinkedList;

use oorandom::Rand32;

use core::ops::Range;

use core::convert::TryFrom;

#[test]
fn test_with_list() {
    let mut list = LinkedList::new();
    list.push_back(4);
    list.push_back(3);
    list.push_back(2);
    list.push_back(1);
    let sorted_vector = list.iter().cnt_sort().unwrap();
    let test_vector = vec![1, 2, 3, 4];
    assert_eq!(test_vector, sorted_vector);
}

#[test]
fn test_with_vector() {
    let mut vector = Vec::new();
    vector.push(4);
    vector.push(3);
    vector.push(2);
    vector.push(1);
    let sorted_vector = vector.iter().cnt_sort().unwrap();
    let test_vector = vec![1, 2, 3, 4];
    assert_eq!(test_vector, sorted_vector);
}

#[test]
fn test_with_slice() {
    let slice: [u8; 4] = [4, 3, 2, 1];
    let sorted_vector = slice.iter().cnt_sort().unwrap();
    let test_vector = vec![1, 2, 3, 4];
    assert_eq!(sorted_vector, test_vector);
}

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
fn create_vector_t_unsigned<T: TryFrom<u32>>(
    number_of_elements: usize,
    range: Range<u32>,
) -> Vec<T> {
    let mut vector: Vec<T> = Vec::with_capacity(number_of_elements);
    let mut rng = Rand32::new(7648730752358173238);
    for _ in 0..number_of_elements {
        let random_u32 = rng.rand_range(range.clone());
        let random_value_result = T::try_from(random_u32);
        match random_value_result {
            Ok(v) => vector.push(v),
            Err(_) => println!("Error occurred converting {}", random_u32),
        };
    }
    vector
}

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
fn convert_to_signed(value: u32) -> i32 {
    let max_i32 = u32::max_value() / 2;
    if value > u32::max_value() / 2 {
        i32::min_value() + (value - max_i32) as i32
    } else {
        value as i32
    }
}

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
fn create_vector_t_signed<T: TryFrom<i32>>(number_of_elements: usize, range: Range<u32>) -> Vec<T> {
    let mut vector: Vec<T> = Vec::with_capacity(number_of_elements);
    let mut rng = Rand32::new(7648730752358173238);
    for _ in 0..number_of_elements {
        let random_u32 = rng.rand_range(range.clone());
        let random_value_result = T::try_from(convert_to_signed(random_u32));
        match random_value_result {
            Ok(v) => vector.push(v),
            Err(_) => println!("Error occurred converting {}", random_u32),
        };
    }
    vector
}

#[test]
fn test_with_vector_u8_10k() {
    let number_of_elements = 10000;
    let range_min: u32 = 0;
    let range_max: u32 = 256;
    let mut vector = create_vector_t_unsigned::<u8>(number_of_elements, range_min..range_max);
    let result = vector.iter().cnt_sort_min_max(&(range_min as u8), &255);
    assert!(result.is_ok());
    vector.sort();
    assert_eq!(vector, result.unwrap());
}

#[ignore]
#[test]
fn test_with_vector_i32_10k() {
    let number_of_elements = 10000;
    let range_min: u32 = 0;
    let range_max: u32 = 0xFFFFF;
    let mut vector = create_vector_t_signed::<i32>(number_of_elements, range_min..range_max);
    let neg_vector = vector
        .iter()
        .filter(|x| x < &&0)
        .map(|x| x.clone())
        .collect::<Vec<i32>>();
    assert!(neg_vector.len() > 0);
    let result = vector.iter().cnt_sort();
    assert!(result.is_ok());
    vector.sort();
    assert_eq!(vector, result.unwrap());
}

#[test]
fn test_does_not_panic_with_signed() {
    let vector: Vec<i8> = vec![-128, 127];
    let sorted_vector = vector.iter().cnt_sort().unwrap();
    assert_eq!(vector, sorted_vector);
}
