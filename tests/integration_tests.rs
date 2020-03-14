use counting_sort::CountingSort;

use std::collections::LinkedList;

use oorandom::Rand32;

use core::ops::Range;

use core::convert::TryFrom;

#[test]
fn test_counting_sort_on_list() {
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
fn test_counting_sort_on_list_2() {
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
fn test_slice() {
    let slice: [u8; 4] = [4, 3, 2, 1];
    let sorted_vector = slice.iter().cnt_sort().unwrap();
    let test_vector = vec![1, 2, 3, 4];
    assert_eq!(sorted_vector, test_vector);
}

fn create_vector_t<T: TryFrom<u32>>(number_of_elements: usize, range: Range<u32>) -> Vec<T> {
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

#[test]
fn test_cnt_sort_min_max_on_u8_vector() {
    let number_of_elements = 10000;
    let range_min: u32 = 0;
    let range_max: u32 = 256;
    let vector = create_vector_t::<u8>(number_of_elements, range_min..range_max);
    let result = vector
        .iter()
        .cnt_sort_min_max(&(range_min as u8), &(range_max as u8));
    assert!(result.is_err());
}
