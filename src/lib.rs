// https://www.cs.usfca.edu/~galles/visualization/CountingSort.html
// https://en.wikipedia.org/wiki/Counting_sort

// Todos:
// 2. Re-execute benchmarks
// 3. Tests: unit, component, docs
//    * code coverage either via tarpaulin or kcov or both
//    * i8, i16, i32, u8, u16, u32
//    * test for lists, sets etc.
//    * Integration tests for larger examples and using lists & vectors
//    * Doc tests for all public methods
// 4. Only "link" against core not std (if possible)
//    * Change Error type so that std is no longer required
//    * You need Vec currently, core is without any allocation afaik
//    * You could only return a slice, but it would have to be created with a macro
// 5. Profile
// 6. Optimizations
//    * Copy elements into vector may result in less copies of the element
//    * currently 2-3 copies per element due to TryInto
//    * T:Clone instead of T copy?
// 7. Analyze / Inspect / Evaluate, or add more errors + 2 versions (abort when too much memory or execute anyway)
//    * Used memory and runtime
// 8. Move benchmark into own library due to long build and test times
// 9. Publish?

use core::cmp::{max, min, Ord};
use core::convert::TryInto;
use core::fmt;
use core::fmt::Display;
use core::ops::Sub;
use std::error::Error;

#[derive(Debug)]
pub enum CountingSortError {
    IntoUsizeError(&'static str),
    MinValueLargerThanValue(&'static str),
    IteratorEmpty(&'static str),
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
            CountingSortError::MinValueLargerThanValue(description) => description.fmt(f),
            CountingSortError::IteratorEmpty(description) => description.fmt(f),
            CountingSortError::SortingUnnecessary(description) => description.fmt(f),
        }
    }
}

impl Error for CountingSortError {}

impl CountingSortError {
    fn from_try_into_error() -> CountingSortError {
        CountingSortError::IntoUsizeError("Out of range integral type conversion attempted")
    }

    fn from_min_value_larger_than_value() -> CountingSortError {
        CountingSortError::MinValueLargerThanValue("Given min_value is larger than value")
    }

    fn from_empty_iterator() -> CountingSortError {
        CountingSortError::IteratorEmpty("There are no element available in the iterator")
    }

