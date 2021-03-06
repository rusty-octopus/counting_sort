//! An counting sort implementation for [`Iterator`](std::iter::Iterator)s.
//!
//! Provides the trait [`CountingSort`] with a blanket implementation for
//! [`Iterator`](std::iter::Iterator)s
//! for all types `T` that implement (beyond other `std` or `core` traits) the here defined
//! [`TryIntoIndex`] trait.
//! Types that implement this trait can be tried to be converted to an
//! [`usize`](std::usize), i.e. an index.
//!
//! This trait is already implemented for the following integer types:
//!
//! * [`u8`](std::u8)
//! * [`u16`](std::u16)
//! * [`u32`](std::u32)
//! * [`usize`](std::usize)
//! * [`i8`](std::i8)
//! * [`i16`](std::i16)
//! * [`i32`](std::i32)
//!
//! This means for all [`Vec`](std::vec::Vec)s,
//! [`LinkedList`](std::collections::LinkedList)s,
//! [`slice`](std::slice)s or any other
//! of the implementors of the [`Iterator`](std::iter::Iterator)
//! trait holding one of the above integers types, counting sort can be executed.
//!
//! **Note:** Counting sort is also implemented for [`BTreeSet`](std::collections::BTreeSet),
//! however it makes no sense to execute it there, since all elements are already in order and further sorting is completely
//! useless.
//!
//! # Example
//!
//! ```rust
//! /*
//!  * Add counting sort to your source code.
//!  * counting sort immediately works "out of the box"
//!  * for all Iterators and integers like
//!  * u8, i8, u16, i16.
//!  */
//! use counting_sort::CountingSort;
//!
//! let vec = vec![2,4,1,3];
//! // counting sort may fail, therefore a result is returned
//! let sorted_vec_result = vec.iter().cnt_sort();
//!
//! assert!(sorted_vec_result.is_ok());
//! // if successful sorted elements were copied into a Vec
//! assert_eq!(vec![1,2,3,4], sorted_vec_result.unwrap());
//! ```
//!
//! # Notes
//!
//! * The counting sort algorithm has an `O(n+d)` (`d` being the range between the minimum value and the maximum value) asymptotic runtime in comparison to an `O(n*log(n))`
//!   of the Rust std library implementation of [`core::slice::sort`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort)
//! * However the memory consumption is higher
//!     * Dependent on the range `d` between the minumum value and the maximum value (`d = max_value - min_value`),
//!       a [`Vec`](std::vec::Vec) of
//!       [`usize`](std::usize)'s is allocated
//!     * This may fast result in GB of memory: the maximum range of [`u32`](std::u32) is
//!       4294967295, if usize is 4 bytes, than the memory consumption is 17179869180 bytes or approximately 16 GB
//!       (1 GB = 1024*1024*1024 bytes)
//!     * Additionally the current implementation does not consume the given iterator
//! * This means the counting sort algorithm excels whenever there are a lot of elements to be sorted but the range
//!   range between minumum value and maximum value is small
//! * counting sort for e.g. [`HashSet`](std::collections::HashSet)'s is sub-optimal since every element exists only
//!   once in a [`HashSet`](std::collections::HashSet). Counting sort excels when a lot of elements exist in the
//!   collection but the number of distinct elements is small.
//! * **<span style="color:red">Caution:</span>** Be careful using this algorithm when the range between minumum value and maximum value is large
//! * An excellent illustration about the counting sort algorithm can be found [here](https://www.cs.usfca.edu/~galles/visualization/CountingSort.html)
//! * Wikipedia article on [counting sort](https://en.wikipedia.org/wiki/Counting_sort)

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use core::cmp::{max, min, Ord};
use core::convert::TryInto;
use core::fmt;
use core::fmt::Display;
use std::error::Error;

