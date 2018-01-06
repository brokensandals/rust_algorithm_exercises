pub fn bubble_sort<T: Ord>(vec: &mut Vec<T>) {
    for _ in 0..(vec.len()) {
        for j in 1..(vec.len()) {
            if vec[j - 1] > vec[j] {
                vec.swap(j - 1, j);
            }
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
        test_basic_properties(&bubble_sort);
    }

    #[bench]
    fn perf_sorted(b: &mut Bencher) {
        bench_already_sorted(b, &bubble_sort);
    }

    #[bench]
    fn perf_reverse_sorted(b: &mut Bencher) {
        bench_reverse_sorted(b, &bubble_sort);
    }

    #[bench]
    fn perf_random(b: &mut Bencher) {
        bench_sort_random(b, &bubble_sort);
    }
}
