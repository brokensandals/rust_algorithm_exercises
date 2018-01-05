mod bubble;
mod insertion;

#[cfg(test)]
mod test_util {
    extern crate rand;

    use std::fmt::Display;

    pub fn rand_vec() -> Vec<i64> {
        let mut rand_vec = Vec::with_capacity(5000);
        for _ in 0..5000 {
            rand_vec.push(rand::random::<i64>());
        }
        return rand_vec;
    }

    pub fn assert_sorted<T: Ord + Display>(vec: &Vec<T>) {
        for i in 1..(vec.len()) {
            assert!(vec[i - 1] <= vec[i], "not sorted, {} not less than {}", vec[i - 1], vec[i]); // TODO provide useful output
        }
    }
}
