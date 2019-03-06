use std::time::{Duration, SystemTime};

type Seconds = f32;
pub fn to_seconds(duration: Duration) -> Seconds {
    duration.as_secs() as Seconds + duration.subsec_nanos() as Seconds * 1e-9
}

macro_rules! race_set {
    ($name: ident, $type: ident) => {
        fn $name(n: u32) {
            use std::collections::$type;

            // setup
            let mut s: $type<_> = (0..n).collect();
            assert_eq!(s.len(), n as usize);
            let mut total: usize = 0;

            // measure
            let sys_time = SystemTime::now();
            while let Some(elt) = s.iter().next().cloned() {
                s.remove(&elt);
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
            println!("Shrinking {:8} size={:4}k: {:.3}s", stringify!($type), n / 1000, secs);
        }
    };
}

race_set!(pop_hashset, HashSet);
race_set!(pop_btreeset, BTreeSet);

fn main() {
    debug_assert!(false, "Run with --release for meaningful measurements");
    pop_btreeset(100_000);
    pop_btreeset(1_000_000);
    pop_btreeset(2_000_000);
    pop_btreeset(3_000_000);
    pop_btreeset(4_000_000);
    pop_hashset(100_000);
}