/// This enumeration is a list of all possible errors that can happen during
/// [`cnt_sort`](CountingSort::cnt_sort()) or
/// [`cnt_sort_min_max`](CountingSort::cnt_sort_min_max()).
#[derive(Debug)]
pub enum CountingSortError {
    /// The conversion from a value of the to-be-sorted type `T` into an
    /// index ([`usize`](std::usize)) failed.
    /// Most likely due to an overflow happening.
    IntoIndexFailed(&'static str),
    /// The iterator is empty and therefore nothing can be sorted.
    IteratorEmpty(&'static str),
    /// The minimum value is equal to the maximum value, this means sorting is unnecessary.
    SortingUnnecessary(&'static str),
    /// The minimum value is larger than the maximum value, most likely due to calling
    /// [`cnt_sort_min_max`](CountingSort::cnt_sort_min_max()) with the switched
    /// parameters.
    MinValueLargerMaxValue(&'static str),
    /// The converted index is still larger than the length of the count value vector. This happens
    /// when the given maximum value is smaller than the actual maximum value when
    /// [`cnt_sort_min_max`](CountingSort::cnt_sort_min_max()) is used.
    IndexOutOfBounds(&'static str),
}

impl Display for CountingSortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CountingSortError::IntoIndexFailed(description)
            | CountingSortError::IteratorEmpty(description)
            | CountingSortError::SortingUnnecessary(description)
            | CountingSortError::MinValueLargerMaxValue(description)
            | CountingSortError::IndexOutOfBounds(description) => description.fmt(f),
        }
    }
}

impl Error for CountingSortError {}

impl CountingSortError {
    /// Create `IntoIndexFailed` error when conversion to index failed.
    fn from_try_into_index_failed() -> CountingSortError {
        CountingSortError::IntoIndexFailed("Conversion into index failed")
    }

    /// Create `IteratorEmpty` error when the iterator is empty.
    fn from_empty_iterator() -> CountingSortError {
        CountingSortError::IteratorEmpty("There are no element available in the iterator")
    }

    /// Create `SortingUnnecessary` when minimum value equals maximum value.
    fn from_sorting_unnecessary() -> CountingSortError {
        CountingSortError::SortingUnnecessary(
            "Minimum value is identical to maximum value, therefore no sorting is necessary",
        )
    }

    /// Create `SortingUnnecessary` when minimum value equals maximum value.
    fn from_min_value_larger_max_value() -> CountingSortError {
        CountingSortError::MinValueLargerMaxValue("Minimum value is larger than maximum value")
    }

    /// Create `IndexOutOfBounds` when minimum value equals maximum value.
    fn from_index_out_of_bounds() -> CountingSortError {
        CountingSortError::IndexOutOfBounds(
            "Index is out of bounds, most likely the given maximum value is too small",
        )
    }
}

/// The interface for counting sort algorithm.
///
/// Interface provides blanket implementation of all collections that implement
/// the [`Iterator`](std::iter::Iterator)
/// trait. These collections must also implement
/// [`Clone`](std::clone::Clone), since the iterator is iterated several times,
/// and [`Sized`](std::marker::Sized). If your collection does provide these,
/// you can simply implement this trait "empty":
///
/// ```rust,compile_fail
/// impl CountingSort for YourType {}
/// ```
///
/// However the intention of this trait is to provide an implementation of all collections that
/// implement the
/// [`Iterator`](std::iter::Iterator)
/// trait like [`Vec`](std::vec::Vec).
///
/// The types which are held by the collections must implement
/// [`Ord`](std::cmp::Ord) in order to sort the elements, as well
/// as [`Copy`](std::marker::Copy), since the elements are copied
/// during the count phase as well as the re-order phase. Finally the type must implement the in this
/// crate defined [`TryIntoIndex`] trait.
pub trait CountingSort<'a, T>
where
    T: Ord + Copy + TryIntoIndex + 'a,
    Self: Clone + Sized + Iterator<Item = &'a T>,
{
    /// Sorts the elements in the
    /// [`Iterator`](std::iter::Iterator)
    /// with the counting sort algorithm.
    ///
    /// This sort is stable (i.e., does not reorder equal elements) and `O(n + d)` worst-case,
    /// where `d` is the distance between the maximum and minimum element in the collection.
    ///
    /// Memory usage is `O(n + d)` as well, since all elements of the collection are copied into a new
    /// [`Vec`](std::vec::Vec) and the frequency of all
    /// elements in the collection are counted in a [`Vec`](std::vec::Vec)
    /// of size `d`.
    ///
    /// **<span style="color:red">Caution:</span>** If distance `d` is large, than memory consumption is large
    /// and you process may run out of memory.
    ///
    /// This method iterates [`Iterator`](std::iter::Iterator)
    /// in the beginning to identify the maximum and mimumum value in order to identify the distance `d`. This means
    /// the runtime is longer due to this additional `n` iterations and the checks needed to identify the minimum and
    /// maximum values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use counting_sort::CountingSort;
    ///
    /// let slice = [20000,-1000,17,333];
    /// let sorted_vec_result = slice.iter().cnt_sort();
    ///
    /// assert_eq!(vec![-1000,17,333,20000], sorted_vec_result.unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// * [`CountingSortError::IntoIndexFailed`] when
    ///   converting into an index fails, this could happen if the distance `d` is larger than
    ///   [`usize::max_value`](https://doc.rust-lang.org/nightly/std/primitive.usize.html#method.max_value)
    /// * [`CountingSortError::IteratorEmpty`] when the iterator
    ///   is empty (and there is nothing to sort)
    /// * [`CountingSortError::SortingUnnecessary`]] when
    ///   the minimum value is equal to the maximum value, this means all values are essentially equal and no sorting
    ///   is necessary
    fn cnt_sort(self) -> Result<Vec<T>, CountingSortError> {
        counting_sort(self)
    }

    /// Sorts the elements in the
    /// [`Iterator`](std::iter::Iterator)
    /// with the counting sort algorithm given the minimum and maximum element of the collection.
    ///
    /// This sort is stable (i.e., does not reorder equal elements) and `O(n + d)` worst-case,
    /// where `d` is the distance between the maximum and minimum element in the collection.
    ///
    /// Memory usage is `O(n + d)` as well, since all elements of the collection are copied into a new
    /// [`Vec`](std::vec::Vec) and the frequency of all
    /// elements in the collection are counted in a [`Vec`](std::vec::Vec)
    /// of size `d`.
    ///
    /// **<span style="color:red">Caution:</span>** If distance `d` is large, than memory consumption is large
    /// and you process may run out of memory.
    ///
    /// This method uses the given minimum and maximum element and therefore does not need to iterate the iterator
    /// to identify the minimum and maximum element.
    ///
    /// **<span style="color:red">Caution:</span>** If any element is either larger than the given maximum value
    /// or smaller than the given minimum value, the method will return with an error. Only use this method if
    /// you know these values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::LinkedList;
    /// use counting_sort::CountingSort;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back(1000001);
    /// list.push_back(1000003);
    /// list.push_back(1000002);
    /// list.push_back(1000000);
    ///
    /// let sorted_vec_result = list.iter().cnt_sort_min_max(&1000000, &1000003);
    ///
    /// assert_eq!(vec![1000000, 1000001, 1000002, 1000003], sorted_vec_result.unwrap());
    ///
    /// // minimum value incorrect
    /// let error = list.iter().cnt_sort_min_max(&1000001, &1000003);
    /// assert!(error.is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// * [`CountingSortError::IntoIndexFailed`] when
    ///   converting into an index fails, this could happen if the distance `d` is larger than
    ///   [`usize::max_value`](https://doc.rust-lang.org/nightly/std/primitive.usize.html#method.max_value)
    /// * [`CountingSortError::SortingUnnecessary`]] when
    ///   the minimum value is equal to the maximum value, this means all values are essentially equal and no sorting
    ///   is necessary
    /// * [`CountingSortError::MinValueLargerMaxValue`]] when
    ///   the given minimum value is larger than the given maximum value
    /// * [`CountingSortError::IndexOutOfBounds`]] when
    ///   the given maximum value is smaller than the actual maximum value of the collection
    fn cnt_sort_min_max(self, min_value: &T, max_value: &T) -> Result<Vec<T>, CountingSortError> {
        counting_sort_min_max(self, min_value, max_value)
    }
}

// Counting sort implementation for ITER with trait bound Iterator.
// This enables that CountingSort is implemented for all implementors of
// Iterator, especially for Vec, LinkedList and slice.
impl<'a, T, ITER> CountingSort<'a, T> for ITER
where
    T: Ord + Copy + TryIntoIndex + 'a,
    ITER: Sized + Iterator<Item = &'a T> + Clone,
{
}

/// The interface for converting values into an index.
///
/// Index is always [`usize`](std::usize). Unfortunatelly
/// [`TryInto`](std::convert::TryInto) for
/// [`usize`](std::usize) is not sufficient since signed
/// integers overflow when calculating `max_value - min_value`. Therefore this trait was added to
/// implement an non-overflowing conversion to [`usize`](std::usize).
///
/// You can implement this trait yourself as long as there is a natural conversion from your type to
/// [`usize`](std::usize). However it must hold for your type that if
/// `t_1 <= t_2` then `YourType::try_into_index(t_1, min_value)? <= YourType::try_into_index(t_2, min_value)?`.
/// Also consider that the size [`Vec`](std::vec::Vec) that holds the
/// frequency of all elements in the collection is calculated like this
///
/// ```rust,compile_fail
/// let length = YourType::try_into_index(max_value,min_value)? + 1;
/// ```
///
/// It is not highly recommended to do this if your type's order is not simply dependent on one integer value
/// of your struct.
///
/// # Example
///
/// ```rust
/// use core::cmp::{Ord, Ordering};
/// use counting_sort::TryIntoIndex;
///
/// #[derive(Copy, Clone)]
/// struct Person {
///     name: &'static str,
///     id: usize
/// }
///
/// impl Ord for Person {
///     fn cmp(&self, other: &Self) -> Ordering {
///         self.id.cmp(&other.id)
///     }
/// }
///
/// impl PartialOrd for Person {
///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///         Some(self.cmp(other))
///     }
/// }
///
/// impl PartialEq for Person {
///     fn eq(&self, other: &Self) -> bool {
///         self.id == other.id
///     }
/// }
///
/// impl Eq for Person {}
///
/// impl TryIntoIndex for Person {
///     type Error = &'static str;
///     
///     fn try_into_index(value: &Self, min_value: &Self) -> Result<usize, Self::Error> {
///         Ok(value.id - min_value.id)
///     }
/// }
///
/// let john = Person { name: "John", id: 1234 };
/// let min_value = Person { name: "Jim", id: 234 };
/// let index_result = Person::try_into_index(&john, &min_value);
/// assert!(index_result.is_ok());
/// assert_eq!(1000, index_result.unwrap_or(0));
/// ```
pub trait TryIntoIndex {
    /// The type returned whenever the conversion into an index failed.
    type Error;

    /// Tries to convert the value into an index.
    ///
    /// The `min_value` parameter is for calculating the offset between the actual value
    /// and the minimum value. This concept is used in order to only allocate a
    /// [`Vec`](std::vec::Vec) that only covers the
    /// distance between the maximum value and the minimum value of the collection.
    ///
    /// # Errors
    ///
    /// Shall return an `Error` when converting value into an index (`usize`) fails.
    fn try_into_index(value: &Self, min_value: &Self) -> Result<usize, Self::Error>;
}

// Macro is needed to implement TryIntoIndex for signed integers, which can overflow during
// index conversion.
macro_rules! try_into_index_impl_for_signed {
    ($smaller_int:ty,$larger_int:ty) => {
        impl TryIntoIndex for $smaller_int {
            type Error = <$larger_int as TryInto<usize>>::Error;

            fn try_into_index(value: &Self, min_value: &Self) -> Result<usize, Self::Error> {
                // This conversion can only fail, if the larger integer type has a larger maximum
                // value then usize. To-be-converted value is always be >= 0 as long as min_value <=
                // value.
                <$larger_int>::try_into(
                    // convert smaller signed integer into larger signed integer to
                    // avoid integer overflow for the smaller integer.
                    // Example: 127 - (-128) overflows in i8 since 255 > 127 = i8::max_value().
                    // It is safe to convert a smaller integer into a larger integer.
                    <$larger_int>::from(*value) - <$larger_int>::from(*min_value),
                )
            }
        }
    };
}

// Macro used for unsigned integer implementations of TryIntoIndex.
macro_rules! try_into_index_impl_for_unsigned {
    ($unsigned:ty) => {
        impl TryIntoIndex for $unsigned {
            type Error = <$unsigned as TryInto<usize>>::Error;

            #[inline]
            fn try_into_index(value: &Self, min_value: &Self) -> Result<usize, Self::Error> {
                // Unsigned integer (e.g. u32) could be larger than usize on some HW.
                <$unsigned>::try_into(*value - *min_value)
            }
        }
    };
}

// Macro used for small unsigned integer implementations of TryIntoIndex.
macro_rules! try_into_index_impl_for_small_unsigned {
    ($unsigned:ty) => {
        impl TryIntoIndex for $unsigned {
            type Error = CountingSortError;
            #[inline]
            fn try_into_index(value: &Self, min_value: &Self) -> Result<usize, Self::Error> {
                // u8 and u16 should always fit into an usize. Therefore no TryInto is needed.
                Ok(usize::from(*value - *min_value))
            }
        }
    };
}

// macro instances for signed integer implementation of TryIntoIndex
try_into_index_impl_for_signed!(i8, i16);
try_into_index_impl_for_signed!(i16, i32);
try_into_index_impl_for_signed!(i32, i64);
// i64 was not added, since i64 could be larger than usize and can also
// result in huge memory consumption if the distance between max_value and
// min_value of the collection is huge.

// macro instances for small unsigned integer implementation of TryIntoIndex
try_into_index_impl_for_small_unsigned!(u8);
try_into_index_impl_for_small_unsigned!(u16);

// macro instances for unsigned integer implementation of TryIntoIndex
try_into_index_impl_for_unsigned!(u32);
try_into_index_impl_for_unsigned!(usize);
// u64 was not added, since i64 could be larger than usize and can also
// result in huge memory consumption if the distance between max_value and
// min_value of the collection is huge.

#[inline]
fn counting_sort<'a, ITER, T>(iterator: ITER) -> Result<Vec<T>, CountingSortError>
where
    ITER: Iterator<Item = &'a T> + Clone,
    T: Ord + Copy + TryIntoIndex + 'a,
{
    let optional_tuple = get_min_max(&mut iterator.clone());
    if let Some((min_value, max_value)) = optional_tuple {
        counting_sort_min_max(iterator, min_value, max_value)
    } else {
        Err(CountingSortError::from_empty_iterator())
    }
}

#[inline]
fn counting_sort_min_max<'a, ITER, T>(
    iterator: ITER,
    min_value: &T,
    max_value: &T,
) -> Result<Vec<T>, CountingSortError>
where
    ITER: Iterator<Item = &'a T> + Clone,
    T: Ord + Copy + TryIntoIndex + 'a,
{
    if min_value == max_value {
        return Err(CountingSortError::from_sorting_unnecessary());
    }
    if min_value > max_value {
        return Err(CountingSortError::from_min_value_larger_max_value());
    }
    let mut count_vector = count_values(&mut iterator.clone(), min_value, max_value)?;

    calculate_prefix_sum(&mut count_vector);
    // last element of the count vector depicts the index-1 of the largest element, hence it is its length
    let length = *count_vector.last().unwrap(); // it's safe to unwrap, since vector has at least one element
    re_order(iterator, &mut count_vector, length, &min_value)
}

#[inline]
fn re_order<'a, T, ITER>(
    iterator: ITER,
    count_vector: &mut Vec<usize>,
    length: usize,
    min_value: &T,
) -> Result<Vec<T>, CountingSortError>
where
    T: Ord + Copy + TryIntoIndex + 'a,
    ITER: Iterator<Item = &'a T>,
{
    let mut sorted_vector: Vec<T> = vec![*min_value; length];
    for value in iterator {
        let index_count_vector_result = T::try_into_index(value, min_value);
        if index_count_vector_result.is_err() {
            return Err(CountingSortError::from_try_into_index_failed());
        } else {
            // index_count_vector_result is ok, unwrapping is safe
            let index_count_vector = index_count_vector_result.unwrap_or(0);
            if index_count_vector >= count_vector.len() {
                return Err(CountingSortError::from_index_out_of_bounds());
            }
            //
            /*
              Get the cumulative frequency of the value before this.
              The cumulative frequency of the preceeding value is the index of
              the first element with this value.

              In order to avoid checks for the index to be 0 (and therefore
              not to try to access the -1-th element) we allocated the 0-the
              element additionally so that we can now safely access it.
              Additionally it holds the index of the next element which
              equals the minimum value.
            */
            let mut index = count_vector[index_count_vector];
            sorted_vector[index] = *value;
            /*
              Increment the index so that successive elements with the same value
              do not override this one.
              This additionally ensures that the sort is stable.
              This actually increments the cumulative frequency of the preceeding
              value. However at the end of the sorting process this frequency will
              be the cumulative frequency of this value.
            */
            index += 1;
            count_vector[index_count_vector] = index;
        }
    }
    Ok(sorted_vector)
}

