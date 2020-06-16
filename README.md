# Counting Sort

A counting sort implementation for [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)s.

## Usage

Add dependency to your `Cargo.toml`:

```toml
[dependencies]
counting_sort = "1.0.3"
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

## Release Notes

* 1.0.5
  * Changed SVG links to github repository
* 1.0.4
  * SVG links are broken on crates.io after renaming of default branch
* 1.0.3
  * Fixed some lint findings found by [clippy](https://github.com/rust-lang/rust-clippy)
  * Finding types:
    * [assign_op_pattern](https://rust-lang.github.io/rust-clippy/master/index.html#assign_op_pattern)
    * [unnecessary unwrap](https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_unwrap)
    * [needless_return](https://rust-lang.github.io/rust-clippy/master/index.html#needless_return)
* 1.0.2
  * Updated `Readme.md`, changed `license-file` to `license`
* 1.0.1
  * Added `keywords`, `categories` and `license-file` to `Cargo.toml`

## Code coverage

```console
[INFO tarpaulin] Coverage Results:
|| Uncovered Lines:
|| src/lib.rs: 105
|| Tested/Total Lines:
|| src/lib.rs: 81/82
||
98.78% coverage, 81/82 lines covered
```

## License

[MIT license](LICENSE).

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
7. Not destroying the original collection:
    * The implementation can fail, especially during the conversion into an index
    * Therefore the elements are not moved out of the original collection but copied during iteration

## Stable counting sort

* Counting sort is a stable algorithm
* One option to achieve this is to reverse iterate the collection (and hence the need for a [DoubleEndedIterator](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html))
* Another option is possible when all elements are an integer, and it combines two loops, see a short description [here](https://en.wikipedia.org/wiki/Counting_sort#Variant_algorithms)
  * Since there is no Integer trait, this option was not possible if the implementation is aimed to be as generic as possible (without implementing everything as a macro)
* In this alogorithm another option is implemented:

1. In the count values vector (or actually the cumulative frequency) an additional element is allocated, representing the value which preceeds the minimum value
    * This element obviously does not exist and is only used in the re-ordering phase
    * The element is allocated with value 0
2. During the calculation of the histogram of all existing values (or more precisely their mapped value to an index) this 0-th element is never accessed
3. During the calculation of the cumulative frequency (or prefix sum) this element is used, but its value is 0, so it does not change anything
4. During the re-ordering of the given collection (the last phase of the algorithm) a trick is applied
    1. When re-ordering each element, it does actually use the cumulative ferquency of the preceeding element to calculate the position of the element in the sorted output vector
    2. This is due to fact, that an additional element is allocated at the "beginning" of the count values vector (or cumulative frequency vector)
    3. In order to understand this, it makes sense to look at [this](https://www.cs.usfca.edu/~galles/visualization/CountingSort.html) pretty nice visualisation of the counting sort algorithm
        * During the re-ordering, an element from the "back" of the collection is "taken"
        * This element is then converted to an index (in the above visualization the element is identical to the index)
        * With this index the cumulative frequency of the element is used to calculate the position of the element in the sorted output vector
        * This calculation is done by decrementing the cumulative frequency by one
        * In this way, the fact that first element of a vector is the 0-the element is considered
        * Additionally it is needed to achieve the stability of the sort, elements keep their order even after the sort
        * When each of the elements, which are equivalent, i.e. they are in an equivalence class, were put into the output vector, the (potentially multiple times decremented) cumulative frequency of this element equals the unmodified cumulative frequency of the preceeding element
        * And this frequency is actually the position of the first element of the above mentioned equivalence class
        * Therefore it is possible to use the cumulative frequency of the preceeding element to calculate the position of the element in the output sorted vector
        * Obviously it is necessary to increment the cumulative frequency of preceeding element, each time an element is put into the sorted output vector, in order to avoid overriding equivalent elements
        * Final note: the additional allocated element of the cumulative frequency does represent the index of the first minimum value of the collection: zero

## Asymptotic performance

1. Iterates all `n` elements and checks if this value is the new minimum value or maximum value
2. Allocates the count values vector of size `d = max_value - min_value` (i.e. the distance `d`)
3. Iterates all `n` elements again to create the histogram of each value
4. Iterates all `d` elements of the count values vector to calculate the prefix sum
5. Allocates a new vector for holding the sorted elements
6. Iterates all `n` elements back to front to re-order the elements into a new vector

Therefore the asymptotic performance is `O(3n+d)`. When using the `cnt_sort_min_max` function (when the minimum and maximum value is known) then the asymptotic performance is `O(2n+d)`.

## Benchmarks

* Comparison to [slice.sort](https://doc.rust-lang.org/std/primitive.slice.html#method.sort) and [count_sort](https://crates.io/crates/count_sort)

### HW

```console
Architecture:                    x86_64
CPU op-mode(s):                  32-bit, 64-bit
Byte Order:                      Little Endian
Address sizes:                   36 bits physical, 48 bits virtual
CPU(s):                          4
On-line CPU(s) list:             0-3
Thread(s) per core:              2
Core(s) per socket:              2
Socket(s):                       1
NUMA node(s):                    1
Vendor ID:                       GenuineIntel
CPU family:                      6
Model:                           42
Model name:                      Intel(R) Core(TM) i5-2410M CPU @ 2.30GHz
Stepping:                        7
CPU MHz:                         1721.799
CPU max MHz:                     2900,0000
CPU min MHz:                     800,0000
BogoMIPS:                        4591.83
Virtualization:                  VT-x
L1d cache:                       64 KiB
L1i cache:                       64 KiB
L2 cache:                        512 KiB
L3 cache:                        3 MiB
NUMA node0 CPU(s):               0-3
Vulnerability Itlb multihit:     KVM: Mitigation: Split huge pages
Vulnerability L1tf:              Mitigation; PTE Inversion; VMX conditional cache flushes, SMT vulnerable
Vulnerability Mds:               Mitigation; Clear CPU buffers; SMT vulnerable
Vulnerability Meltdown:          Mitigation; PTI
Vulnerability Spec store bypass: Mitigation; Speculative Store Bypass disabled via prctl and seccomp
Vulnerability Spectre v1:        Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:        Mitigation; Full generic retpoline, IBPB conditional, IBRS_FW, STIBP conditional, RSB filling
Vulnerability Tsx async abort:   Not affected
Flags:                           fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ht tm pbe syscall nx rdtscp l
                                 m constant_tsc arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf pni pclmulqdq dtes64 monitor ds_cpl vmx est tm
                                 2 ssse3 cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic popcnt tsc_deadline_timer aes xsave avx lahf_lm epb pti ssbd ibrs ibpb stibp tpr_shadow
                                  vnmi flexpriority ept vpid xsaveopt dtherm ida arat pln pts md_clear flush_l1d
