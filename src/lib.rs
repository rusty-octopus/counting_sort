// https://www.cs.usfca.edu/~galles/visualization/CountingSort.html
// https://en.wikipedia.org/wiki/Counting_sort

// ideas:
// * counting_sort for usigned and signed
//   * how to "divide"? how to share code
// * Macros?
//   * index type?
// * test errors / uncovered lines
// * tests for signed unsigned u8/16/u32/u64 etc.
// * Tests for DoubleEndedIterator other than Vec
// * Use benchmarking?
//   * time measurement for performance evaluation vs. std lib
// * todo enable analysis method <= should be called inspect & InspectionResult
//   * consider memory size!!! can you avoid allocating "gaps"?
// * docs + docs test
//   * How to use it for a non integer


use std::cmp::{min,max,Ord};
use std::ops::Sub;
use std::convert::TryInto;
use std::num::TryFromIntError;

pub trait ToUsize {
    fn to(self) -> Option<usize>;
}

//impl ToUsize for u8 {
//    fn to(self) -> Option<usize> {
//        
//    }
//}

pub trait CountingSort<T:TryInto<usize>> 
{
    // searches for the min and max value independent from T::max_value()/min_value()
    fn counting_sort(&mut self)-> Result<(),<T as std::convert::TryInto<usize>>::Error>;
    //fn counting_sort_known_min_max(& mut self, known_min_value:T, known_max_value:T);
}

impl CountingSort<u8> for Vec<u8> {
    fn counting_sort(&mut self) -> Result<(),<u8 as std::convert::TryInto<usize>>::Error>{
        let optional_tuple = get_min_max(& mut self.iter());
            if optional_tuple.is_some() {
            let (min_value,max_value) = optional_tuple.unwrap();

            let mut count_vector = count_values(& mut self.iter(), &min_value, &max_value)?;

            calculate_prefix_sum(&mut count_vector);

            let sorted_vector = re_order(self.iter(), & mut count_vector, self.len(), min_value)?;

            *self = sorted_vector;
        }
        Ok(())
    }

    //fn counting_sort_known_min_max(&mut self, known_min_value: u8, known_max_value: u8){
    //}
}

//pub enum MemorySize {
//    BYTES,
//    KILO_BTYES,
//    MEGA_BYTES,
//    GIGA_BYTES,
//    TERA_BYTES
//}
//
//pub enum Feasibility {
//    YES,
//    TOO_MUCH_MEMORY(u32, MemorySize),
//    TOO_MUCH_EFFORT
//}

pub struct InspectionResult<T> 
    where T: Ord + Copy
{
    pub min_value: T,
    pub max_value: T,
    pub range: usize,
    pub asymptotic_effort: u64,
    pub overflow_possible: bool
}

impl<'a,T> InspectionResult<T>
    where T: Ord + Copy + 'a
{
    fn inspect_signed<ITER>(iterator: ITER) -> Option<InspectionResult<T>> 
        where ITER: DoubleEndedIterator<Item=&'a T> + Clone
    {
        let min_max_tuple = get_min_max(& mut iterator.clone());
        if min_max_tuple.is_some() {
            let (min_value,max_value) = min_max_tuple.unwrap();
            //let opt_range = get_range_signed(&min_value, &max_value);
            Some(InspectionResult{
                min_value: min_value.clone(),
                max_value: max_value.clone(),
                range: 0,
                asymptotic_effort: 0,
                overflow_possible: false
            })
        } else {
            None
        }
    }
}

//pub trait IntoIndex<T> {
//    fn into(value:&T) -> usize {
//        i8::into::<usize>(-8)
//    }
//}

pub fn counting_sort<'a,ITER,T>(iterator:& mut ITER) -> Result<Vec<T>,<T as std::convert::TryInto<usize>>::Error>
    where ITER: DoubleEndedIterator<Item=&'a T> + Clone,
             T: Ord + Copy + TryInto<usize> + Sub<Output=T> + 'a
{
    let optional_tuple = get_min_max(& mut iterator.clone());
    if optional_tuple.is_some() {
        let (min_value, max_value) = optional_tuple.unwrap();
        counting_sort_known_min_max(iterator, min_value, max_value)
    } else {
        Ok(vec![])
    }
}

