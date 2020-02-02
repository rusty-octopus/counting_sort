// https://www.cs.usfca.edu/~galles/visualization/CountingSort.html
// https://en.wikipedia.org/wiki/Counting_sort

// ideas:
// * counting_sort with canonical mapping T -> Integer?
// * consider memory size!!! can you avoid allocating "gaps"?
// * no std?
// * helper tool to create randomized lists
// * time measurement for performance evaluation vs. std lib
// * "Default" implementation (e.g. only using iter) BUT
//      * Important Test: Can I import the trait and implement it for a Vec?
// * Use benchmarking?

use std::cmp::{min,max,Ord};
use std::ops::{Sub,Add};

pub trait CountingSort<T> {
    // searches for the min and max value independent from T::max_value()/min_value()
    fn counting_sort(&mut self);
    fn counting_sort_known_min_max(& mut self, known_min_value:T, known_max_value:T);
}

impl CountingSort<u8> for Vec<u8> {
    fn counting_sort(&mut self){
        let (min_value,max_value) = get_min_max(&self, &0, &255);

        let mut count_vector = count_values(&self, &min_value, &max_value, 0, 1);

    }

    fn counting_sort_known_min_max(&mut self, known_min_value: u8, known_max_value: u8){
    }
}

fn count_values<T>(unsorted_vector:&Vec<T> /* impl Iterator? */, min_value:&T, max_value:&T, zero:T, one:T) -> Vec<T>
    where T: Ord+Copy+Sub<Output=T>+Add<Output=T>+Into<usize>
{
    let (min_value,max_value) = get_min_max(unsorted_vector, min_value, max_value);

    let length:usize = T::into(one + max_value - min_value);
    let mut count_vector:Vec<<T as std::ops::Add>::Output> = vec![zero;length];
    let offset = min_value;

    for value in unsorted_vector {
        let index = T::into(*value - offset);
        let new_count_value = count_vector[index] + one;
        count_vector[index] = new_count_value;
    }

    count_vector
}

fn get_min_max<T:Ord+Copy>(slice:&[T], min_value:&T, max_value:&T)-> (T,T)
{
    slice.iter().fold( (*max_value, *min_value), |(min_val,max_val),value|{
        (min(min_val,*value),max(max_val,*value))
    })
}

fn get_min_max_iter<T,ITER>(iterator:ITER, min_value:&T, max_value:&T)-> (T,T)
    where T:Ord+Copy,
          ITER:Iterator<Item=T>
{
    iterator.fold( (*max_value, *min_value), |(min_val,max_val),value|{
        (min(min_val,value),max(max_val,value))
    })
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
    fn test_unsigned_get_min_max() {
        let test_vector:Vec<u8> = vec![1,2,3,4];
        let (min_value,max_value) = get_min_max(&test_vector, &0, &255);
        assert_eq!(1,min_value);
        assert_eq!(4,max_value);
    }

    #[test]
    fn test_signed_get_min_max() {
        let test_vector:Vec<i8> = vec![-128,2,3,127];
        let (min_value,max_value) = get_min_max(&test_vector, &-128, &127);
        assert_eq!(-128,min_value);
        assert_eq!(127,max_value);
    }

    #[test]
    fn test_unsigned_get_min_max_iter() {
        let test_vector:Vec<u8> = vec![1,2,3,4];
        let (min_value,max_value) = get_min_max_iter(test_vector.iter(), &&0, &&255);
        assert_eq!(1,*min_value);
        assert_eq!(4,*max_value);
    }
}
