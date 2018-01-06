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
    use self::test::Bencher;
    use super::super::test_util::*;

    #[test]
    fn it_sorts_correctly() {
        test_basic_properties(&insertion_sort);
    }

    #[bench]
    fn perf_sorted(b: &mut Bencher) {
        bench_already_sorted(b, &insertion_sort);
    }

    #[bench]
    fn perf_reverse_sorted(b: &mut Bencher) {
        bench_reverse_sorted(b, &insertion_sort);
    }

    #[bench]
    fn perf_random(b: &mut Bencher) {
        bench_sort_random(b, &insertion_sort);
    }
}