pub fn counting_sort_known_min_max<'a,ITER,T>(iterator:& mut ITER, min_value:&T, max_value:&T) -> Result<Vec<T>,<T as std::convert::TryInto<usize>>::Error>
    where ITER: DoubleEndedIterator<Item=&'a T> + Clone,
             T: Ord + Copy + TryInto<usize> + Sub<Output=T> + 'a
{
    let mut count_vector = count_values(& mut iterator.clone(), min_value, max_value)?;
    calculate_prefix_sum(& mut count_vector);
    // last element of the count vector depicts the index-1 of the largest element, hence it is its length
    let length = count_vector.last();
    if length.is_some() {
        let length = *length.unwrap();
        let sorted_vector = re_order(iterator, & mut count_vector, length, &min_value);
        return sorted_vector;
    }
    Ok(vec![])
}

macro_rules! get_usize {
    ($T:ty,$int_type:ty,$a:ident,$b:ident) => {{
        let a:$int_type = <$T>::into(*$a);
        let b:$int_type = <$T>::into(*$b);
        if a >= b {
            (a - b) as usize
        } else {
            (b - a) as usize
        }
    }};
}

fn blub() -> usize {
    i8::try_into(-4).unwrap()
}

fn get_usize_signed<T>(a:&T,b:&T) -> usize 
    where T: Into<isize>+Copy
{
    get_usize!(T,isize,a,b)
}

fn get_usize_unsigned<T>(a:&T,b:&T) -> usize 
    where T: Into<usize>+Copy+Ord
{
    get_usize!(T,usize,a,b)
}

fn re_order<'a,T,ITER>(iterator:ITER, count_vector:&mut Vec<usize>, length:usize, min_value:&T)-> Result<Vec<T>,<T as std::convert::TryInto<usize>>::Error>
    where T:Ord+Copy+TryInto<usize>+Sub<Output=T>+'a,
          ITER:DoubleEndedIterator<Item=&'a T>
{ //<Type as std::convert::TryInto>::Error
  //<usize as std::convert::TryInto>::Error
    let mut sorted_vector:Vec<T> = vec![*min_value; length];
    for value in iterator.rev() {
        let index_count_vector = T::try_into(*value - *min_value)?;
        let mut index =  count_vector[index_count_vector];
        index -= 1;
        count_vector[index_count_vector] = index;
        sorted_vector[index] = *value;
    }
    Ok(sorted_vector)
}

fn count_values<'a,ITER,T>(iterator:& mut ITER, min_value:&T, max_value:&T) -> Result<Vec<usize>,<T as std::convert::TryInto<usize>>::Error>
    where   ITER: Iterator<Item=&'a T>,
            T: Ord+Copy+TryInto<usize>+Sub<Output=T>+'a
{
    // max_value - min_value should always be >= "0"
    // however it could overflow isize
    let length = T::try_into(*max_value - *min_value)? + 1;
    let mut count_vector:Vec<usize> = vec![0;length];

    for value in iterator {
        let index = T::try_into(*value - *min_value)?;
        let new_count_value = count_vector[index] + 1;
        count_vector[index] = new_count_value;
    }

    Ok(count_vector)
}

