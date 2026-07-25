use crate::asym::PoincarePlot;
use crate::common::Annotations;
// use crate::stat_funcs::mean;
// use crate::stat_funcs::sd;

fn runs_form_pp(rr_intervals: &Vec<f64>, annotations: &Vec<Annotations>) -> PoincarePlot {
    let mut xi: Vec<f64> = vec![];
    let mut xii: Vec<f64> = vec![];
    println!("run rr intervals: {:?}", rr_intervals);
    println!("annotations: {:?}", annotations);
    for idx in 0..(rr_intervals.len() - 1) {
        if (annotations[idx] == Annotations::N) & (annotations[idx + 1] == Annotations::N) {
            xi.push(rr_intervals[idx]);
            xii.push(rr_intervals[idx + 1])
        }
    }
    return PoincarePlot { xi: xi, xii: xii };
}

pub fn get_mean_for_sd1(rr_intervals: &Vec<f64>, annotations: &Vec<Annotations>) -> f64 {
    let mut sum = 0.;
    let pp = runs_form_pp(rr_intervals, annotations);
    for i in 0..pp.xi.len() {
        sum += pp.xii[i] - pp.xi[i];
    }
    return sum / pp.xi.len() as f64;
}