    fn from_sorting_unnecessary() -> CountingSortError {
        CountingSortError::SortingUnnecessary(
            "Minimum value is identical to maximum value, therefore no sorting is necessary",
        )
    }
}

pub trait CountingSort<'a, T>
where
    T: Ord + Copy + Sub<Output = T> + TryIntoIndex + 'a,
    Self: Clone + Sized + DoubleEndedIterator<Item = &'a T>,
{
    fn cnt_sort(self) -> Result<Vec<T>, CountingSortError> {
        counting_sort(self)
    }

    fn cnt_sort_min_max(self, min_value: &T, max_value: &T) -> Result<Vec<T>, CountingSortError> {
        counting_sort_min_max(self, min_value, max_value)
    }
}

impl<'a, T, ITER> CountingSort<'a, T> for ITER
where
    T: Ord + Copy + Sub<Output = T> + TryIntoIndex + 'a,
    ITER: Sized + DoubleEndedIterator<Item = &'a T> + Clone,
{
}

pub trait TryIntoIndex {
    type Error;
    fn try_into_index(value: &Self, min_value: &Self) -> Result<usize, Self::Error>;
}

macro_rules! try_into_index_impl_for_signed {
    ($smaller_int:ty,$larger_int:ty) => {
        impl TryIntoIndex for $smaller_int {
            type Error = CountingSortError;

            fn try_into_index(value: &Self, min_value: &Self) -> Result<usize, Self::Error> {
                if min_value <= value {
                    let result = <$larger_int>::try_into(
                        <$larger_int>::from(*value) - <$larger_int>::from(*min_value),
                    );
                    if result.is_err() {
                        Err(CountingSortError::from_try_into_error())
                    } else {
                        Ok(result.unwrap_or(0))
                    }
                } else {
                    Err(CountingSortError::from_min_value_larger_than_value())
                }
            }
        }
    };
}

macro_rules! try_into_index_impl_for_unsigned {
    ($unsigned:ty) => {
        impl TryIntoIndex for $unsigned {
            type Error = CountingSortError;

            fn try_into_index(value: &Self, min_value: &Self) -> Result<usize, Self::Error> {
                if min_value <= value {
                    let result = <$unsigned>::try_into(*value - *min_value);
                    if result.is_err() {
                        Err(CountingSortError::from_try_into_error())
                    } else {
                        Ok(result.unwrap_or(0))
                    }
                } else {
                    Err(CountingSortError::from_min_value_larger_than_value())
                }
            }
        }
    };
}

macro_rules! try_into_index_impl_for_small_unsigned {
    ($unsigned:ty) => {
        impl TryIntoIndex for $unsigned {
            type Error = CountingSortError;

            fn try_into_index(value: &Self, min_value: &Self) -> Result<usize, Self::Error> {
                if min_value <= value {
                    Ok(usize::from(*value - *min_value))
                } else {
                    Err(CountingSortError::from_min_value_larger_than_value())
                }
            }
        }
    };
}

try_into_index_impl_for_signed!(i8, i16);
try_into_index_impl_for_signed!(i16, i32);
try_into_index_impl_for_signed!(i32, i64);
try_into_index_impl_for_small_unsigned!(u8);
try_into_index_impl_for_small_unsigned!(u16);
try_into_index_impl_for_unsigned!(u32);

fn counting_sort<'a, ITER, T>(iterator: ITER) -> Result<Vec<T>, CountingSortError>
where
    ITER: DoubleEndedIterator<Item = &'a T> + Clone,
    T: Ord + Copy + TryIntoIndex + Sub<Output = T> + 'a,
{
    let optional_tuple = get_min_max(&mut iterator.clone());
    if optional_tuple.is_some() {
        let (min_value, max_value) = optional_tuple.unwrap();
        counting_sort_min_max(iterator, min_value, max_value)
    } else {
        Err(CountingSortError::from_empty_iterator())
    }
}

fn counting_sort_min_max<'a, ITER, T>(
    iterator: ITER,
    min_value: &T,
    max_value: &T,
) -> Result<Vec<T>, CountingSortError>
where
    ITER: DoubleEndedIterator<Item = &'a T> + Clone,
    T: Ord + Copy + TryIntoIndex + Sub<Output = T> + 'a,
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
    let length = *count_vector.last().unwrap(); // it's safe to unwrap, since vector has at least one element
    let sorted_vector_result = re_order(iterator, &mut count_vector, length, &min_value);
    if sorted_vector_result.is_err() {
        return Err(CountingSortError::from_try_into_error());
    } else {
        return Ok(sorted_vector_result.unwrap_or(vec![]));
    }
}