```

### sorting u8

* Average execution time
* Distance: 256
  * minimum value 0
  * maximum value 256

|# elements|cnt_sort|cnt_sort_min_max|vector.sort|sort_u8|
|---------:|-------:|---------------:|----------:|------:|
|     20000|   82 us|           63 us|    1048 us|  27 us|
|     40000|  161 us|          123 us|    2239 us|  55 us|
|     60000|  244 us|          185 us|    3513 us|  82 us|
|     80000|  323 us|          248 us|    4761 us| 109 us|
|    100000|  406 us|          310 us|    6180 us| 137 us|

![Lines u8](https://github.com/rusty-octopus/counting_sort/blob/trunk/lines_u8.svg)

### sorting u16

* Average execution time
* Distance: 512
  * minimum value  512
  * maximum value 1024
  * This is an ideal solution for this counting sort implementation

|# elements|cnt_sort|cnt_sort_min_max|vector.sort|sort_u16|
|---------:|-------:|---------------:|----------:|-------:|
|     20000|   89 us|           73 us|    1051 us|   95 us|
|     40000|  188 us|          172 us|    2250 us|  122 us|
|     60000|  318 us|          229 us|    3513 us|  148 us|
|     80000|  382 us|          355 us|    4785 us|  174 us|
|    100000|  477 us|          392 us|    6200 us|  205 us|

![Lines u16](https://github.com/rusty-octopus/counting_sort/blob/trunk/lines_u16.svg)
