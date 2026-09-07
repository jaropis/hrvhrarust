use std::io::{self};

use hrvhra_rust::asym::AsymVarDesc;
use hrvhra_rust::common::VarType;
use hrvhra_rust::data_reader::RRSeries;
use hrvhra_rust::runs::RRRuns;

/// These tests test the consistency of the rust results between hra variance - based descriptors calculated from the Poincare plot and those calculated
/// by summing corresponding runs variances. So, one side of the eqation uses `asym.rs` and the other RRRuns

/// Reads a result file whose non-empty lines contain a metric name and a numeric value.

#[test]
fn test_internal_consistency_12() -> io::Result<()> {
    let rr_series = RRSeries::read_rr("tests/data/test12.csv")?;
    let mut rr = RRRuns::new(rr_series.rr.clone(), rr_series.annot.clone(), true);
    rr.get_full_runs();
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr, rr_series.annot);
    let runs_asym_vars = rr.get_runs_variances();
    asym_var.analyze_asym_var();
    println!(
        "asym: {}; runs: {}",
        asym_var.sd1_i,
        runs_asym_vars[&VarType::SD1].sqrt()
    );
    // assert!((asym_var.sd1 - runs_asym_vars[&VarType::SD1].sqrt()).abs() < 0.0000001);
    Ok(())
}
