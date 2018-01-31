pub fn quicksort<T: Ord + Copy>(vec: &mut Vec<T>) {
    if vec.is_empty() {
        return;
    }

    let right = vec.len() - 1;
    quicksort_slice(vec, 0, right);
}

pub fn quicksort_slice<T: Ord + Copy>(vec: &mut Vec<T>, left: usize, right: usize) {
    if right <= left {
        return;
    }

    let pivot = vec[(left + right) / 2];
    let mut i = left;
    let mut j = right;
    loop {
        while vec[i] < pivot {
            i += 1;
        }
        while vec[j] > pivot {
            j -= 1;
        }
        if i >= j {
            break;
        }
        vec.swap(i, j);
    }

    let partition = j;
    
    if partition > left {
        quicksort_slice(vec, left, partition - 1);
    }

    quicksort_slice(vec, partition + 1, right);
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use self::test::{Bencher, black_box};
    use super::super::test_util::*;

    sorting_tests!(quicksort);
}
