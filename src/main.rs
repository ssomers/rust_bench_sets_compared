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
    let elt = s.iter().next().cloned()?;
    s.remove(&elt);
    Some(elt)
}

fn btree_take_next(s: &mut BTreeSet<i32>) -> Option<i32> {
    let elt = s.iter().next().cloned()?;
    s.take(&elt)
}

fn btree_take_next_back(s: &mut BTreeSet<i32>) -> Option<i32> {
    let elt = s.iter().next_back().cloned()?;
    s.take(&elt)
}

fn hash_take_next(s: &mut HashSet<i32>) -> Option<i32> {
    let elt = s.iter().next().cloned()?;
    s.take(&elt)
}

fn hash_remove_next(s: &mut HashSet<i32>) -> Option<i32> {
    let elt = s.iter().next().cloned()?;
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

bench_set!(bench_btree_back, btree_take_next_back, BTreeSet);
bench_set!(bench_btree_remove, btree_remove_next, BTreeSet);
bench_set!(bench_btree_take, btree_take_next, BTreeSet);
bench_set!(bench_hash_remove, hash_remove_next, HashSet);
bench_set!(bench_hash_take, hash_take_next, HashSet);

fn main() {
    debug_assert!(false, "Run with --release for meaningful measurements");
    let mut stats: Stats = Default::default();
    for i in (1..=5).rev() {
        print!("{}", i);
        stdout().flush().expect("Unable to flush stdout");
        bench_btree_remove(&mut stats, 100_000);
        bench_btree_take(&mut stats, 100_000);
        bench_btree_back(&mut stats, 100_000);
        bench_hash_remove(&mut stats, 100_000);
        bench_hash_take(&mut stats, 100_000);
        for n in (1_000_000..=5_000_000).step_by(1_000_000) {
            bench_btree_remove(&mut stats, n);
            bench_btree_take(&mut stats, n);
            bench_btree_back(&mut stats, n);
        }
    }
    println!(" done!");
    for ((n, name), stat) in stats {
        let mean = stat.mean();
        let dev = stat.deviation() / mean;
        println!(
            "{:20} size={:4}k: {:.3}s ±{:3.0}%",
            name,
            n / 1000,
            mean,
            dev * 100.0,
        );
    }
}
