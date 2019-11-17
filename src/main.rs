#![feature(map_first_last)]

pub mod stats;
use stats::SampleStatistics;

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::io::{stdout, Write};
use std::time::{Duration, SystemTime};

type Seconds = f64;
pub fn to_seconds(duration: Duration) -> Seconds {
    duration.as_secs() as Seconds + duration.subsec_nanos() as Seconds * 1e-9
}

fn btree_remove_next(s: &mut BTreeSet<i32>) -> Option<i32> {
    let elt = s.iter().next().copied()?;
    s.remove(&elt);
    Some(elt)
}

fn btree_take_next(s: &mut BTreeSet<i32>) -> Option<i32> {
    let elt = s.iter().next().copied()?;
    s.take(&elt)
}

fn btree_take_next_back(s: &mut BTreeSet<i32>) -> Option<i32> {
    let elt = s.iter().next_back().copied()?;
    s.take(&elt)
}

fn btree_take_first(s: &mut BTreeSet<i32>) -> Option<i32> {
    let elt = s.first().copied();
    elt.and_then(|e| s.take(&e))
}

fn btree_pop_first(s: &mut BTreeSet<i32>) -> Option<i32> {
    s.pop_first()
}

fn btree_pop_last(s: &mut BTreeSet<i32>) -> Option<i32> {
    s.pop_last()
}

fn hash_take_next(s: &mut HashSet<i32>) -> Option<i32> {
    let elt = s.iter().next().copied()?;
    s.take(&elt)
}

fn hash_remove_next(s: &mut HashSet<i32>) -> Option<i32> {
    let elt = s.iter().next().copied()?;
    s.remove(&elt);
    Some(elt)
}

fn hash_retain(s: &mut HashSet<i32>) -> Option<i32> {
    let mut snatched: Option<i32> = None;
    s.retain(|elt| match snatched {
        None => {
            snatched = Some(*elt);
            false
        }
        Some(_) => true,
    });
    snatched
}

type Stats = BTreeMap<(&'static str, i32), SampleStatistics>;

macro_rules! bench_set {
    ($name: ident, $pop: ident, $type: ident) => {
        fn $name(stats: &mut Stats, n: i32) {
            // setup
            let mut s: $type<_> = (0..n).collect();
            let mut total: usize = 0;

            // measure
            let sys_time = SystemTime::now();
            while let Some(elt) = $pop(&mut s) {
                total += elt as usize;
            }
            let secs = match sys_time.elapsed() {
                Ok(duration) => to_seconds(duration),
                Err(err) => {
                    eprintln!("Could not get time ({})", err);
                    -99.9
                }
            };
            assert_eq!(s.len(), 0);
            assert_eq!(total, n as usize * (n - 1) as usize / 2);
            let key = (stringify!($name), n);
            stats.entry(key).or_insert(Default::default()).put(secs);
            /*
            println!(
                "{:20} size={:4}k: {:.3}s",
                stringify!($name),
                n / 1000,
                secs,
            );
            */
        }
    };
}

bench_set!(bench_btree_remove_next, btree_remove_next, BTreeSet);
bench_set!(bench_btree_take_next, btree_take_next, BTreeSet);
bench_set!(bench_btree_take_next_back, btree_take_next_back, BTreeSet);
bench_set!(bench_btree_take_first, btree_take_first, BTreeSet);
bench_set!(bench_btree_pop_first, btree_pop_first, BTreeSet);
bench_set!(bench_btree_pop_last, btree_pop_last, BTreeSet);
bench_set!(bench_hash_remove_next, hash_remove_next, HashSet);
bench_set!(bench_hash_take_next, hash_take_next, HashSet);
bench_set!(bench_hash_retain, hash_retain, HashSet);

fn main() {
    debug_assert!(false, "Run with --release for meaningful measurements");
    let mut stats: Stats = Default::default();
    for i in (1..=5).rev() {
        print!("{}", i);
        stdout().flush().expect("Unable to flush stdout");
        for n in (20_000..200_000).step_by(20_000) {
            bench_btree_remove_next(&mut stats, n);
            bench_btree_take_next(&mut stats, n);
            bench_btree_take_next_back(&mut stats, n);
            bench_btree_take_first(&mut stats, n);
            bench_btree_pop_first(&mut stats, n);
            bench_btree_pop_last(&mut stats, n);
            bench_hash_remove_next(&mut stats, n);
            bench_hash_take_next(&mut stats, n);
            if n <= 50_000 {
                bench_hash_retain(&mut stats, n);
            }
        }
        for n in (200_000..1_000_000).step_by(100_000) {
            bench_btree_remove_next(&mut stats, n);
            bench_btree_take_next(&mut stats, n);
            bench_btree_take_next_back(&mut stats, n);
            bench_btree_take_first(&mut stats, n);
            bench_btree_pop_first(&mut stats, n);
            bench_btree_pop_last(&mut stats, n);
        }
        for n in (1_000_000..=5_000_000).step_by(1_000_000) {
            bench_btree_remove_next(&mut stats, n);
            bench_btree_take_next(&mut stats, n);
            bench_btree_take_next_back(&mut stats, n);
            bench_btree_take_first(&mut stats, n);
            bench_btree_pop_first(&mut stats, n);
            bench_btree_pop_last(&mut stats, n);
        }
    }
    println!(" done!");
    let mut prev_name = "";
    for ((name, n), stat) in stats {
        if prev_name != name {
            prev_name = name;
            println!("{}", name);
            println!("  {:>9} seconds", "size");
        }
        let mean = stat.mean();
        let dev = stat.deviation() / mean;
        println!("  {:>9} {:.3} ±{:3.0}%", n, mean, dev * 100.0);
    }
}
