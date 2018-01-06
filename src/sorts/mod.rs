pub mod bubble;
pub mod insertion;

#[cfg(test)]
mod test_util {
    extern crate rand;
    extern crate test;

    use std::fmt::Display;
    use self::test::{Bencher, black_box};

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

    pub fn test_basic_properties<F>(sort: &F)
        where F: 'static + Fn(&mut Vec<i64>) {

        {
            let mut vec: Vec<i64> = vec![];
            sort(&mut vec);
            assert!(vec.is_empty());
        }

        {
            let mut vec = vec![1i64];
            sort(&mut vec);
            assert_sorted(&vec);
        }

        {
            let mut vec = (0i64..1000).collect();
            sort(&mut vec);
            assert_sorted(&vec);
        }

        {
            let mut vec = (0i64..1000).rev().collect();
            sort(&mut vec);
            assert_sorted(&vec);
        }

        {
            let mut vec = rand_vec(1000);
            sort(&mut vec);
            assert_sorted(&vec);
        }
    }

    pub fn bench_already_sorted<F>(b: &mut Bencher, sort: &F)
        where F: 'static + Fn(&mut Vec<i64>) {

        b.iter(|| {
            let mut vec = (0i64..5000).collect();
            sort(&mut vec);
            black_box(&vec);
        });
    }

    pub fn bench_reverse_sorted<F>(b: &mut Bencher, sort: &F)
        where F: 'static + Fn(&mut Vec<i64>) {

        b.iter(|| {
            let mut vec = (0i64..5000).rev().collect();
            sort(&mut vec);
            black_box(&vec);
        });
    }

    pub fn bench_sort_random<F>(b: &mut Bencher, sort: &F)
        where F: 'static + Fn(&mut Vec<i64>) {

        let original = rand_vec(5000);

        b.iter(|| {
            let mut vec = original.clone();
            sort(&mut vec);
            black_box(&vec);
        });
    }
}