fn calculate_prefix_sum(count_vector:&mut Vec<usize>)
{
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

fn get_range_signed_checked<T:Into<isize>+Copy>(min_value:&T,max_value:&T) -> Option<isize> {
    T::into(*max_value).checked_sub(T::into(*min_value))
}

fn get_range_unsigned<T:Into<isize>+Copy>(min_value:&T,max_value:&T) -> Option<isize> {
    T::into(*max_value).checked_sub(T::into(*min_value))
}

//fn get_value_range<T,INT_TYPE>(min_value:&T, max_value:&T) -> usize 
//    where T: Into<INT_TYPE>,
//        INT_TYPE: Sub<Output=INT_TYPE>
//{
//    let range = T::into(*max_value) - T::into(*min_value);
//    usize::from(range)
//}

fn get_min_max<T,ITER>(iterator:& mut ITER)-> Option<(T,T)>
    where T:Ord+Copy,
          ITER:Iterator<Item=T>
{
    // consume first element to initialize as min and max value
    let min_value = iterator.next();
    if min_value.is_some() {
        let min_value = min_value.unwrap();
        let tuple =  iterator.fold( (min_value, min_value), |(min_val,max_val),value|{
            (min(min_val,value),max(max_val,value))
        });
        return Some(tuple)
    }
    None

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_for_u8() {
        let mut test_vector:Vec<u8> = vec![4,3,2,1];
        test_vector.counting_sort();
        assert_eq!(vec![1,2,3,4], test_vector);
    }

    #[test]
    fn test_for_u8_iter() {
        let test_vector:Vec<u8> = vec![4,3,2,1];
        let sorted_vector = counting_sort(& mut test_vector.iter()).unwrap();
        assert_eq!(vec![1,2,3,4], sorted_vector);
    }

    #[test]
    fn test_for_u16_iter() {
        let test_vector:Vec<u16> = vec![4,3,2,1];
        //let sorted_vector = vec![];// = 
        let sorted_vector = counting_sort(& mut test_vector.iter()).unwrap();
        assert_eq!(vec![1,2,3,4], sorted_vector);
    }

    #[test]
    fn test_get_usize_signed() {
        assert_eq!(0xFFFF, get_usize_signed::<i16>(&i16::min_value(),&i16::max_value()));
    }

    #[test]
    fn test_for_i8_iter() {
        let test_vector:Vec<i8> = vec![2,-2,1,-6];
        let sorted_vector = counting_sort(& mut test_vector.iter()).unwrap();
        assert_eq!(vec![-6,-2,1,2], sorted_vector);
    }

    #[test]
    fn test_counting_sort() {
        let mut test_vector = 
            vec![13, 24, 27, 3, 10, 1, 9, 17, 6, 7, 3, 30, 14, 15, 2, 3, 7, 11, 21, 16, 7, 11, 21, 5, 23, 25, 26, 28, 28, 4];
        test_vector.counting_sort();
        let sorted_vector = 
            vec![1, 2, 3, 3, 3, 4, 5, 6, 7, 7, 7, 9, 10, 11, 11, 13, 14, 15, 16, 17, 21, 21, 23, 24, 25, 26, 27, 28, 28, 30];

        assert_eq!(sorted_vector, test_vector);
    }

    #[test]
    fn test_counting_sort_iter() {
        let test_vector:Vec<u8> = 
            vec![13, 24, 27, 3, 10, 1, 9, 17, 6, 7, 3, 30, 14, 15, 2, 3, 7, 11, 21, 16, 7, 11, 21, 5, 23, 25, 26, 28, 28, 4];
        let sorted_vector = counting_sort(& mut test_vector.iter()).unwrap();
        let expected_vector = 
            vec![1, 2, 3, 3, 3, 4, 5, 6, 7, 7, 7, 9, 10, 11, 11, 13, 14, 15, 16, 17, 21, 21, 23, 24, 25, 26, 27, 28, 28, 30];

        assert_eq!(expected_vector, sorted_vector);
    }

    #[test]
    fn test_unsigned_get_min_max() {
        let test_vector:Vec<u8> = vec![1,2,3,4];
        let tuple = get_min_max(& mut test_vector.iter());
        assert!(tuple.is_some());
        let (min_value,max_value) = tuple.unwrap();
        assert_eq!(1,*min_value);
        assert_eq!(4,*max_value);
    }

    #[test]
    fn test_signed_get_min_max() {
        let test_vector:Vec<i8> = vec![-128,2,3,127];
        let tuple = get_min_max(& mut test_vector.iter());
        assert!(tuple.is_some());
        let (min_value,max_value) = tuple.unwrap();
        assert_eq!(-128,*min_value);
        assert_eq!(127,*max_value);
    }

    #[test]
    fn test_prefix_sum_1() {
        let mut test_vector:Vec<usize> = vec![1;4];
        calculate_prefix_sum(&mut test_vector);
        assert_eq!(vec![1,2,3,4], test_vector);
    }

    #[test]
    fn test_prefix_sum_2() {
        let mut test_vector:Vec<usize> = vec![1,2,3,4,5];
        calculate_prefix_sum(&mut test_vector);
        assert_eq!(vec![1,3,6,10,15], test_vector);
    }

}
