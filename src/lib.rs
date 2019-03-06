#![feature(test)]
extern crate test;

macro_rules! bench_set {
    ($name: ident, $type: ident, $size: expr) => {
        #[bench]
        pub fn $name(b: &mut test::Bencher) {
            use std::collections::$type;

            // setup
            let s: $type<i32> = (0..$size).collect();
            assert_eq!(s.len(), $size);

            // measure
            b.iter(|| {
                let set = test::black_box(&s);
                let elt = *set.iter().next().unwrap();
                test::black_box(elt);
            })
        }
    };
}

bench_set!(peek_hashset_1, HashSet, 1);
bench_set!(peek_hashset_100, HashSet, 100);
bench_set!(peek_hashset_10k, HashSet, 10_000);
bench_set!(peek_hashset_1kk, HashSet, 1_000_000);
bench_set!(peek_btreeset_1, BTreeSet, 1);
bench_set!(peek_btreeset_100, BTreeSet, 100);
bench_set!(peek_btreeset_10k, BTreeSet, 10_000);
bench_set!(peek_btreeset_1kk, BTreeSet, 1_000_000);
