#[derive(Debug, Clone)]
pub struct RRSeries {
    rr_intervals: Vec<f64>,
    annotations: Vec<u8>,
    length: usize,
    quality_stats: QualityStats,
    time_length: f64,
    mean_rr: f64,
    sdnn: f64,
    sd1: f64,
    sd2: f64,
    pp: PoincarePlot,
    sd1a: f64,
    sd1d: f64,
    sd2a: f64,
    sd2d: f64,
    sdnn_a: f64,
    sdnn_d: f64,
}
#[derive(Debug, Clone, Default)] // the Default trait makes sure the starting values are all 0
pub struct QualityStats {
    n: usize, // normal
    v: usize, // ventricular
    s: usize, // supraventricular
    x: usize, // artifact
}

#[derive(Debug, Clone)]
pub struct PoincarePlot {
    xi: Vec<f64>,
    xii: Vec<f64>,
}

impl RRSeries {
    pub fn new(rr_intervals: Vec<f64>, annotations: Vec<u8>) -> Self {
        let length = rr_intervals.len();
        let quality_stats = get_quality_stats();
        return RRSeries {
            rr_intervals: rr_intervals,
            annotations: annotations,
            length: length,
            quality_stats: QualityStats,
            time_length: f64,
            mean_rr: f64,
            sdnn: f64,
            sd1: f64,
            sd2: f64,
            pp: PoincarePlot,
            sd1a: f64,
            sd1d: f64,
            sd2a: f64,
            sd2d: f64,
            sdnn_a: f64,
            sdnn_d: f64,
        };
    }
    fn get_quality_stats(self) -> QualityStats {
        let quality_stats: QualityStats;
        for i in 0..self.length - 1 {
            match self.annot(i) {
                0 => quality_stats.n = quality_stats.n + 1,
                1 => quality_stats.v = quality_stats.v + 1,
                2 => quality_stats.s = quality_stats.s + 1,
                3 => quality_stats.x = quality_stats.x + 1,
            }
        }
        quality_stats
    }
    fn form_pp(self) -> PoincarePlot {
        let idx: usize;
        let mut xi: Vec<f64>;
        let mut xii: Vec<f64>;
        for idx in 0..self.length {
            if (self.annotations[idx] == 0 & (self.annotations[idx + 1] == 0)) {
                xi.push(self.rr_intervals[idx]);
                xii.push(self.rr_intervals[idx + 1])
            }
        }
        return PoincarePlot { xi: xi, xii: xii };
    }
    fn mean_rr_full(self) -> f64 {
        // this is a regular mean from all RR's that are of sinusl origin
        let mut accumulator = 0.0;
        for i in 0..self.xi.len() as usize {
            accumulator = accumulator + self.xi[i];
        }
        accumulator = accumulator + self.xii[xi.len() - 1];
        return accumulator / (self.xi.len() + 1 as f64);
    }
    fn mean_rr_pp(self) -> f64 {
        // this is calculated from xi only
        let mut accumulator = 0.0;
        for i in 0..self.xi.len() as usize {
            accumulator = accumulator + self.xi[i];
        }
        return accumulator / (self.xi.len() as f64);
    }
    /// Returns the standard deviation
    /// #Arguments
    /// * `sample` - Whether sample sd or sd as an estimator should be estimated
    /// * `full` - Whether the sd for the full recording should be calculated, or only for xi?
    fn sd(self, sample: bool, full: bool) -> f64 {
        let mean_rr = self.mean_rr_full();
        let mut var_accu = 0.0;
        let mut comp = 0.0;
        for rr in self.rr_intervals {
            (comp, var_accu) = self.sum_of_squares(rr, mean_rr, var_accu, comp);
        }
        let n = if full {
            self.xi.len() + 1
        } else {
            self.xi.len()
        };
        let divisor = if sample { n } else { n - 1 };
        return (var_accu / divisor).sqrt();
    }
    fn sum_of_squares(rr: f64, mean_rr: f64, comp: f64, var_accu: f64) -> (f64, f64) {
        let diff = rr - mean_rr;
        let term = diff * diff;
        let y = term - comp;
        let t = var_accu + y;
        return ((t - var_accu) - y, t); // this returns the new values of comp and var_accu, in order
    }
}
