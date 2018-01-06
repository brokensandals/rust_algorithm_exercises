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
    use self::test::{Bencher, black_box};
    use super::super::test_util::*;

    sorting_tests!(bubble_sort);
}
