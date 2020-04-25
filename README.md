# Counting Sort

A counting sort implementation for [`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html)s.

## Usage

Add dependency to your `Cargo.toml`:

```toml
[dependencies]
counting_sort = "1.0.0"
```

Works immediately "out of the box" for e.g. [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)s holding integers like [`u8`](https://doc.rust-lang.org/std/primitive.u8.html), [`u16`](https://doc.rust-lang.org/std/primitive.u16.html), [`i8`](https://doc.rust-lang.org/std/primitive.i8.html), [`i16`](https://doc.rust-lang.org/std/primitive.i16.html) etc..

```rust
/*
 * Add counting sort to your source code.
 */
use counting_sort::CountingSort;

let vec = vec![2,4,1,3];

// counting sort may fail, therefore a result is returned
let sorted_vec_result = vec.iter().cnt_sort();
assert!(sorted_vec_result.is_ok());

// if successful sorted elements were copied into a Vec
assert_eq!(vec![1,2,3,4], sorted_vec_result.unwrap());
```

## Design goals

1. Learn more Rust on a simple algorithm
2. As much quality as possible, therefore
    * High code coverage
    * "Long" code comments
    * A lot of units & integration tests
3. As generic as possible considering iterators and integers
    * I wanted an interface which is not limited to slices or vectors
    * I wanted to support as much integer types as reasonable
4. A usable interface: "just" call `cnt_sort` on your `Vec`, `LinkedList` etc.
5. Slight memory consumption optimization
    * The count values vector, which is needed for the histogram of all used values in the collection, does only allocate the maximum amount of memory absolute necessary and not more
    * That's the reason why I calculate the minimum and maximum value of use the given parameters in `cnt_sort_min_max`
    * The idea is to support counting sort algorithm for [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) and [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) without allocating `2³²-1` [`usize`](https://doc.rust-lang.org/std/primitive.usize.html) integers if the distance `d = max_value - min_value` is smaller than that.
6. Safety over performance
    * E.g. I'll check that no index is out of bounds, although this should only happen when a user uses the `cnt_sort_min_max` method with a too small maximum value and Rust panics when the index is out of bounds

## Asymptotic performance

1. Iterates all `n` elements and checks if this value is the new minimum value or maximum value
2. Allocates the count values vector of size `d = max_value - min_value` (i.e. the distance `d`)
3. Iterates all `n` elements again to create the histogram of each value
4. Iterates all `d` elements of the count values vector to calculate the prefix sum
5. Allocates a new vector for holding the sorted elements
6. Iterates all `n` elements back to front to re-order the elements into a new vector

Therefore the asymptotic performance is `O(3n+d)`. When using the `cnt_sort_min_max` function (when the minimum and maximum value is known) then the asymptotic performance is `O(2n+d)`.

## Benchmarks

## Code coverage

```console
[INFO tarpaulin] Coverage Results:
|| Uncovered Lines:
|| src/lib.rs: 118
|| Tested/Total Lines:
|| src/lib.rs: 83/84 +0%
||
98.81% coverage, 83/84 lines covered, +0% change in coverage
```

## License

## Todos

1. Performance table / diagram
2. Profile
3. Optimizations
   * Combine slide window and re_order into one step?
   * Drain the iterator on count_values, this means trait bound DoubleEndedIterator can be lifted
      * Is primarily needed for keeping the original order (is the order important?)
      * If iterator is not traversed back to front, then the elements are sorted in reverse order, this is strange
      * Draining the iterator will "destroy" the original collection which is devastating when an error happens
      * However, the elements could be swapped instead of copied into the new Vec
   * Copy elements into vector may result in less copies of the element
   * currently 2-3 copies per element due to TryInto
   * T:Clone instead of T copy?
4. Publish? License?
