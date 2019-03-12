#[derive(Clone, Default)]
pub struct SampleStatistics {
    max: f64,
    min: f64,
    samples: isize,
    sum: f64,
    sum_of_squares: f64,
}

impl SampleStatistics {
    pub fn put(&mut self, v: f64) {
        if self.samples == 0 {
            self.min = v.clone();
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
        if self.samples > 0 {
            self.sum / self.samples as f64
        } else {
            std::f64::NAN
        }
    }
    pub fn variance(&self) -> f64 {
        if self.samples > 1 {
            let n = self.samples as f64;
            let m = self.sum / n;
            (self.sum_of_squares - 2.0 * m * self.sum + m.powi(2) * n) / (n - 1.0)
        } else {
            std::f64::NAN
        }
    }
    pub fn deviation(&self) -> f64 {
        self.variance().sqrt()
    }
}

#[cfg(test)]
mod tests {
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
