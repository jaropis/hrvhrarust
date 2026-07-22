use hrvhra_rust::stat_funcs::{mean, sd};

#[test]
fn mean_empty_returns_zero() {
    let rr = vec![];
    assert_eq!(mean(&rr), 0.0);
}

#[test]
fn mean_basic_values() {
    let rr = vec![1.0, 2.0, 3.0, 4.0];
    assert_eq!(mean(&rr), 2.5);
}

#[test]
fn sd_empty_returns_zero() {
    let rr = vec![];
    assert_eq!(sd(&rr, false), 0.0);
    assert_eq!(sd(&rr, true), 0.0);
}

#[test]
fn sd_single_sample_mode_returns_zero() {
    let rr = vec![1.0];
    assert_eq!(sd(&rr, true), 0.0);
}

#[test]
fn sd_population_and_sample_match_expected() {
    let rr = vec![1.0, 2.0, 3.0, 4.0];

    let pop = sd(&rr, false);
    let sample = sd(&rr, true);

    // Population SD = sqrt(1.25) ~= 1.11803398875
    assert!((pop - 1.11803398875).abs() < 1e-10);
    // Sample SD = sqrt(5/3) ~= 1.29099444874
    assert!((sample - 1.29099444874).abs() < 1e-10);
}
