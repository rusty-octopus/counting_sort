// https://www.cs.usfca.edu/~galles/visualization/CountingSort.html
// https://en.wikipedia.org/wiki/Counting_sort

// Todos:
// 1. Only "link" against core not std (if possible)
//    * Change Error type so that std is no longer required
//    * You need Vec currently, core is without any allocation afaik
//    * You could only return a slice, but it would have to be created with a macro
// 2. Tests: unit, component, docs
//    * code coverage either via tarpaulin or kcov or both
//    * i8, i16, i32, u8, u16, u32
//    * Test for errors: e.g. when TryInto may fail
//    * test for lists, sets etc.
//    * Unit tests for all package private functions & all functions but with smaller examples
//    * Integration tests for larger examples and using lists & vectors
//    * Doc tests for all public methods
// 3. Profile
// 4. Optimizations
//    * Copy elements into vector may result in less copies of the element
//    * currently 2-3 copies per element due to TryInto
//    * T:Clone instead of T copy?
// 5. Analyze / Inspect / Evaluate, or add more errors + 2 versions (abort when too much memory or execute anyway)
//    * Used memory and runtime
// 6. Publish?

use core::cmp::{max, min, Ord};
use core::convert::TryInto;
use core::fmt;
use core::fmt::Display;
use core::ops::Sub;
use std::error::Error;

#[derive(Debug)]
pub enum CountingSortError {
    IntoUsizeError(&'static str),
    IteratorEmpty(&'static str),
    EmptyCountValuesVector(&'static str),
    SortingUnnecessary(&'static str),
}

impl Display for CountingSortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CountingSortError::IntoUsizeError(description) => write!(
                f,
                "Error from TryInto<usize>. Original error message: {}.",
                description
            ),
            CountingSortError::IteratorEmpty(description) => description.fmt(f),
            CountingSortError::EmptyCountValuesVector(description) => description.fmt(f),
            CountingSortError::SortingUnnecessary(description) => description.fmt(f),
        }
    }
}

impl Error for CountingSortError {}

impl CountingSortError {
    fn from_try_into_error() -> CountingSortError {
        CountingSortError::IntoUsizeError("Out of range integral type conversion attempted")
    }

    fn from_empty_iterator() -> CountingSortError {
        CountingSortError::IteratorEmpty("There are no element available in the iterator")
    }

    fn from_empty_count_values_vector() -> CountingSortError {
        CountingSortError::IteratorEmpty(
            "The count values vector is empty which should not have happened",
        )
    }

