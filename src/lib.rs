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
    //fn counting_sort_known_min_max(& mut self, known_min_value:T, known_max_value:T);
}

impl CountingSort<u8> for Vec<u8> {
    fn counting_sort(&mut self){
        let optional_tuple = get_min_max_iter_opt(& mut self.iter());
            if optional_tuple.is_some() {
            let (min_value,max_value) = optional_tuple.unwrap();

            let mut count_vector = count_values_iter(& mut self.iter(), &min_value, &max_value);

            prefix_sum(&mut count_vector);

            let sorted_vector = re_order_iter(self.iter(), & mut count_vector, self.len(), min_value);

            *self = sorted_vector;
        }
    }

    //fn counting_sort_known_min_max(&mut self, known_min_value: u8, known_max_value: u8){
    //}
}

pub fn counting_sort<'a,ITER,T>(iterator:& mut ITER) -> Vec<T>
    where ITER: DoubleEndedIterator<Item=&'a T> + Clone,
             T: Ord + Copy + Into<usize> + Sub<Output=T> + 'a
{
    let optional_tuple = get_min_max_iter_opt(& mut iterator.clone());
    if optional_tuple.is_some() {
        let (min_value, max_value) = optional_tuple.unwrap();
        let mut count_vector = count_values_iter(& mut iterator.clone(), min_value, max_value);
        println!("{:?}", count_vector);
        prefix_sum(& mut count_vector);
        // last element of the count vector depicts the index-1 of the largest element, hence it is its length
        let length = count_vector.last();
        if length.is_some() {
            let length = *length.unwrap();
            println!("length={}", length);
            let sorted_vector = re_order_iter(iterator, & mut count_vector, length, &min_value);
            return sorted_vector;
        }
    }
    vec![]
}

//fn re_order<T>(vector:&Vec<T>, count_vector:&mut Vec<usize>, length:usize, min_value:&T)-> Vec<T>
//    where T:Ord+Copy+Into<usize>+Sub<Output=T>
//{
//    let mut sorted_vector:Vec<T> = vec![*min_value; length];
//    for value in vector.iter().rev() {
//        let index_count_vector = T::into(*value - *min_value);
//        println!("index_count_vector: {}", index_count_vector);
//        let mut index =  count_vector[index_count_vector as usize];
//        index -= 1;
//        count_vector[index_count_vector as usize] = index;
//        sorted_vector[index as usize] = *value;
//    }
//    sorted_vector
//}

fn re_order_iter<'a,T,ITER>(iterator:ITER, count_vector:&mut Vec<usize>, length:usize, min_value:&T)-> Vec<T>
    where T:Ord+Copy+Into<usize>+Sub<Output=T>+'a,
          ITER:DoubleEndedIterator<Item=&'a T>
{
    let mut sorted_vector:Vec<T> = vec![*min_value; length];
    for value in iterator.rev() {
        let index_count_vector = T::into(*value - *min_value);
        let mut index =  count_vector[index_count_vector as usize];
        index -= 1;
        count_vector[index_count_vector] = index;
        sorted_vector[index] = *value;
    }
    sorted_vector
}

fn prefix_sum<T>(count_vector:&mut Vec<T>)
    where T:Copy+Add<Output=T>
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

//fn count_values<T>(unsorted_vector:&Vec<T> /* impl Iterator? */, min_value:&T, max_value:&T) -> Vec<usize>
//    where T: Ord+Copy+Into<usize>+Sub<Output=T>
//{
//    let (min_value,max_value) = get_min_max_iter(unsorted_vector.iter(), &min_value, &max_value);
//
//    let offset = T::into(*min_value);
//    let length:usize = T::into(*max_value - *min_value) + 1;
//    let mut count_vector:Vec<usize> = vec![0;length];
//
//    for value in unsorted_vector {
//        let index = T::into(*value) - offset;
//        let new_count_value = count_vector[index] + 1;
//        count_vector[index] = new_count_value;
//    }
//
//    count_vector
//}

fn count_values_iter<'a,ITER,T>(iterator:& mut ITER, min_value:&T, max_value:&T) -> Vec<usize>
    where   ITER: Iterator<Item=&'a T>,
            T: Ord+Copy+Into<usize>+Sub<Output=T>+'a
{

    let offset = T::into(*min_value);
    let length:usize = T::into(*max_value - *min_value) + 1;
    let mut count_vector:Vec<usize> = vec![0;length];

    for value in iterator {
        let index = T::into(*value) - offset;
        let new_count_value = count_vector[index] + 1;
        count_vector[index] = new_count_value;
    }

    count_vector
}

//fn get_min_max<T:Ord+Copy>(slice:&[T], min_value:&T, max_value:&T)-> (T,T)
//{
//    slice.iter().fold( (*max_value, *min_value), |(min_val,max_val),value|{
//        (min(min_val,*value),max(max_val,*value))
//    })
//}

//fn get_min_max_iter<T,ITER>(iterator:ITER, min_value:&T, max_value:&T)-> (T,T)
//    where T:Ord+Copy,
//          ITER:Iterator<Item=T>
//{
//    iterator.fold( (*max_value, *min_value), |(min_val,max_val),value|{
//        (min(min_val,value),max(max_val,value))
//    })
//}

fn get_min_max_iter_opt<T,ITER>(iterator:& mut ITER)-> Option<(T,T)>
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
        let sorted_vector = counting_sort(& mut test_vector.iter());
        assert_eq!(vec![1,2,3,4], sorted_vector);
    }

    #[test]
    fn test_for_i8_iter() {
        let test_vector:Vec<i8> = vec![2,-2,1,-6];
        let sorted_vector = vec![];
        let i:i8 = -15;
        //let sorted_vector = counting_sort(& mut test_vector.iter());
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
    fn test_unsigned_get_min_max() {
        let test_vector:Vec<u8> = vec![1,2,3,4];
        let tuple = get_min_max_iter_opt(& mut test_vector.iter());
        assert!(tuple.is_some());
        let (min_value,max_value) = tuple.unwrap();
        assert_eq!(1,*min_value);
        assert_eq!(4,*max_value);
    }

    #[test]
    fn test_signed_get_min_max() {
        let test_vector:Vec<i8> = vec![-128,2,3,127];
        let tuple = get_min_max_iter_opt(& mut test_vector.iter());
        assert!(tuple.is_some());
        let (min_value,max_value) = tuple.unwrap();
        assert_eq!(-128,*min_value);
        assert_eq!(127,*max_value);
    }

    #[test]
    fn test_prefix_sum_1() {
        let mut test_vector:Vec<u8> = vec![1;4];
        prefix_sum(&mut test_vector);
        assert_eq!(vec![1,2,3,4], test_vector);
    }

    #[test]
    fn test_prefix_sum_2() {
        let mut test_vector:Vec<u8> = vec![1,2,3,4,5];
        prefix_sum(&mut test_vector);
        assert_eq!(vec![1,3,6,10,15], test_vector);
    }

}
