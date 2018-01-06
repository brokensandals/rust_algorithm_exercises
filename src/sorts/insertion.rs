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

    sorting_tests!(insertion_sort);
}
