use crate::common::Annotations;
use crate::stat_funcs::mean;
use crate::stat_funcs::sd;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AsymVarDesc {
    rr_intervals: Vec<f64>,
    annotations: Vec<Annotations>,
    pp: PoincarePlot,
    length: usize,
    quality_stats: QualityStats,
    time_length: f64,
    pub mean_rr: f64,
    pub sdnn: f64,
    sd1: f64,
    sd2: f64,
    sd1_i: f64,
    sd1a: f64,
    sd1d: f64,
    sd2a: f64,
    sd2d: f64,
    sdnn_a: f64,
    sdnn_d: f64,
    analyzed: bool,
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
            pp: PoincarePlot {
                xi: vec![],
                xii: vec![],
            },
            length: length,
            quality_stats: QualityStats::default(),
            time_length: 1.0,
            mean_rr: 0.0,
            sdnn: 0.0,
            sd1: 0.0,
            sd2: 1.0,
            sd1_i: 1.0,
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
        self.analyzed = true;
        self.sd1 = self.sd1();
        (self.sd2, self.sd2d, self.sd2a) = self.sd2();
        (self.sd1_i, self.sd1d, self.sd1a) = self.sd1_i();
    }

    fn get_quality_stats(&self) -> QualityStats {
        let mut quality_stats = QualityStats {
            n: 0,
            v: 0,
            s: 0,
            x: 0,
        };
        for i in 0..self.length {
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
        for idx in 0..self.length - 1 {
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
        let (mean, n) = if full {
            (self.mean_rr, self.pp.xi.len() + 1)
        } else {
            (self.mean_rr_pp(), self.pp.xi.len())
        };

        if n == 0 || (sample && n < 2) {
            return 0.0;
        }

        if full {
            for rr in &self.pp.xi {
                (comp, var_accu) = self.sum_of_squares(*rr, mean, comp, var_accu);
            }
            if let Some(last) = self.pp.xii.last() {
                (_, var_accu) = self.sum_of_squares(*last, mean, comp, var_accu);
            }
        } else {
            for rr in &self.pp.xi {
                (comp, var_accu) = self.sum_of_squares(*rr, mean, comp, var_accu);
            }
        }

        let divisor = if sample { n - 1 } else { n };
        return (var_accu / divisor as f64).sqrt();
    }
    fn sum_of_squares(&self, rr: f64, mean: f64, comp: f64, var_accu: f64) -> (f64, f64) {
        let diff = rr - mean;
        let term = diff * diff;
        let y = term - comp;
        let t = var_accu + y;
        return ((t - var_accu) - y, t); // this returns the new values of comp and var_accu, in order
    }
    fn sd1(&self) -> f64 {
        let pp_len = self.pp.xi.len();
        let mut diff = vec![0.0; pp_len];
        for i in 0..pp_len {
            let local_diff = &self.pp.xii[i] - &self.pp.xi[i];
            diff[i] = local_diff / 2.0;
        }
        return sd(&diff, true);
    }

    fn sd2(&self) -> (f64, f64, f64) {
        let pp_len = self.pp.xi.len();
        let mut sum = vec![0.0; pp_len];
        let mut var_2_d = 0.0;
        let mut var_2_a = 0.0;
        let modifier = 1.0 / (pp_len - 1) as f64; // -1 because pp are shorter by 1
        let mean_rr_i = mean(&self.pp.xi);
        let mean_rr_ii = mean(&self.pp.xii);
        for i in 0..pp_len {
            let local_sum = &self.pp.xii[i] + &self.pp.xi[i];
            let local_l2_perp_dist =
                (&self.pp.xi[i] - mean_rr_i + &self.pp.xii[i] - mean_rr_ii) / 2_f64.sqrt();
            let local_diff = &self.pp.xii[i] - &self.pp.xi[i];
            sum[i] = local_sum / 2_f64.sqrt();
            let local_l2_perp_dist_squared = local_l2_perp_dist * local_l2_perp_dist;
            if local_diff > 0.0 {
                var_2_d += local_l2_perp_dist_squared;
            }
            if local_diff < 0. {
                var_2_a += local_l2_perp_dist_squared;
            }
            if local_diff == 0. {
                // spreading the variance on l1 equally between accelerations and decelerations
                var_2_d += 0.5 * local_l2_perp_dist_squared;
                var_2_a += 0.5 * local_l2_perp_dist_squared;
            }
        }
        // note that the way of calculating sd2 is totally different from the way of calculating
        // sd2d and sd2a - this is done to facilitate testing the partitioning
        return (
            sd(&sum, true),
            (modifier * var_2_d).sqrt(),
            (modifier * var_2_a).sqrt(),
        );
    }
    fn sd1_i(&self) -> (f64, f64, f64) {
        let pp_len = self.pp.xi.len();
        let mut var_1_i = 0.0;
        let mut var_1_d = 0.0;
        let mut var_1_a = 0.0;
        let modifier = (1.0 / pp_len as f64) * 1.0 / 2.;
        for i in 0..pp_len {
            let local_diff = &self.pp.xii[i] - &self.pp.xi[i];
            let local_diff_squared = local_diff * local_diff;
            var_1_i += local_diff_squared / 2.;
            if local_diff > 0.0 {
                var_1_d += local_diff_squared / 2.;
            }
            if local_diff < 0.0 {
                var_1_a += local_diff_squared / 2.;
            }
        }
        return (
            (modifier * var_1_i).sqrt(),
            (modifier * var_1_d).sqrt(),
            (modifier * var_1_a).sqrt(),
        );
    }
}
