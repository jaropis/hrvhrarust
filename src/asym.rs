use std::cmp;

#[derive(Debug, Clone, Copy)]

pub struct RRSeries {
    rr_intervals: Vec(f64),
    annotations: Vec(u8),
    length: usize,
    time_length: f64,
    mean_rr: f64,
    SDNN: f64,
    SD1: f64,
    SD2: f64,
    SD1a: f64,
    SD1d: f64,
    SD2a: f64,
    SD2d: f64,
    SDNNa: f64,
    SDNNd: f64,
}
