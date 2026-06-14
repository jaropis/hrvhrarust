#[derive(Debug, Clone)]
pub struct AsymVarDesc {
    rr_intervals: Vec<f64>,
    annotations: Vec<Annotations>,
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
    analyzed: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Annotations {
    N = 0,
    V = 1,
    S = 2,
    X = 3,
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

impl AsymVarDesc {
    pub fn new(rr_intervals: Vec<f64>, annotations: Vec<Annotations>) -> Self {
        let length = rr_intervals.len();

        return AsymVarDesc {
            rr_intervals: rr_intervals,
            annotations: annotations,
            length: length,
            quality_stats: QualityStats::default(),
            time_length: 1.0,
            mean_rr: 0.0,
            sdnn: 0.0,
            sd1: 1.0,
            sd2: 1.0,
            pp: PoincarePlot {
                xi: vec![],
                xii: vec![],
            },
            sd1a: 1.0,
            sd1d: 1.0,
            sd2a: 1.0,
            sd2d: 1.0,
            sdnn_a: 1.0,
            sdnn_d: 1.0,
            analyzed: false,
        };
    }
    pub fn analyze_asym_var(&mut self) {
        self.quality_stats = self.get_quality_stats();
        self.pp = self.form_pp();
        self.mean_rr = self.mean_rr_full();
        self.sdnn = self.sd(true, true);
    }

    fn get_quality_stats(&self) -> QualityStats {
        let mut quality_stats = QualityStats {
            n: 0,
            v: 0,
            s: 0,
            x: 0,
        };
        for i in 0..self.length - 1 {
            match self.annotations[i] {
                Annotations::N => quality_stats.n = quality_stats.n + 1,
                Annotations::S => quality_stats.v = quality_stats.v + 1,
                Annotations::V => quality_stats.s = quality_stats.s + 1,
                Annotations::X => quality_stats.x = quality_stats.x + 1,
            }
        }
        quality_stats
    }
    fn form_pp(&mut self) -> PoincarePlot {
        let mut xi: Vec<f64> = vec![];
        let mut xii: Vec<f64> = vec![];
        for idx in 0..self.length {
            if (self.annotations[idx] == Annotations::N)
                & (self.annotations[idx + 1] == Annotations::N)
            {
                xi.push(self.rr_intervals[idx]);
                xii.push(self.rr_intervals[idx + 1])
            }
        }
        return PoincarePlot { xi: xi, xii: xii };
    }
    fn mean_rr_full(&self) -> f64 {
        // this is a regular mean from all RR's that are of sinus origin
        let mut accumulator = 0.0;
        let length = self.pp.xi.len() - 1;
        for i in 0..self.pp.xi.len() as usize {
            accumulator = accumulator + self.pp.xi[i];
        }
        accumulator = accumulator + self.pp.xii[length];
        return accumulator / (self.pp.xi.len() + 1) as f64;
    }
    fn mean_rr_pp(&self) -> f64 {
        // this is calculated from xi only
        let mut accumulator = 0.0;
        for i in 0..self.pp.xi.len() as usize {
            accumulator = accumulator + self.pp.xi[i];
        }
        return accumulator / (self.pp.xi.len() as f64);
    }
    /// Returns the standard deviation
    /// #Arguments
    /// * `sample` - Whether sample sd or sd as an estimator should be estimated
    /// * `full` - Whether the sd for the full recording should be calculated, or only for xi?
    fn sd(&self, sample: bool, full: bool) -> f64 {
        let mut var_accu = 0.0;
        let mut comp = 0.0;
        for rr in &self.rr_intervals {
            (comp, var_accu) = self.sum_of_squares(rr, var_accu, comp);
        }
        let n = if full {
            self.pp.xi.len() + 1
        } else {
            self.pp.xi.len()
        };
        let divisor = if sample { n } else { n - 1 };
        return (var_accu / divisor as f64).sqrt();
    }
    fn sum_of_squares(&self, rr: &f64, comp: f64, var_accu: f64) -> (f64, f64) {
        let diff = rr - self.mean_rr;
        let term = diff * diff;
        let y = term - comp;
        let t = var_accu + y;
        return ((t - var_accu) - y, t); // this returns the new values of comp and var_accu, in order
    }
}
