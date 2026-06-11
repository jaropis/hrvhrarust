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
        let xi: Vec<f64>;
        let xii: Vec<f64>;
        for idx in 0..self.length {
            println!("dupa");
        }
    }
}
