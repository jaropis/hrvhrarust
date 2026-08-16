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

pub fn sd1_i(rr_intervals: &Vec<f64>, annotations: &Vec<Annotations>) -> (Vec<Option<f64>>, f64) {
    let mut var_1_i = 0.0;
    let mut var_1_d = 0.0;
    let mut var_1_a = 0.0;
    let rr_length = rr_intervals.len();
    let mut point_sd1_i_vars: Vec<Option<f64>> = vec![None; rr_length];
    let mut none_counter = 1;
    point_sd1_i_vars[0] = None; // PAPER: this is not really necessary, because of the pre-allocation of the vector,
                                // but the runs can only start from 1 - each point in the PP plot contributes variance
                                // so let's look for the runs in the SECOND vector in the PP - this is the only possibility
    for i in 1..(rr_intervals.len()) {
        if annotations[i] != Annotations::N && annotations[i + 1] != Annotations::N {
            point_sd1_i_vars[i] = None;
            none_counter += 1;
            continue;
        }
        let local_diff = rr_intervals[i] - rr_intervals[i - 1];
        let local_diff_squared = local_diff * local_diff;
        var_1_i += local_diff_squared / 2.;
        if local_diff > 0.0 {
            point_sd1_i_vars[i] = Some(local_diff_squared / 2.);
            var_1_d += local_diff_squared / 2.;
        }
        if local_diff < 0.0 {
            point_sd1_i_vars[i] = Some(local_diff_squared / 2.);
            var_1_a += local_diff_squared / 2.;
        }
        if i == rr_length - 1 {
            println!("i = {}, local_diff_squared: {}", i, local_diff_squared)
        }
    }
    let modifier = 1.0 / (rr_length - none_counter) as f64;

    return (point_sd1_i_vars, modifier);
}
