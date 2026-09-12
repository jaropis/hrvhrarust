use std::io::{self};

use hrvhra_rust::asym::AsymVarDesc;
use hrvhra_rust::common::VarType;
use hrvhra_rust::data_reader::RRSeries;
use hrvhra_rust::runs::RRRuns;

/// These tests test the consistency of the rust results between hra variance - based descriptors calculated from the Poincare plot and those calculated
/// by summing corresponding runs variances. So, one side of the eqation uses `asym.rs` and the other RRRuns

/// Reads a result file whose non-empty lines contain a metric name and a numeric value.

#[test]
fn test_internal_consistency_16_6_percent() -> io::Result<()> {
    let num_threshold = 0.000000001;
    let rr_series = RRSeries::read_rr("tests/data/test12.csv")?;
    let mut rr = RRRuns::new(rr_series.rr.clone(), rr_series.annot.clone(), true);
    rr.get_full_runs();
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr, rr_series.annot);
    let runs_asym_vars = rr.get_runs_variances();
    asym_var.analyze_asym_var();

    assert!(asym_var.sd1 - runs_asym_vars[&VarType::Var1i].sqrt().abs() < num_threshold);
    assert!(
        asym_var.sd1_i
            - (runs_asym_vars[&VarType::Var1iD] + runs_asym_vars[&VarType::Var1iA]).sqrt()
            < num_threshold
    );
    assert!(asym_var.sd1a - runs_asym_vars[&VarType::Var1iA].sqrt() < num_threshold);
    assert!(asym_var.sd1d - runs_asym_vars[&VarType::Var1iD].sqrt() < num_threshold);

    assert!(asym_var.sd2 - (runs_asym_vars[&VarType::Var2]).sqrt() < num_threshold);
    assert!(asym_var.sd2a - runs_asym_vars[&VarType::Var2A].sqrt() < num_threshold);
    assert!(asym_var.sd2d - runs_asym_vars[&VarType::Var2D].sqrt() < num_threshold);

    assert!(asym_var.sdnn - (runs_asym_vars[&VarType::VarNNi]).sqrt() < num_threshold);
    assert!(asym_var.sdnn_a - runs_asym_vars[&VarType::VarNNiA].sqrt() < num_threshold);
    assert!(asym_var.sdnn_d - runs_asym_vars[&VarType::VarNNiD].sqrt() < num_threshold);
    Ok(())
}

#[test]
fn test_internal_consistency_25_percent() -> io::Result<()> {
    let num_threshold = 0.000000001;
    let rr_series = RRSeries::read_rr("tests/data/test13.csv")?;
    let mut rr = RRRuns::new(rr_series.rr.clone(), rr_series.annot.clone(), true);
    rr.get_full_runs();
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr, rr_series.annot);
    let runs_asym_vars = rr.get_runs_variances();
    asym_var.analyze_asym_var();

    assert!(asym_var.sd1 - runs_asym_vars[&VarType::Var1i].sqrt().abs() < num_threshold);
    assert!(
        asym_var.sd1_i
            - (runs_asym_vars[&VarType::Var1iD] + runs_asym_vars[&VarType::Var1iA]).sqrt()
            < num_threshold
    );
    assert!(asym_var.sd1a - runs_asym_vars[&VarType::Var1iA].sqrt() < num_threshold);
    assert!(asym_var.sd1d - runs_asym_vars[&VarType::Var1iD].sqrt() < num_threshold);

    assert!(asym_var.sd2 - (runs_asym_vars[&VarType::Var2]).sqrt() < num_threshold);
    assert!(asym_var.sd2a - runs_asym_vars[&VarType::Var2A].sqrt() < num_threshold);
    assert!(asym_var.sd2d - runs_asym_vars[&VarType::Var2D].sqrt() < num_threshold);

    assert!(asym_var.sdnn - (runs_asym_vars[&VarType::VarNNi]).sqrt() < num_threshold);
    assert!(asym_var.sdnn_a - runs_asym_vars[&VarType::VarNNiA].sqrt() < num_threshold);
    assert!(asym_var.sdnn_d - runs_asym_vars[&VarType::VarNNiD].sqrt() < num_threshold);
    Ok(())
}

