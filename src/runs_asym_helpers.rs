use crate::asym::PoincarePlot;
use crate::common::Annotations;
use crate::stat_funcs::mean;
use crate::stat_funcs::sd;

fn runs_form_pp(rr_intervals: Vec<f64>, annotations: Vec<Annotations>) -> PoincarePlot {
    let mut xi: Vec<f64> = vec![];
    let mut xii: Vec<f64> = vec![];
    for idx in 0..self.length - 1 {
        if (annotations[idx] == Annotations::N) & (annotations[idx + 1] == Annotations::N) {
            xi.push(rr_intervals[idx]);
            xii.push(rr_intervals[idx + 1])
        }
    }
    return PoincarePlot { xi: xi, xii: xii };
}
