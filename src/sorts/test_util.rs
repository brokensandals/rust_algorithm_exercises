extern crate rand;
extern crate test;

use std::fmt::Display;

pub fn rand_vec(size: usize) -> Vec<i64> {
    let mut rand_vec = Vec::with_capacity(size);
    for _ in 0..size {
        rand_vec.push(rand::random::<i64>());
    }
    return rand_vec;
}

pub fn assert_sorted<T: Ord + Display>(vec: &Vec<T>) {
    for i in 1..(vec.len()) {
        assert!(vec[i - 1] <= vec[i], "not sorted, {} not less than {}", vec[i - 1], vec[i]); // TODO provide useful output
    }
}

#[macro_export]
macro_rules! sorting_tests {
    ( $f:ident ) => {
        #[test]
        fn it_handles_empty_vec() {
            let mut vec: Vec<i64> = vec![];
            $f(&mut vec);
            assert!(vec.is_empty());
        }

        #[test]
        fn it_handles_single_element() {
            let mut vec = vec![1i64];
            $f(&mut vec);
            assert_sorted(&vec);
        }

        #[test]
        fn it_handles_sorted_input() {
            let mut vec = (0i64..1000).collect();
            $f(&mut vec);
            assert_sorted(&vec);
        }

        #[test]
        fn it_handles_reverse_sorted_input() {
            let mut vec = (0i64..1000).rev().collect();
            $f(&mut vec);
            assert_sorted(&vec);
        }

        #[test]
        fn it_sorts_even_length_input() {
            let mut vec = rand_vec(1000);
            $f(&mut vec);
            assert_sorted(&vec);
        }

        #[test]
        fn it_sorts_odd_length_input() {
            let mut vec = rand_vec(1001);
            $f(&mut vec);
            assert_sorted(&vec);
        }

        #[bench]
        fn bench_sort_sorted(b: &mut Bencher) {
            b.iter(|| {
                let mut vec = (0i64..5000).collect();
                $f(&mut vec);
                black_box(&vec);
            });
        }

        #[bench]
        fn bench_sort_reverse_sorted(b: &mut Bencher) {
            b.iter(|| {
                let mut vec = (0i64..5000).rev().collect();
                $f(&mut vec);
                black_box(&vec);
            });
        }

        #[bench]
        fn bench_sort_random(b: &mut Bencher) {
            let original = rand_vec(5000);

            b.iter(|| {
                let mut vec = original.clone();
                $f(&mut vec);
                black_box(&vec);
            });
        }
    }
}
