pub fn selection_sort<T: Ord>(vec: &mut Vec<T>) {
    for i in 0..vec.len() {
        let mut min = i;
        for j in (i + 1)..vec.len() {
            if vec[j] < vec[min] {
                min = j;
            }
        }
        vec.swap(i, min);
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use self::test::{Bencher, black_box};
    use super::super::test_util::*;

    sorting_tests!(selection_sort);
}
