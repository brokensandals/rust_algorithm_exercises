extern crate rand;
extern crate test;

use self::rand::Rng;

pub fn rand_vec(size: usize) -> Vec<i64> {
    let mut rand_vec = Vec::with_capacity(size);
    for _ in 0..size {
        rand_vec.push(rand::random::<i64>());
    }
    return rand_vec;
}

pub fn rand_order(size: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..size).collect();
    rand::thread_rng().shuffle(&mut vec);
    vec
}

#[macro_export]
macro_rules! dictionary_tests {
    ( $t:ident ) => {
        use self::test::{Bencher, black_box};

        #[test]
        fn it_returns_none_when_searching_for_missing_elements() {
            let mut dict: $t<i64, i64> = $t::new();
            assert!(dict.search(2).value().is_none());
            dict.search(1).set_value(100);
            assert!(dict.search(2).value().is_none());
        }

        #[test]
        fn it_inserts_and_searches() {
            let keys = rand_vec(1000);
            let vals = rand_vec(1000);
            let mut dict: $t<i64, i64> = $t::new();
            for i in 0..1000 {
                let mut cursor = dict.search(keys[i]);
                cursor.set_value(vals[i]);
                assert_eq!(Some(vals[i]), cursor.value().cloned());
            }
            let indexes = rand_order(1000);
            for i in indexes {
                assert_eq!(Some(vals[i]), dict.search(keys[i]).value().cloned());
            }
        }

        #[test]
        fn it_deletes() {
            let mut dict: $t<i64, i64> = $t::new();
            dict.search(1).set_value(100);
            assert_eq!(Some(100), dict.search(1).value().cloned());
            dict.search(1).delete();
            assert_eq!(None, dict.search(1).value());
        }

        #[test]
        fn it_returns_min_and_max_elements() {
            let mut dict: $t<i64, i64> = $t::new();
            assert!(dict.min().is_none());
            assert!(dict.max().is_none());
            dict.search(2).set_value(200);
            assert!(dict.min().is_some());
            assert!(dict.max().is_some());
            assert_eq!(Some(200), dict.min().unwrap().value().cloned());
            assert_eq!(Some(200), dict.max().unwrap().value().cloned());
            dict.search(1).set_value(100);
            dict.search(3).set_value(300);
            assert!(dict.min().is_some());
            assert!(dict.max().is_some());
            assert_eq!(Some(100), dict.min().unwrap().value().cloned());
            assert_eq!(Some(300), dict.max().unwrap().value().cloned());
        }

        #[test]
        fn it_returns_predecessors() {
            let mut dict: $t<i64, i64> = $t::new();
            dict.search(2).set_value(200);
            assert!(dict.search(2).predecessor().is_none());
            dict.search(1).set_value(100);
            assert!(dict.search(2).predecessor().is_some());
            assert_eq!(Some(100), dict.search(2).predecessor().unwrap().value().cloned());
        }

        #[test]
        fn it_returns_successors() {
            let mut dict: $t<i64, i64> = $t::new();
            dict.search(2).set_value(200);
            assert!(dict.search(2).successor().is_none());
            dict.search(3).set_value(300);
            assert!(dict.search(2).successor().is_some());
            assert_eq!(Some(300), dict.search(2).successor().unwrap().value().cloned());
        }

        #[test]
        fn it_traverses_from_min() {
            let keys = rand_vec(1000);
            let vals = rand_vec(1000);
            let mut dict: $t<i64, i64> = $t::new();
            for i in 0..1000 {
                dict.search(keys[i]).set_value(vals[i]);
            }

            {
                let mut cursor = dict.min().unwrap();
                let mut prev = *cursor.key();
                for _ in 0..999 {
                    cursor = cursor.successor().unwrap();
                    let cur = *cursor.key();
                    assert!(prev < cur);
                    prev = cur;
                }
                assert!(cursor.successor().is_none());
            }
        }

        #[test]
        fn it_traverses_from_max() {
            let keys = rand_vec(1000);
            let vals = rand_vec(1000);
            let mut dict: $t<i64, i64> = $t::new();
            for i in 0..1000 {
                dict.search(keys[i]).set_value(vals[i]);
            }

            {
                let mut cursor = dict.max().unwrap();
                let mut prev = *cursor.key();
                for _ in 0..999 {
                    cursor = cursor.predecessor().unwrap();
                    let cur = *cursor.key();
                    assert!(prev > cur);
                    prev = cur;
                }
                assert!(cursor.predecessor().is_none());
            }
        }

        #[bench]
        fn bench_insert(b: &mut Bencher) {
            let keys = rand_vec(5000);
            let vals = rand_vec(5000);
            b.iter(|| {
                let mut dict: $t<i64, i64> = $t::new();
                for i in 0..5000 {
                    dict.search(keys[i]).set_value(vals[i]);
                }
                black_box(dict);
            });
        }

        #[bench]
        fn bench_search(b: &mut Bencher) {
            let keys = rand_vec(5000);
            let vals = rand_vec(5000);
            let mut dict: $t<i64, i64> = $t::new();
            for i in 0..5000 {
                dict.search(keys[i]).set_value(vals[i]);
            }

            let indexes = &rand_order(5000)[0..1000];

            b.iter(|| {
                for i in indexes {
                    black_box(dict.search(keys[*i]).value());
                }
            });
        }

        #[bench]
        fn bench_traverse_from_min(b: &mut Bencher) {
            let keys = rand_vec(5000);
            let vals = rand_vec(5000);
            let mut dict: $t<i64, i64> = $t::new();
            for i in 0..5000 {
                dict.search(keys[i]).set_value(vals[i]);
            }

            b.iter(|| {
                let mut cursor = dict.min().unwrap();
                loop {
                    black_box(&cursor.value().unwrap());
                    match cursor.successor() {
                        None => break,
                        Some(next) => { cursor = next },
                    };
                }
            });
        }

        #[bench]
        fn bench_traverse_from_max(b: &mut Bencher) {
            let keys = rand_vec(5000);
            let vals = rand_vec(5000);
            let mut dict: $t<i64, i64> = $t::new();
            for i in 0..5000 {
                dict.search(keys[i]).set_value(vals[i]);
            }

            b.iter(|| {
                let mut cursor = dict.max().unwrap();
                loop {
                    black_box(&cursor.value().unwrap());
                    match cursor.predecessor() {
                        None => break,
                        Some(next) => { cursor = next },
                    };
                }
            });
        }

        #[bench]
        fn bench_delete(b: &mut Bencher) {
            let keys = rand_vec(5000);
            let vals = rand_vec(5000);
            let mut dict: $t<i64, i64> = $t::new();
            for i in 0..5000 {
                dict.search(keys[i]).set_value(vals[i]);
            }

            let indexes = &rand_order(5000)[0..1000];

            b.iter(|| {
                for i in indexes {
                    dict.search(keys[*i]).delete();
                }
                black_box(&dict);
            });
        }
    }
}