#[inline]
fn count_values<'a, ITER, T>(
    iterator: &mut ITER,
    min_value: &T,
    max_value: &T,
) -> Result<Vec<usize>, CountingSortError>
where
    ITER: Iterator<Item = &'a T>,
    T: Ord + Copy + TryIntoIndex + 'a,
{
    let distance_result = T::try_into_index(max_value, min_value);
    if distance_result.is_ok() {
        /*
          Length must hold all possible distinct values of the collection,
          this means the complete distance + 1. E.g. the distance of 0 and 255
          is 255, but there 256 distinct values between 0 and 255.

          We allocate another value in our vector to represent the value that preceeds
          the minimum value. This value actually does not exist in the collection but
          is introduced as an optimization for enabling the stable sort of this algorithm
          without the need of a DoubleEndedIterator and the reverse iteration of the
          collection when the given collection is re-ordered.
        */
        let length = distance_result.unwrap_or(0) + 2; // distance_result is okay so unwrapping is safe
        let mut count_vector: Vec<usize> = vec![0; length];

        for value in iterator {
            let index_result = T::try_into_index(value, min_value);
            if index_result.is_err() {
                return Err(CountingSortError::from_try_into_index_failed());
            } else {
                /*
                  Always add + 1 to not use the 0-the element in the vector.
                  This element is just allocated to optimize the re-ordering
                  of the given collection later on.
                  The 0-the element does in a way represent the value that preceeds
                  the minimum value, i.e. this value does not exist in the given
                  collection.
                */
                let index = index_result.unwrap_or(0) + 1; // index_result is ok, unwrapping is safe
                if index >= count_vector.len() {
                    return Err(CountingSortError::from_index_out_of_bounds());
                }
                let new_count_value = count_vector[index] + 1;
                count_vector[index] = new_count_value;
            }
        }
        return Ok(count_vector);
    }
    Err(CountingSortError::from_try_into_index_failed())
}

