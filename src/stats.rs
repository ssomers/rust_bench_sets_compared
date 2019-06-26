#[derive(Clone, Default)]
pub struct SampleStatistics {
    max: f64,
    min: f64,
    samples: u32,
    sum: f64,
    sum_of_squares: f64,
}

impl SampleStatistics {
    pub fn put(&mut self, v: f64) {
        if self.samples == 0 {
            self.min.clone_from(&v);
            self.max = v;
        } else if self.min > v {
            self.min = v;
        } else if self.max < v {
            self.max = v;
        }
        self.samples += 1;
        self.sum += v;
        self.sum_of_squares += v.powi(2);
    }

    pub fn max(&self) -> f64 {
        self.max.clone()
    }
    pub fn min(&self) -> f64 {
        self.min.clone()
    }
    pub fn mean(&self) -> f64 {
        if self.samples < 1 {
            std::f64::NAN
        } else if self.min == self.max {
            self.min
        } else {
            self.sum / f64::from(self.samples)
        }
    }
    pub fn variance(&self) -> f64 {
        if self.samples < 2 {
            std::f64::NAN
        } else if self.min == self.max {
            0.
        } else {
            let n = f64::from(self.samples);
            // may become slightly negative because of rounding:
            (self.sum_of_squares - self.sum.powi(2) / n).max(0.) / (n - 1.)
        }
    }
    pub fn deviation(&self) -> f64 {
        self.variance().sqrt()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;

    #[test]
    fn stats_0() {
        let s: SampleStatistics = Default::default();
        assert!(s.mean().is_nan());
        assert!(s.variance().is_nan());
        assert!(s.deviation().is_nan());
    }

    #[test]
    fn stats_1() {
        let mut s: SampleStatistics = Default::default();
        s.put(-1.0);
        assert_eq!(s.mean(), -1.0);
        assert!(s.variance().is_nan());
        assert!(s.deviation().is_nan());
    }

    #[test]
    fn stats_2() {
        let mut s: SampleStatistics = Default::default();
        s.put(-1.0);
        s.put(1.0);
        assert_eq!(s.mean(), 0.0);
        assert_eq!(s.variance(), 2.0);
        assert_eq!(s.deviation(), 2.0_f64.sqrt());
    }

    #[test]
    fn stats_3() {
        let mut s: SampleStatistics = Default::default();
        s.put(89.0);
        s.put(90.0);
        s.put(91.0);
        assert_eq!(s.mean(), 90.0);
        assert_eq!(s.variance(), 1.0);
        assert_eq!(s.deviation(), 1.0);
    }

    #[test]
    fn stats_9() {
        let mut s: SampleStatistics = Default::default();
        s.put(2.0);
        s.put(4.0);
        s.put(4.0);
        s.put(4.0);
        s.put(5.0);
        s.put(5.0);
        s.put(5.0);
        s.put(7.0);
        s.put(9.0);
        assert_eq!(s.mean(), 5.0);
        assert_eq!(s.variance(), 4.0);
        assert_eq!(s.deviation(), 2.0);
    }
}

#[cfg(test)]
mod proptests {
    extern crate proptest;
    use self::proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        fn put_1(x in proptest::num::f64::NORMAL) {
            let mut s: SampleStatistics = Default::default();
            s.put(x);
            assert!(s.mean() >= s.min());
            assert!(s.mean() <= s.max());
        }

        #[test]
        fn put_2(x in proptest::num::f64::NORMAL, y in proptest::num::f64::NORMAL) {
            let mut s: SampleStatistics = Default::default();
            s.put(x);
            s.put(y);
            assert!(s.mean() >= s.min());
            assert!(s.mean() <= s.max());
            assert!(s.variance() >= 0.);
            assert!(s.deviation() <= (s.max() - s.min()) * 1.5);
        }

        #[test]
        fn put_n(i in 2..99, x in proptest::num::f64::NORMAL) {
            let mut s: SampleStatistics = Default::default();
            for _ in 0..i {
                s.put(x);
            }
            assert!(s.mean() >= s.min());
            assert!(s.mean() <= s.max());
            assert!(s.variance() >= 0.);
            assert!(s.deviation() <= (s.max() - s.min()));
        }
    }
}
