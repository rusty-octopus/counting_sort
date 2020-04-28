#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod integration_tests {

    use counting_sort::CountingSort;

    use std::collections::LinkedList;

    use oorandom::Rand32;

    use core::convert::TryFrom;

    use std::fmt::Display;

    use std::collections::HashSet;

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

    trait Absolute {
        fn absolute(&self) -> Self;
    }

    macro_rules! impl_abs_for_int {
        ($int_type:ty) => {
            impl Absolute for $int_type {
                fn absolute(&self) -> Self {
                    self.abs()
                }
            }
        };
    }

    impl_abs_for_int!(i32);

    impl Absolute for u32 {
        fn absolute(&self) -> Self {
            *self
        }
    }

    fn create_test_vector<INT, RANDINT, F>(
        number_of_elements: usize,
        min_value: INT,
        max_value: INT,
        mut f: F,
    ) -> Vec<INT>
    where
        INT: Ord + Display + TryFrom<RANDINT> + Into<RANDINT> + Copy,
        F: FnMut(&mut Rand32) -> RANDINT,
        RANDINT: Copy
            + core::ops::Rem<Output = RANDINT>
            + core::ops::Add<Output = RANDINT>
            + Display
            + core::ops::Sub<Output = RANDINT>
            + Absolute,
    {
        if min_value > max_value {
            panic!(
                "min value {} must be smaller than max_value {}",
                min_value, max_value
            );
        }
        let mut vec = Vec::with_capacity(number_of_elements);
        let mut rng = Rand32::new(7648730752358173238);
        let max_range = INT::into(max_value) - INT::into(min_value);
        let min_val = INT::into(min_value);
        for _ in 0..number_of_elements {
            let value = (f(&mut rng) % max_range).absolute() + min_val;
            let random_integer = INT::try_from(value);
            if random_integer.is_ok() {
                vec.push(random_integer.unwrap_or(min_value));
            } else {
                panic!("Could not convert {}", value);
            }
        }
        vec
    }

    fn create_test_vector_unsigned<INT: Ord + Display + TryFrom<u32> + Into<u32> + Copy>(
        number_of_elements: usize,
        min_value: INT,
        max_value: INT,
    ) -> Vec<INT> {
        let rand_u32 = |rng: &mut Rand32| rng.rand_u32();
        create_test_vector(number_of_elements, min_value, max_value, rand_u32)
    }

    fn create_test_vector_signed<INT: Ord + Display + TryFrom<i32> + Into<i32> + Copy>(
        number_of_elements: usize,
        min_value: INT,
        max_value: INT,
    ) -> Vec<INT> {
        let rand_i32 = |rng: &mut Rand32| rng.rand_i32();
        create_test_vector(number_of_elements, min_value, max_value, rand_i32)
    }

    #[test]
    fn test_with_vector_u8_10k() {
        let number_of_elements = 10000;
        let mut vector = create_test_vector_unsigned::<u8>(number_of_elements, 0, 255);
        let result = vector.iter().cnt_sort_min_max(&0, &255);
        assert!(result.is_ok());
        vector.sort();
        assert_eq!(vector, result.unwrap());
    }

    #[test]
    fn test_with_vector_i8_10k() {
        let number_of_elements = 10000;
        let mut vector = create_test_vector_signed::<i8>(number_of_elements, -128, 127);
        let result = vector.iter().cnt_sort();
        assert!(result.is_ok());
        vector.sort();
        assert_eq!(vector, result.unwrap());
    }

    #[test]
    fn test_with_vector_u16_10k() {
        let number_of_elements = 10000;
        let mut vector = create_test_vector_unsigned::<u16>(number_of_elements, 0, 0xFFFF);
        let result = vector.iter().cnt_sort();
        assert!(result.is_ok());
        vector.sort();
        assert_eq!(vector, result.unwrap());
    }

    #[test]
    fn test_with_vector_i16_10k() {
        let number_of_elements = 10000;
        let mut vector = create_test_vector_signed::<i16>(
            number_of_elements,
            i16::min_value(),
            i16::max_value(),
        );
        let result = vector.iter().cnt_sort();
        assert!(result.is_ok());
        vector.sort();
        assert_eq!(vector, result.unwrap());
    }

    #[test]
    fn test_with_vector_u32_10k() {
        let number_of_elements = 10000;
        let mut vector = create_test_vector_unsigned::<u32>(
            number_of_elements,
            u32::from(u16::min_value()) + 1000,
            u32::from(u16::max_value()) + 10000,
        );
        let result = vector.iter().cnt_sort();
        assert!(result.is_ok());
        vector.sort();
        assert_eq!(vector, result.unwrap());
    }

    #[test]
    fn test_with_vector_i32_10k() {
        let number_of_elements = 10000;
        let mut vector = create_test_vector_signed::<i32>(
            number_of_elements,
            i32::from(i16::min_value()) - 1000,
            i32::from(i16::max_value()) + 1000,
        );
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

    #[test]
    fn test_stable_sort() {
        use core::cmp::{Ord, Ordering};
        use counting_sort::TryIntoIndex;

        #[derive(Copy, Clone, Debug)]
        struct Person {
            name: &'static str,
            id: usize,
        }

        impl Ord for Person {
            fn cmp(&self, other: &Self) -> Ordering {
                self.id.cmp(&other.id)
            }
        }

        impl PartialOrd for Person {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for Person {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl Eq for Person {}

        impl TryIntoIndex for Person {
            type Error = &'static str;

            fn try_into_index(value: &Self, min_value: &Self) -> Result<usize, Self::Error> {
                Ok(value.id - min_value.id)
            }
        }

        let first = Person {
            name: "first",
            id: 3,
        };
        let second = Person {
            name: "second",
            id: 1,
        };
        let third = Person {
            name: "third",
            id: 1,
        };
        let fourth = Person {
            name: "fourth",
            id: 2,
        };

        let vec = vec![first, second, third, fourth];

        let sorted_vec = vec.iter().cnt_sort().unwrap();

        assert_eq!(vec![second, third, fourth, first], sorted_vec);
    }

    #[test]
    fn test_hash_set() {
        let mut set = HashSet::new();
        set.insert(4);
        set.insert(3);
        set.insert(2);
        set.insert(4);

        let result = set.iter().cnt_sort();

        assert!(result.is_ok());

        assert_eq!(vec![2,3,4], result.unwrap());

    }
}