    fn from_sorting_unnecessary() -> CountingSortError {
        CountingSortError::SortingUnnecessary(
            "Minimum value is identical to maximum value. Therefore no sorting is necessary",
        )
    }
}

pub trait CountingSort<'a, T>
where
    T: Ord + Copy + Sub<Output = T> + TryInto<usize> + 'a,
    Self: Clone + Sized + DoubleEndedIterator<Item = &'a T>,
{
    fn cnt_sort(self) -> Result<Vec<T>, CountingSortError> {
        counting_sort(self)
    }

    fn cnt_sort_min_max(self, min_value: &T, max_value: &T) -> Result<Vec<T>, CountingSortError> {
        counting_sort_known_min_max(self, min_value, max_value)
    }
}

impl<'a, T, ITER> CountingSort<'a, T> for ITER
where
    T: Ord + Copy + Sub<Output = T> + TryInto<usize> + 'a,
    ITER: Sized + DoubleEndedIterator<Item = &'a T> + Clone,
{
}

fn counting_sort<'a, ITER, T>(iterator: ITER) -> Result<Vec<T>, CountingSortError>
where
    ITER: DoubleEndedIterator<Item = &'a T> + Clone,
    T: Ord + Copy + TryInto<usize> + Sub<Output = T> + 'a,
{
    let optional_tuple = get_min_max(&mut iterator.clone());
    if optional_tuple.is_some() {
        let (min_value, max_value) = optional_tuple.unwrap();
        counting_sort_known_min_max(iterator, min_value, max_value)
    } else {
        Err(CountingSortError::from_empty_iterator())
    }
}

fn counting_sort_known_min_max<'a, ITER, T>(
    iterator: ITER,
    min_value: &T,
    max_value: &T,
) -> Result<Vec<T>, CountingSortError>
where
    ITER: DoubleEndedIterator<Item = &'a T> + Clone,
    T: Ord + Copy + TryInto<usize> + Sub<Output = T> + 'a,
{
    if min_value == max_value {
        return Err(CountingSortError::from_sorting_unnecessary());
    }
    let count_vector_result = count_values(&mut iterator.clone(), min_value, max_value);
    if count_vector_result.is_err() {
        return Err(CountingSortError::from_try_into_error());
    }
    let mut count_vector = count_vector_result.unwrap_or(vec![]);
    calculate_prefix_sum(&mut count_vector);
    // last element of the count vector depicts the index-1 of the largest element, hence it is its length
    let length = count_vector.last();
    if length.is_some() {
        let length = *length.unwrap();
        let sorted_vector_result = re_order(iterator, &mut count_vector, length, &min_value);
        if sorted_vector_result.is_err() {
            return Err(CountingSortError::from_try_into_error());
        } else {
            return Ok(sorted_vector_result.unwrap_or(vec![]));
        }
    } else {
        Err(CountingSortError::from_empty_count_values_vector())
    }
}

fn re_order<'a, T, ITER>(
    iterator: ITER,
    count_vector: &mut Vec<usize>,
    length: usize,
    min_value: &T,
) -> Result<Vec<T>, <T as TryInto<usize>>::Error>
where
    T: Ord + Copy + TryInto<usize> + Sub<Output = T> + 'a,
    ITER: DoubleEndedIterator<Item = &'a T>,
{
    let mut sorted_vector: Vec<T> = vec![*min_value; length];
    for value in iterator.rev() {
        let index_count_vector = T::try_into(*value - *min_value)?;
        let mut index = count_vector[index_count_vector];
        index -= 1;
        count_vector[index_count_vector] = index;
        sorted_vector[index] = *value;
    }
    Ok(sorted_vector)
}

fn count_values<'a, ITER, T>(
    iterator: &mut ITER,
    min_value: &T,
    max_value: &T,
) -> Result<Vec<usize>, <T as TryInto<usize>>::Error>
where
    ITER: Iterator<Item = &'a T>,
    T: Ord + Copy + TryInto<usize> + Sub<Output = T> + 'a,
{
    // max_value - min_value should always be >= "0"
    // however it could overflow usize
    let length = T::try_into(*max_value - *min_value)? + 1;
    let mut count_vector: Vec<usize> = vec![0; length];

    for value in iterator {
        let index = T::try_into(*value - *min_value)?;
        let new_count_value = count_vector[index] + 1;
        count_vector[index] = new_count_value;
    }

    Ok(count_vector)
}

fn calculate_prefix_sum(count_vector: &mut Vec<usize>) {
    let mut iterator = count_vector.iter_mut();
    // skip first element
    let first_element = iterator.next();
    if first_element.is_some() {
        let mut total = *first_element.unwrap();
        for value in iterator {
            total = total + *value;
            *value = total;
        }
    }
}

