use std::cmp;

#[derive(Debug, Clone, Copy)]

pub struct RRSeries {
    rr_intervals: Vec(f64),
    annotations: Vec(u8),
    length: usize,
    quality_stats: QualityStats,
    time_length: f64,
    mean_rr: f64,
    SDNN: f64,
    SD1: f64,
    SD2: f64,
    pp: PoincarePlot,
    SD1a: f64,
    SD1d: f64,
    SD2a: f64,
    SD2d: f64,
    SDNNa: f64,
    SDNNd: f64,
}

pub struct QualityStats {
    n: usize = 0, // normal
    v: usize = 0, // ventricular
    s: usize = 0, // supraventricular
    x: usize = 0, // artifact
}

pub struct PoincarePlot {
    xi: Vec<f64>,
    xii: Vec<f64>,
}

impl RRSeries {
    fn get_quality_stats(self) -> QualityStats {
        let quality_stats: QualityStats;
        for i in 0..=self.length - 1 {
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
        for idx in 0..=self.length {
            println!("dupa");
        }
    }
}