fn re_order<'a, T, ITER>(
    iterator: ITER,
    count_vector: &mut Vec<usize>,
    length: usize,
    min_value: &T,
) -> Result<Vec<T>, <T as TryIntoIndex>::Error>
where
    T: Ord + Copy + TryIntoIndex + Sub<Output = T> + 'a,
    ITER: DoubleEndedIterator<Item = &'a T>,
{
    let mut sorted_vector: Vec<T> = vec![*min_value; length];
    for value in iterator.rev() {
        let index_count_vector = T::try_into_index(value, min_value)?;
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
) -> Result<Vec<usize>, <T as TryIntoIndex>::Error>
where
    ITER: Iterator<Item = &'a T>,
    T: Ord + Copy + TryIntoIndex + Sub<Output = T> + 'a,
{
    let length = T::try_into_index(max_value, min_value)? + 1;
    let mut count_vector: Vec<usize> = vec![0; length];

    for value in iterator {
        let index = T::try_into_index(value, min_value)?;
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
mod unit_tests {

    use super::*;

    const TEST_ARRAY_MIN_VALUE: u8 = 1;

    const TEST_ARRAY_MAX_VALUE: u8 = 30;

    const TEST_ARRAY_UNSORTED: [u8; 30] = [
        13, 24, 27, 3, 10, 1, 9, 17, 6, 7, 3, 30, 14, 15, 2, 3, 7, 11, 21, 16, 7, 11, 21, 5, 23,
        25, 26, 28, 28, 4,
    ];

    const TEST_ARRAY_SORTED: [u8; 30] = [
        1, 2, 3, 3, 3, 4, 5, 6, 7, 7, 7, 9, 10, 11, 11, 13, 14, 15, 16, 17, 21, 21, 23, 24, 25, 26,
        27, 28, 28, 30,
    ];

    const TEST_COUNT_VALUES_ARRAY: [usize; 30] = [
        1, 1, 3, 1, 1, 1, 3, 0, 1, 1, 2, 0, 1, 1, 1, 1, 1, 0, 0, 0, 2, 0, 1, 1, 1, 1, 1, 2, 0, 1,
    ];

    const TEST_PREFIX_SUM_ARRAY: [usize; 30] = [
        1, 2, 5, 6, 7, 8, 11, 11, 12, 13, 15, 15, 16, 17, 18, 19, 20, 20, 20, 20, 22, 22, 23, 24,
        25, 26, 27, 29, 29, 30,
    ];

    #[test]
    fn test_cnt_sort_i8_vector() {
        let test_vector: Vec<i8> = vec![2, -2, 1, -6];
        let sorted_vector = counting_sort(test_vector.iter()).unwrap();
        assert_eq!(vec![-6, -2, 1, 2], sorted_vector);
    }

    #[test]
    fn test_cnt_sort_i8_vector_with_overflow() {
        let test_vector: Vec<i8> = vec![2, -100, 50, -6];
        let sorted_vector = counting_sort(test_vector.iter()).unwrap();
        assert_eq!(vec![-100, -6, 2, 50], sorted_vector);
    }

    #[test]
    fn test_cnt_sort_u8_vector() {
        let mut test_vector = TEST_ARRAY_UNSORTED.to_vec();
        test_vector = test_vector.iter().cnt_sort().unwrap();
        let sorted_vector = TEST_ARRAY_SORTED.to_vec();
        assert_eq!(sorted_vector, test_vector);
    }

    #[test]
    fn test_cnt_sort_min_max_u8_vector() {
        let mut test_vector = TEST_ARRAY_UNSORTED.to_vec();
        test_vector = test_vector
            .iter()
            .cnt_sort_min_max(&TEST_ARRAY_MIN_VALUE, &TEST_ARRAY_MAX_VALUE)
            .unwrap();
        let sorted_vector = TEST_ARRAY_SORTED.to_vec();
        assert_eq!(sorted_vector, test_vector);
    }

    #[test]
    fn test_into_index_i8() {
        assert_eq!(255, i8::try_into_index(&127, &-128).unwrap());
        assert_eq!(0, i8::try_into_index(&-128, &-128).unwrap());
        assert_eq!(150, i8::try_into_index(&50, &-100).unwrap());
        assert_eq!(50, i8::try_into_index(&-50, &-100).unwrap());
        assert_eq!(27, i8::try_into_index(&127, &100).unwrap());
    }

    #[test]
    fn test_into_index_i16() {
        assert_eq!(0xFFFF, i16::try_into_index(&32767, &-32768).unwrap());
        assert_eq!(0, i16::try_into_index(&-32768, &-32768).unwrap());
        assert_eq!(0, i16::try_into_index(&32767, &32767).unwrap());
    }

    #[test]
    fn test_into_index_i32() {
        assert_eq!(
            0xFFFFFFFF,
            i32::try_into_index(&2147483647, &-2147483648).unwrap()
        );
        assert_eq!(0, i32::try_into_index(&-2147483648, &-2147483648).unwrap());
        assert_eq!(1, i32::try_into_index(&-2147483647, &-2147483648).unwrap());
        assert_eq!(0, i32::try_into_index(&2147483647, &2147483647).unwrap());
    }

    #[test]
    fn test_into_index_u8() {
        assert_eq!(255, u8::try_into_index(&255, &0).unwrap());
        assert_eq!(0, u8::try_into_index(&0, &0).unwrap());
        assert_eq!(0, u8::try_into_index(&255, &255).unwrap());
        assert_eq!(50, u8::try_into_index(&150, &100).unwrap());
        assert_eq!(50, u8::try_into_index(&100, &50).unwrap());
        assert_eq!(27, i8::try_into_index(&127, &100).unwrap());
    }

    #[test]
    fn test_into_index_u16() {
        assert_eq!(0xFFFF, u16::try_into_index(&0xFFFF, &0).unwrap());
        assert_eq!(0, u16::try_into_index(&0, &0).unwrap());
        assert_eq!(0, u16::try_into_index(&0xFFFF, &0xFFFF).unwrap());
        assert_eq!(1, u16::try_into_index(&0xFFFF, &0xFFFE).unwrap());
    }

    #[test]
    fn test_into_index_u32() {
        assert_eq!(0xFFFFFFFF, u32::try_into_index(&0xFFFFFFFF, &0).unwrap());
        assert_eq!(0, u32::try_into_index(&0, &0).unwrap());
        assert_eq!(50, u32::try_into_index(&1000000, &999950).unwrap());
        assert_eq!(50, u8::try_into_index(&100, &50).unwrap());
        assert_eq!(27, i8::try_into_index(&127, &100).unwrap());
    }

    #[test]
    fn test_into_index_error() {
        let error = i8::try_into_index(&-127, &127);
        assert_eq!(
            "Given min_value is larger than value",
            format!("{}", error.unwrap_err())
        );
        let error = u8::try_into_index(&0, &127);
        assert_eq!(
            "Given min_value is larger than value",
            format!("{}", error.unwrap_err())
        );
        let error = u32::try_into_index(&0, &127);
        assert_eq!(
            "Given min_value is larger than value",
            format!("{}", error.unwrap_err())
        );
    }

    #[test]
    fn test_counting_sort() {
        let test_vector: Vec<u8> = TEST_ARRAY_UNSORTED.to_vec();
        let sorted_vector = counting_sort(test_vector.iter()).unwrap();
        let expected_vector = TEST_ARRAY_SORTED.to_vec();
        assert_eq!(expected_vector, sorted_vector);
    }

    #[test]
    fn test_counting_sort_min_max() {
        let test_vector: Vec<u8> = TEST_ARRAY_UNSORTED.to_vec();
        let sorted_vector = counting_sort_min_max(
            test_vector.iter(),
            &TEST_ARRAY_MIN_VALUE,
            &TEST_ARRAY_MAX_VALUE,
        )
        .unwrap();
        let expected_vector = TEST_ARRAY_SORTED.to_vec();
        assert_eq!(expected_vector, sorted_vector);
    }

    #[test]
    fn test_count_values() {
        let test_vector = TEST_ARRAY_UNSORTED.to_vec();
        let result_count_value_vector = count_values(
            &mut test_vector.iter(),
            &TEST_ARRAY_MIN_VALUE,
            &TEST_ARRAY_MAX_VALUE,
        );
        assert!(result_count_value_vector.is_ok());
        let count_values_vector = result_count_value_vector.unwrap();
        let expected_vector = TEST_COUNT_VALUES_ARRAY.to_vec();
        assert_eq!(expected_vector, count_values_vector);
    }

    #[test]
    fn test_get_min_max_unsigned() {
        let test_vector: Vec<u8> = vec![1, 2, 3, 4];
        let tuple = get_min_max(&mut test_vector.iter());
        assert!(tuple.is_some());
        let (min_value, max_value) = tuple.unwrap();
        assert_eq!(1, *min_value);
        assert_eq!(4, *max_value);
    }

    #[test]
    fn test_get_min_max_without_min() {
        let test_vector: Vec<u8> = Vec::new();
        let tuple = get_min_max(&mut test_vector.iter());
        assert!(tuple.is_none());
    }

    #[test]
    fn test_get_min_max_signed() {
        let test_vector: Vec<i8> = vec![-128, 2, 3, 127];
        let tuple = get_min_max(&mut test_vector.iter());
        assert!(tuple.is_some());
        let (min_value, max_value) = tuple.unwrap();
        assert_eq!(-128, *min_value);
        assert_eq!(127, *max_value);
    }

    #[test]
    fn test_calculate_prefix_sum_1() {
        let mut test_vector: Vec<usize> = vec![1; 4];
        calculate_prefix_sum(&mut test_vector);
        assert_eq!(vec![1, 2, 3, 4], test_vector);
    }

    #[test]
    fn test_calculate_prefix_sum_2() {
        let mut test_vector: Vec<usize> = vec![1, 2, 3, 4, 5];
        calculate_prefix_sum(&mut test_vector);
        assert_eq!(vec![1, 3, 6, 10, 15], test_vector);
    }

    #[test]
    fn test_calculate_prefix_sum_3() {
        let mut test_vector = TEST_COUNT_VALUES_ARRAY.to_vec();
        calculate_prefix_sum(&mut test_vector);
        assert_eq!(TEST_PREFIX_SUM_ARRAY.to_vec(), test_vector);
    }

    #[test]
    fn test_re_order() {
        let test_vector = TEST_ARRAY_UNSORTED.to_vec();
        let mut test_count_vector = TEST_PREFIX_SUM_ARRAY.to_vec();
        let result_sorted_vector = re_order(
            test_vector.iter(),
            &mut test_count_vector,
            test_vector.len(),
            &TEST_ARRAY_MIN_VALUE,
        );
        assert!(result_sorted_vector.is_ok());
        let sorted_vector = result_sorted_vector.unwrap();
        assert_eq!(TEST_ARRAY_SORTED.to_vec(), sorted_vector);
    }

    #[test]
    fn test_sorting_unnecessary_error() {
        let test_vector = vec![1];
        let result = counting_sort_min_max(test_vector.iter(), &1, &1);
        assert!(result.is_err());
        assert_eq!(
            "Minimum value is identical to maximum value, therefore no sorting is necessary",
            format!("{}", result.unwrap_err())
        );
    }

    #[test]
    fn test_empty_iterator_error() {
        let test_vector: Vec<u8> = vec![];
        let result = counting_sort(test_vector.iter());
        assert!(result.is_err());
        assert_eq!(
            "There are no element available in the iterator",
            format!("{}", result.unwrap_err())
        );
    }

    #[test]
    fn test_try_into_error() {
        #[derive(Ord, PartialOrd, PartialEq, Eq, Copy, Clone, Debug)]
        struct ValueWithTryIntoError {
            value: u8,
        };

        impl Sub for ValueWithTryIntoError {
            type Output = ValueWithTryIntoError;
            fn sub(self, other: ValueWithTryIntoError) -> Self::Output {
                ValueWithTryIntoError {
                    value: self.value - other.value,
                }
            }
        }

        let min_value = ValueWithTryIntoError { value: 0 };
        let max_value = ValueWithTryIntoError {
            value: u8::max_value(),
        };

        impl TryIntoIndex for ValueWithTryIntoError {
            type Error = String;
            fn try_into_index(_value: &Self, _min_value: &Self) -> Result<usize, Self::Error> {
                Err(String::from("TryInto always fails"))
            }
        }

        let test_vector: Vec<ValueWithTryIntoError> = Vec::new();
        let result = counting_sort_min_max(test_vector.iter(), &min_value, &max_value);
        assert!(result.is_err());
        assert_eq!("Error from TryInto<usize>. Original error message: Out of range integral type conversion attempted.", format!("{}", result.unwrap_err()));
    }

    #[test]
    fn test_empty_count_values_vector_is_impossible() {
        #[derive(Ord, PartialOrd, PartialEq, Eq, Copy, Clone, Debug)]
        struct ValueWithWrongSubstraction {
            value: usize,
        };

        impl Sub for ValueWithWrongSubstraction {
            type Output = ValueWithWrongSubstraction;
            fn sub(self, _other: ValueWithWrongSubstraction) -> Self::Output {
                ValueWithWrongSubstraction { value: 0 }
            }
        }

        let min_value = ValueWithWrongSubstraction { value: 0 };
        let max_value = ValueWithWrongSubstraction {
            value: usize::max_value(),
        };

        impl TryIntoIndex for ValueWithWrongSubstraction {
            type Error = String;
            fn try_into_index(_value: &Self, _min_value: &Self) -> Result<usize, Self::Error> {
                Ok(0)
            }
        }

        let test_vector: Vec<ValueWithWrongSubstraction> = Vec::new();
        let result = counting_sort_min_max(test_vector.iter(), &min_value, &max_value);
        assert!(result.is_ok());
        assert_eq!(test_vector, result.unwrap());
    }
}
