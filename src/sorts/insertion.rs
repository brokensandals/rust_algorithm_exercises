pub fn insertion_sort<T: Ord>(vec: &mut Vec<T>) {
    for i in 1..(vec.len()) {
        let mut j = i;
        loop {
            if j <= 0 || vec[j - 1] <= vec[j] {
                break;
            }
            vec.swap(j - 1, j);
            j = j - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use self::test::{Bencher, black_box};
    use super::super::test_util::*;

    #[test]
    fn it_handles_empty_array() {
        let expected: Vec<i32> = vec![];
        let mut vec = expected.clone();
        insertion_sort(&mut vec);
        assert_eq!(expected, vec);
    }

    #[bench]
    fn bench_already_sorted(b: &mut Bencher) {
        let mut vec = (0..5000).collect();
        b.iter(|| {
            insertion_sort(&mut vec);
            black_box(&vec);
        });
        assert_sorted(&vec);
    }

    #[bench]
    fn bench_reverse_sorted(b: &mut Bencher) {
        let reverse_vec: Vec<i64> = (0..5000).rev().collect();
        b.iter(|| {
            let mut vec = reverse_vec.clone();
            insertion_sort(&mut vec);
            black_box(&vec);
            assert_sorted(&vec);
        });
    }

    #[bench]
    fn bench_sort_random(b: &mut Bencher) {
        let rand_vec = rand_vec();
        b.iter(|| {
            let mut vec = rand_vec.clone();
            insertion_sort(&mut vec);
            black_box(&vec);
            assert_sorted(&vec);
        });
    }
}