#[test]
fn test_internal_consistency_33_percent() -> io::Result<()> {
    let num_threshold = 0.000000001;
    let rr_series = RRSeries::read_rr("tests/data/test14.csv")?;
    let mut rr = RRRuns::new(rr_series.rr.clone(), rr_series.annot.clone(), true);
    rr.get_full_runs();
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr, rr_series.annot);
    let runs_asym_vars = rr.get_runs_variances();
    asym_var.analyze_asym_var();

    assert!(asym_var.sd1 - runs_asym_vars[&VarType::Var1i].sqrt().abs() < num_threshold);
    assert!(
        asym_var.sd1_i
            - (runs_asym_vars[&VarType::Var1iD] + runs_asym_vars[&VarType::Var1iA]).sqrt()
            < num_threshold
    );

    assert!(asym_var.sd1a - runs_asym_vars[&VarType::Var1iA].sqrt() < num_threshold);
    assert!(asym_var.sd1d - runs_asym_vars[&VarType::Var1iD].sqrt() < num_threshold);
    assert!(asym_var.sd2 - (runs_asym_vars[&VarType::Var2]).sqrt() < num_threshold);
    assert!(asym_var.sd2a - runs_asym_vars[&VarType::Var2A].sqrt() < num_threshold);
    assert!(asym_var.sd2d - runs_asym_vars[&VarType::Var2D].sqrt() < num_threshold);
    assert!(asym_var.sdnn_i - (runs_asym_vars[&VarType::VarNNi]).sqrt() < num_threshold);
    assert!(asym_var.sdnn_a - runs_asym_vars[&VarType::VarNNiA].sqrt() < num_threshold);
    assert!(asym_var.sdnn_d - runs_asym_vars[&VarType::VarNNiD].sqrt() < num_threshold);
    Ok(())
}

#[test]
fn test_internal_consistency_41_6_percent() -> io::Result<()> {
    let num_threshold = 0.000000001;
    let rr_series = RRSeries::read_rr("tests/data/test15.csv")?;
    let mut rr = RRRuns::new(rr_series.rr.clone(), rr_series.annot.clone(), true);
    rr.get_full_runs();
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr, rr_series.annot);
    let runs_asym_vars = rr.get_runs_variances();
    asym_var.analyze_asym_var();
    assert!(asym_var.sd1 - runs_asym_vars[&VarType::Var1i].sqrt().abs() < num_threshold);
    assert!(
        asym_var.sd1_i
            - (runs_asym_vars[&VarType::Var1iD] + runs_asym_vars[&VarType::Var1iA]).sqrt()
            < num_threshold
    );
    assert!(asym_var.sd1a - runs_asym_vars[&VarType::Var1iA].sqrt() < num_threshold);
    assert!(asym_var.sd1d - runs_asym_vars[&VarType::Var1iD].sqrt() < num_threshold);
    assert!(asym_var.sd2 - (runs_asym_vars[&VarType::Var2]).sqrt() < num_threshold);
    assert!(asym_var.sd2a - runs_asym_vars[&VarType::Var2A].sqrt() < num_threshold);
    assert!(asym_var.sd2d - runs_asym_vars[&VarType::Var2D].sqrt() < num_threshold);
    assert!(asym_var.sdnn_i - (runs_asym_vars[&VarType::VarNNi]).sqrt() < num_threshold);
    assert!(asym_var.sdnn_a - runs_asym_vars[&VarType::VarNNiA].sqrt() < num_threshold);
    assert!(asym_var.sdnn_d - runs_asym_vars[&VarType::VarNNiD].sqrt() < num_threshold);
    Ok(())
}

#[test]
fn test_internal_consistency_50_percent() -> io::Result<()> {
    let num_threshold = 0.000000001;
    let rr_series = RRSeries::read_rr("tests/data/test16.csv")?;
    let mut rr = RRRuns::new(rr_series.rr.clone(), rr_series.annot.clone(), true);
    rr.get_full_runs();
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr, rr_series.annot);
    let runs_asym_vars = rr.get_runs_variances();
    asym_var.analyze_asym_var();

    assert!(asym_var.sd1 - runs_asym_vars[&VarType::Var1i].sqrt().abs() < num_threshold);
    assert!(
        asym_var.sd1_i
            - (runs_asym_vars[&VarType::Var1iD] + runs_asym_vars[&VarType::Var1iA]).sqrt()
            < num_threshold
    );

    assert!(asym_var.sd1a - runs_asym_vars[&VarType::Var1iA].sqrt() < num_threshold);
    assert!(asym_var.sd1d - runs_asym_vars[&VarType::Var1iD].sqrt() < num_threshold);
    assert!(asym_var.sd2 - (runs_asym_vars[&VarType::Var2]).sqrt() < num_threshold);
    assert!(asym_var.sd2a - runs_asym_vars[&VarType::Var2A].sqrt() < num_threshold);
    assert!(asym_var.sd2d - runs_asym_vars[&VarType::Var2D].sqrt() < num_threshold);
    assert!(asym_var.sdnn_i - (runs_asym_vars[&VarType::VarNNi]).sqrt() < num_threshold);
    assert!(asym_var.sdnn_a - runs_asym_vars[&VarType::VarNNiA].sqrt() < num_threshold);
    assert!(asym_var.sdnn_d - runs_asym_vars[&VarType::VarNNiD].sqrt() < num_threshold);
    Ok(())
}
