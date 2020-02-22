use counting_sort;

use std::collections::LinkedList;

#[test]
fn test_counting_sort_on_list() {
    let mut list = LinkedList::new();
    list.push_back(4);
    list.push_back(3);
    list.push_back(2);
    list.push_back(1);
    let sorted_vector = counting_sort::counting_sort(& mut list.iter()).unwrap();
    assert_eq!(vec![1,2,3,4], sorted_vector);
}