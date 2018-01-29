// TODO - what are the alternatives to requiring the Copy trait?
pub fn merge_sort<T: Ord + Copy>(vec: &mut Vec<T>) {
    if vec.is_empty() {
        return;
    }

    let mut tmp = vec.clone();
    let right = vec.len() - 1;
    merge_sort_slice(vec, &mut tmp, 0, right);
}

fn merge_sort_slice<T: Ord + Copy>(vec: &mut Vec<T>, tmp: &mut Vec<T>, left: usize, right: usize) {
    if right <= left {
        return;
    }

    let middle = left + ((right - left) / 2);
    merge_sort_slice(vec, tmp, left, middle);
    merge_sort_slice(vec, tmp, middle + 1, right);
    
    for i in left..(right + 1) {
        tmp[i] = vec[i];
    }

    let mut target = left;
    let mut left_source = left;
    let mut right_source = middle + 1;
    while left_source <= middle && right_source <= right {
        if tmp[left_source] <= tmp[right_source] {
            vec[target] = tmp[left_source];
            left_source += 1;
        } else {
            vec[target] = tmp[right_source];
            right_source += 1;
        }
        target += 1;
    }
    while left_source <= middle {
        vec[target] = tmp[left_source];
        left_source += 1;
        target += 1;
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use self::test::{Bencher, black_box};
    use super::super::test_util::*;

    sorting_tests!(merge_sort);
}

