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

fn hash_take_next(s: &mut HashSet<i32>) -> Option<i32> {
    let elt = s.iter().next().copied()?;
    s.take(&elt)
}

fn hash_remove_next(s: &mut HashSet<i32>) -> Option<i32> {
    let elt = s.iter().next().copied()?;
    s.remove(&elt);
    Some(elt)
}

type Stats = BTreeMap<(i32, &'static str), SampleStatistics>;

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
            let key = (n, stringify!($name));
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

bench_set!(bench_btree_take_next_back, btree_take_next_back, BTreeSet);
bench_set!(bench_btree_remove_next, btree_remove_next, BTreeSet);
bench_set!(bench_btree_take_next, btree_take_next, BTreeSet);
bench_set!(bench_hash_remove_next, hash_remove_next, HashSet);
bench_set!(bench_hash_take_next, hash_take_next, HashSet);

fn main() {
    debug_assert!(false, "Run with --release for meaningful measurements");
    let mut stats: Stats = Default::default();
    for i in (1..=5).rev() {
        print!("{}", i);
        stdout().flush().expect("Unable to flush stdout");
        for n in (20_000..=100_000).step_by(20_000) {
            bench_btree_remove_next(&mut stats, n);
            bench_btree_take_next(&mut stats, n);
            bench_btree_take_next_back(&mut stats, n);
            bench_hash_remove_next(&mut stats, n);
            bench_hash_take_next(&mut stats, n);
        }
        for n in (1_000_000..=5_000_000).step_by(1_000_000) {
            bench_btree_remove_next(&mut stats, n);
            bench_btree_take_next(&mut stats, n);
            bench_btree_take_next_back(&mut stats, n);
        }
    }
    println!(" done!");
    let mut prev_n = 0;
    for ((n, name), stat) in stats {
        if prev_n != n {
            prev_n = n;
            if n < 1_000_000 {
                println!("Size {}k", n / 1_000);
            } else {
                println!("Size {}M", n / 1_000_000);
            };
        }
        let mean = stat.mean();
        let dev = stat.deviation() / mean;
        println!("  {:26}: {:.3}s ±{:3.0}%", name, mean, dev * 100.0);
    }
}
