fn mean_rr(rr: Vec(f64)) -> f64 {
    let mut accumulator = 0.0;
    let length = rr.len();
    for i in 0..length as usize {
        accumulator = accumulator + self.pp.xi[i];
    }
    return accumulator / (length) as f64;
}