fn get_min_max<T, ITER>(iterator: &mut ITER) -> Option<(T, T)>
where
    T: Ord + Copy,
    ITER: Iterator<Item = T>,
{
    // consume first element to initialize as min and max value
    let min_value = iterator.next();
    if min_value.is_some() {
        let min_value = min_value.unwrap();
        let tuple = iterator.fold((min_value, min_value), |(min_val, max_val), value| {
            (min(min_val, value), max(max_val, value))
        });
        return Some(tuple);
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_for_u8() {
        let mut test_vector: Vec<u8> = vec![4, 3, 2, 1];
        test_vector = test_vector.iter().cnt_sort().unwrap();
        assert_eq!(vec![1, 2, 3, 4], test_vector);
    }

    #[test]
    fn test_for_u8_iter() {
        let test_vector: Vec<u8> = vec![4, 3, 2, 1];
        let sorted_vector = counting_sort(test_vector.iter()).unwrap();
        assert_eq!(vec![1, 2, 3, 4], sorted_vector);
    }

    #[test]
    fn test_for_u16_iter() {
        let test_vector: Vec<u16> = vec![4, 3, 2, 1];
        //let sorted_vector = vec![];// =
        let sorted_vector = counting_sort(test_vector.iter()).unwrap();
        assert_eq!(vec![1, 2, 3, 4], sorted_vector);
    }

    #[test]
    fn test_for_i8_iter() {
        let test_vector: Vec<i8> = vec![2, -2, 1, -6];
        let sorted_vector = counting_sort(test_vector.iter()).unwrap();
        assert_eq!(vec![-6, -2, 1, 2], sorted_vector);
    }

    #[test]
    fn test_counting_sort() {
        let mut test_vector = vec![
            13, 24, 27, 3, 10, 1, 9, 17, 6, 7, 3, 30, 14, 15, 2, 3, 7, 11, 21, 16, 7, 11, 21, 5,
            23, 25, 26, 28, 28, 4,
        ];
        test_vector = test_vector.iter().cnt_sort().unwrap();
        let sorted_vector = vec![
            1, 2, 3, 3, 3, 4, 5, 6, 7, 7, 7, 9, 10, 11, 11, 13, 14, 15, 16, 17, 21, 21, 23, 24, 25,
            26, 27, 28, 28, 30,
        ];

        assert_eq!(sorted_vector, test_vector);
    }

    #[test]
    fn test_counting_sort_iter() {
        let test_vector: Vec<u8> = vec![
            13, 24, 27, 3, 10, 1, 9, 17, 6, 7, 3, 30, 14, 15, 2, 3, 7, 11, 21, 16, 7, 11, 21, 5,
            23, 25, 26, 28, 28, 4,
        ];
        let sorted_vector = counting_sort(test_vector.iter()).unwrap();
        let expected_vector = vec![
            1, 2, 3, 3, 3, 4, 5, 6, 7, 7, 7, 9, 10, 11, 11, 13, 14, 15, 16, 17, 21, 21, 23, 24, 25,
            26, 27, 28, 28, 30,
        ];

        assert_eq!(expected_vector, sorted_vector);
    }

    #[test]
    fn test_unsigned_get_min_max() {
        let test_vector: Vec<u8> = vec![1, 2, 3, 4];
        let tuple = get_min_max(&mut test_vector.iter());
        assert!(tuple.is_some());
        let (min_value, max_value) = tuple.unwrap();
        assert_eq!(1, *min_value);
        assert_eq!(4, *max_value);
    }

    #[test]
    fn test_signed_get_min_max() {
        let test_vector: Vec<i8> = vec![-128, 2, 3, 127];
        let tuple = get_min_max(&mut test_vector.iter());
        assert!(tuple.is_some());
        let (min_value, max_value) = tuple.unwrap();
        assert_eq!(-128, *min_value);
        assert_eq!(127, *max_value);
    }

    #[test]
    fn test_prefix_sum_1() {
        let mut test_vector: Vec<usize> = vec![1; 4];
        calculate_prefix_sum(&mut test_vector);
        assert_eq!(vec![1, 2, 3, 4], test_vector);
    }

    #[test]
    fn test_prefix_sum_2() {
        let mut test_vector: Vec<usize> = vec![1, 2, 3, 4, 5];
        calculate_prefix_sum(&mut test_vector);
        assert_eq!(vec![1, 3, 6, 10, 15], test_vector);
    }
}
