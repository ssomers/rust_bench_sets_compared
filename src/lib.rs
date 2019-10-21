#![feature(test)]
extern crate fnv;
extern crate hashbrown;
extern crate test;
#[cfg(test)]
use fnv::FnvHashSet;
#[cfg(test)]
use hashbrown::HashSet as HashbrownSet;
#[cfg(test)]
use std::collections::BTreeSet;
#[cfg(test)]
use std::collections::HashSet;

macro_rules! bench_set {
    ($name: ident, $type: ident, $size: expr) => {
        #[bench]
        pub fn $name(b: &mut test::Bencher) {
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

bench_set!(peek_btreeset_1, BTreeSet, 1);
bench_set!(peek_btreeset_100, BTreeSet, 100);
bench_set!(peek_btreeset_10k, BTreeSet, 10_000);
bench_set!(peek_btreeset_1kk, BTreeSet, 1_000_000);

bench_set!(peek_hashset_1, HashSet, 1);
bench_set!(peek_hashset_100, HashSet, 100);
bench_set!(peek_hashset_10k, HashSet, 10_000);
bench_set!(peek_hashset_1kk, HashSet, 1_000_000);

bench_set!(peek_fnvhashtreeset_1, FnvHashSet, 1);
bench_set!(peek_fnvhashtreeset_100, FnvHashSet, 100);
bench_set!(peek_fnvhashtreeset_10k, FnvHashSet, 10_000);
bench_set!(peek_fnvhashtreeset_1kk, FnvHashSet, 1_000_000);

bench_set!(peek_hashbrownset_1, HashbrownSet, 1);
bench_set!(peek_hashbrownset_100, HashbrownSet, 100);
bench_set!(peek_hashbrownset_10k, HashbrownSet, 10_000);
bench_set!(peek_hashbrownset_1kk, HashbrownSet, 1_000_000);
