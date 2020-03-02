use counting_sort::CountingSort;

use std::collections::LinkedList;

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