#[inline]
fn calculate_prefix_sum(count_vector: &mut Vec<usize>) {
    let mut iterator = count_vector.iter_mut();
    // skip first element
    let optional_first_element = iterator.next();
    if let Some(first_element) = optional_first_element {
        let mut total = *first_element;
        for value in iterator {
            total += *value;
            *value = total;
        }
    }
}

#[inline]
fn get_min_max<T, ITER>(iterator: &mut ITER) -> Option<(T, T)>
where
    T: Ord + Copy,
    ITER: Iterator<Item = T>,
{
    // consume first element to initialize as min and max value
    let min_value_optional = iterator.next();
    if let Some(min_value) = min_value_optional {
        let tuple = iterator.fold((min_value, min_value), |(min_val, max_val), value| {
            (min(min_val, value), max(max_val, value))
        });
        return Some(tuple);
    }
    None
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
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

    const TEST_COUNT_VALUES_ARRAY: [usize; 31] = [
        0, 1, 1, 3, 1, 1, 1, 3, 0, 1, 1, 2, 0, 1, 1, 1, 1, 1, 0, 0, 0, 2, 0, 1, 1, 1, 1, 1, 2, 0, 1,
    ];

    const TEST_PREFIX_SUM_ARRAY: [usize; 31] = [
        0, 1, 2, 5, 6, 7, 8, 11, 11, 12, 13, 15, 15, 16, 17, 18, 19, 20, 20, 20, 20, 22, 22, 23,
        24, 25, 26, 27, 29, 29, 30,
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
    fn test_min_value_larger_max_value_error() {
        let test_vector = vec![1];
        let result = counting_sort_min_max(test_vector.iter(), &1, &0);
        assert!(result.is_err());
        assert_eq!(
            "Minimum value is larger than maximum value",
            format!("{}", result.unwrap_err())
        );
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
        let test_vector: Vec<u8> = vec![];
        let result = counting_sort_min_max(test_vector.iter(), &0, &1);
        assert!(result.is_ok());
        assert_eq!(test_vector, result.unwrap());
    }

    #[test]
    fn test_incorrect_given_min_max_values() {
        let vec = vec![4, 3, 2, 1];

        let error = vec.iter().cnt_sort_min_max(&2, &4);
        assert!(error.is_err());
        assert_eq!(
            "Conversion into index failed",
            format!("{}", error.unwrap_err())
        );

        let error = vec.iter().cnt_sort_min_max(&1, &3);
        assert!(error.is_err());
        assert_eq!(
            "Index is out of bounds, most likely the given maximum value is too small",
            format!("{}", error.unwrap_err())
        );
    }

    #[test]
    fn test_try_into_error() {
        #[derive(Ord, PartialOrd, PartialEq, Eq, Copy, Clone, Debug)]
        struct ValueWithTryIntoError {
            value: u8,
        };

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
        assert_eq!(
            CountingSortError::from_try_into_index_failed().to_string(),
            result.unwrap_err().to_string()
        );

        let mut count_vector = vec![0, 0];
        let test_vector = vec![max_value, min_value];
        let result = re_order(test_vector.iter(), &mut count_vector, 2, &min_value);
        assert!(result.is_err());
        assert_eq!(
            CountingSortError::from_try_into_index_failed().to_string(),
            result.unwrap_err().to_string()
        );
    }

    #[test]
    fn test_empty_count_values_vector_is_impossible() {
        #[derive(Ord, PartialOrd, PartialEq, Eq, Copy, Clone, Debug)]
        struct ValueWithWrongSubstraction {
            value: usize,
        };

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

    #[test]
    fn test_re_order_index_out_of_bounds_error() {
        let vec = vec![1, 2];
        let mut count_vector = vec![1];
        let result = re_order(vec.iter(), &mut count_vector, 2, &1);
        assert!(result.is_err());
        assert_eq!(
            CountingSortError::from_index_out_of_bounds().to_string(),
            result.unwrap_err().to_string()
        );
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(doctest)]
macro_rules! doc_check {
    ($x:expr) => {
        #[doc = $x]
        extern {}
    };
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(doctest)]
doc_check!(include_str!("../README.md"));
