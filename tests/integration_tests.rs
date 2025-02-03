use std::io;
// Import the needed types from your library
use hrvhra_rust::data_reader::RRSeries;
use hrvhra_rust::runs::RRRuns;

#[test]
fn test_case_1() -> io::Result<()> {
    // reading test data
    let rr_series = RRSeries::read_rr("tests/data/test1.csv")?;
    let mut rr = RRRuns::new(rr_series.rr, rr_series.annot, true);

    // getting full analysis
    rr.get_full_runs();

    // asserting expected results - note the double vec![]
    assert_eq!(rr.get_runs_summary(), vec![vec![1, 2, 0]]);
    Ok(())
}

#[test]
fn test_case_2() -> io::Result<()> {
    let rr_series = RRSeries::read_rr("tests/data/test2.csv")?;
    let mut rr = RRRuns::new(rr_series.rr, rr_series.annot, true);
    rr.get_full_runs();

    assert_eq!(rr.get_runs_summary(), vec![vec![2, 2, 1], vec![0, 0, 1]]);
    Ok(())
}

#[test]
fn test_case_3() -> io::Result<()> {
    let rr_series = RRSeries::read_rr("tests/data/test3.csv")?;
    let mut rr = RRRuns::new(rr_series.rr, rr_series.annot, true);
    rr.get_full_runs();

    assert_eq!(rr.get_runs_summary(), vec![vec![1, 1, 1], vec![0, 0, 1]]);
    Ok(())
}

#[test]
fn test_case_4() -> io::Result<()> {
    let rr_series = RRSeries::read_rr("tests/data/test4.csv")?;
    let mut rr = RRRuns::new(rr_series.rr, rr_series.annot, true);
    rr.get_full_runs();

    assert_eq!(rr.get_runs_summary(), vec![vec![0, 0, 0]]);
    Ok(())
}

#[test]
fn test_case_5() -> io::Result<()> {
    let rr_series = RRSeries::read_rr("tests/data/test5.csv")?;
    let mut rr = RRRuns::new(rr_series.rr, rr_series.annot, true);
    rr.get_full_runs();

    assert_eq!(rr.get_runs_summary(), vec![vec![0, 1, 1]]);
    Ok(())
}

#[test]
fn test_case_6() -> io::Result<()> {
    let rr_series = RRSeries::read_rr("tests/data/test6.csv")?;
    let mut rr = RRRuns::new(rr_series.rr, rr_series.annot, true);
    rr.get_full_runs();

    assert_eq!(rr.get_runs_summary(), vec![vec![0, 1, 0]]);
    Ok(())
}

#[test]
fn test_case_7() -> io::Result<()> {
    let rr_series = RRSeries::read_rr("tests/data/test7.csv")?;
    let mut rr = RRRuns::new(rr_series.rr, rr_series.annot, true);
    rr.get_full_runs();

    assert_eq!(rr.get_runs_summary(), vec![vec![1, 2, 0], vec![0, 0, 1]]);
    Ok(())
}
