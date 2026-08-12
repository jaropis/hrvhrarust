use crate::asym::PoincarePlot;
use crate::common::Annotations;
// use crate::stat_funcs::mean;
// use crate::stat_funcs::sd;

fn runs_form_pp(rr_intervals: &Vec<f64>, annotations: &Vec<Annotations>) -> PoincarePlot {
    let mut xi: Vec<f64> = vec![];
    let mut xii: Vec<f64> = vec![];
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

pub fn sd1_i(rr_intervals: &Vec<f64>, annotations: &Vec<Annotations>) -> (f64, f64, f64) {
    let pp = runs_form_pp(rr_intervals, annotations);
    let pp_len = pp.xi.len();
    let mut var_1_i = 0.0;
    let mut var_1_d = 0.0;
    let mut var_1_a = 0.0;
    let modifier = 1.0 / pp_len as f64;
    for i in 0..pp_len {
        let local_diff = pp.xii[i] - pp.xi[i];
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
