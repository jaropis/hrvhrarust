use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use hrvhra_rust::asym::AsymVarDesc;
use hrvhra_rust::common::Annotations;
use hrvhra_rust::data_reader::RRSeries;
use hrvhra_rust::runs::RRRuns;

/// Reads a result file whose non-empty lines contain a metric name and a numeric value.
///
/// For example, `SDNN 74.0870762649683` becomes the map entry
/// `"SDNN".to_owned(): 74.0870762649683`.
fn read_expected_results(path: impl AsRef<Path>) -> io::Result<HashMap<String, f64>> {
    let file = File::open(path)?;
    let mut results = HashMap::new();

    for (line_index, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        let mut fields = line.split_whitespace();

        let Some(name) = fields.next() else {
            continue;
        };
        let value_text = fields.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("line {} is missing a value", line_index + 1),
            )
        })?;

        if fields.next().is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "line {} contains more than a name and value",
                    line_index + 1
                ),
            ));
        }

        let value = value_text.parse::<f64>().map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("line {} has an invalid value: {error}", line_index + 1),
            )
        })?;

        if results.insert(name.to_owned(), value).is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("line {} repeats the metric {name}", line_index + 1),
            ));
        }
    }

    Ok(results)
}

#[test]
fn test_16_6_percent() -> io::Result<()> {
    let results = read_expected_results("tests/data/test_result12.csv")?;
    let expected_names = [
        "SDNN", "SD1", "SD2", "SD1I", "MEAN_RR", "SDNNd", "SDNNa", "SD1d", "SD1a", "SD2d", "SD2a",
        "PI",
    ];
    assert_eq!(results.len(), expected_names.len());
    for name in expected_names {
        assert!(results.contains_key(name), "missing metric {name}");
    }
    let rr_series = RRSeries::read_rr("tests/data/test12.csv")?;
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr.clone(), rr_series.annot.clone());
    asym_var.analyze_asym_var();
    assert!((asym_var.sdnn - results["SDNN"]).abs() < 0.0000001);
    assert!((asym_var.sd1 - results["SD1"]).abs() < 0.0000001);
    assert!((asym_var.sd2 - results["SD2"]).abs() < 0.0000001);
    assert!((asym_var.sd1_i - results["SD1I"]).abs() < 0.0000001);
    assert!((asym_var.sd1a - results["SD1a"]).abs() < 0.0000001);
    assert!((asym_var.sd1d - results["SD1d"]).abs() < 0.0000001);
    assert!((asym_var.sd2a - results["SD2a"]).abs() < 0.0000001);
    assert!((asym_var.sd2d - results["SD2d"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_d - results["SDNNd"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_a - results["SDNNa"]).abs() < 0.0000001);
    assert!((asym_var.mean_rr - results["MEAN_RR"]).abs() < 0.0000001);
    Ok(())
}

#[test]
fn test_25_percent() -> io::Result<()> {
    let results = read_expected_results("tests/data/test_result13.csv")?;
    let expected_names = [
        "SDNN", "SD1", "SD2", "SD1I", "MEAN_RR", "SDNNd", "SDNNa", "SD1d", "SD1a", "SD2d", "SD2a",
        "PI",
    ];
    assert_eq!(results.len(), expected_names.len());
    for name in expected_names {
        assert!(results.contains_key(name), "missing metric {name}");
    }
    let rr_series = RRSeries::read_rr("tests/data/test13.csv")?;
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr.clone(), rr_series.annot.clone());
    asym_var.analyze_asym_var();
    assert!((asym_var.sdnn - results["SDNN"]).abs() < 0.0000001);
    assert!((asym_var.sd1 - results["SD1"]).abs() < 0.0000001);
    assert!((asym_var.sd2 - results["SD2"]).abs() < 0.0000001);
    assert!((asym_var.sd1_i - results["SD1I"]).abs() < 0.0000001);
    assert!((asym_var.sd1a - results["SD1a"]).abs() < 0.0000001);
    assert!((asym_var.sd1d - results["SD1d"]).abs() < 0.0000001);
    assert!((asym_var.sd2a - results["SD2a"]).abs() < 0.0000001);
    assert!((asym_var.sd2d - results["SD2d"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_d - results["SDNNd"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_a - results["SDNNa"]).abs() < 0.0000001);
    assert!((asym_var.mean_rr - results["MEAN_RR"]).abs() < 0.0000001);
    Ok(())
}

#[test]
fn test_33_percent() -> io::Result<()> {
    let results = read_expected_results("tests/data/test_result14.csv")?;
    let expected_names = [
        "SDNN", "SD1", "SD2", "SD1I", "MEAN_RR", "SDNNd", "SDNNa", "SD1d", "SD1a", "SD2d", "SD2a",
        "PI",
    ];
    assert_eq!(results.len(), expected_names.len());
    for name in expected_names {
        assert!(results.contains_key(name), "missing metric {name}");
    }
    let rr_series = RRSeries::read_rr("tests/data/test14.csv")?;
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr.clone(), rr_series.annot.clone());
    asym_var.analyze_asym_var();
    assert!((asym_var.sdnn - results["SDNN"]).abs() < 0.0000001);
    assert!((asym_var.sd1 - results["SD1"]).abs() < 0.0000001);
    assert!((asym_var.sd2 - results["SD2"]).abs() < 0.0000001);
    assert!((asym_var.sd1_i - results["SD1I"]).abs() < 0.0000001);
    assert!((asym_var.sd1a - results["SD1a"]).abs() < 0.0000001);
    assert!((asym_var.sd1d - results["SD1d"]).abs() < 0.0000001);
    assert!((asym_var.sd2a - results["SD2a"]).abs() < 0.0000001);
    assert!((asym_var.sd2d - results["SD2d"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_d - results["SDNNd"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_a - results["SDNNa"]).abs() < 0.0000001);
    assert!((asym_var.mean_rr - results["MEAN_RR"]).abs() < 0.0000001);
    Ok(())
}

#[test]
fn test_41_6_percent() -> io::Result<()> {
    let results = read_expected_results("tests/data/test_result15.csv")?;
    let expected_names = [
        "SDNN", "SD1", "SD2", "SD1I", "MEAN_RR", "SDNNd", "SDNNa", "SD1d", "SD1a", "SD2d", "SD2a",
        "PI",
    ];
    assert_eq!(results.len(), expected_names.len());
    for name in expected_names {
        assert!(results.contains_key(name), "missing metric {name}");
    }
    let rr_series = RRSeries::read_rr("tests/data/test15.csv")?;
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr.clone(), rr_series.annot.clone());
    asym_var.analyze_asym_var();
    assert!((asym_var.sdnn - results["SDNN"]).abs() < 0.0000001);
    assert!((asym_var.sd1 - results["SD1"]).abs() < 0.0000001);
    assert!((asym_var.sd2 - results["SD2"]).abs() < 0.0000001);
    assert!((asym_var.sd1_i - results["SD1I"]).abs() < 0.0000001);
    assert!((asym_var.sd1a - results["SD1a"]).abs() < 0.0000001);
    assert!((asym_var.sd1d - results["SD1d"]).abs() < 0.0000001);
    assert!((asym_var.sd2a - results["SD2a"]).abs() < 0.0000001);
    assert!((asym_var.sd2d - results["SD2d"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_d - results["SDNNd"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_a - results["SDNNa"]).abs() < 0.0000001);
    assert!((asym_var.mean_rr - results["MEAN_RR"]).abs() < 0.0000001);
    Ok(())
}

#[test]
fn test_50_percent() -> io::Result<()> {
    let results = read_expected_results("tests/data/test_result16.csv")?;
    let expected_names = [
        "SDNN", "SD1", "SD2", "SD1I", "MEAN_RR", "SDNNd", "SDNNa", "SD1d", "SD1a", "SD2d", "SD2a",
        "PI",
    ];
    assert_eq!(results.len(), expected_names.len());
    for name in expected_names {
        assert!(results.contains_key(name), "missing metric {name}");
    }
    let rr_series = RRSeries::read_rr("tests/data/test16.csv")?;
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr.clone(), rr_series.annot.clone());
    asym_var.analyze_asym_var();
    assert!((asym_var.sdnn - results["SDNN"]).abs() < 0.0000001);
    assert!((asym_var.sd1 - results["SD1"]).abs() < 0.0000001);
    assert!((asym_var.sd2 - results["SD2"]).abs() < 0.0000001);
    assert!((asym_var.sd1_i - results["SD1I"]).abs() < 0.0000001);
    assert!((asym_var.sd1a - results["SD1a"]).abs() < 0.0000001);
    assert!((asym_var.sd1d - results["SD1d"]).abs() < 0.0000001);
    assert!((asym_var.sd2a - results["SD2a"]).abs() < 0.0000001);
    assert!((asym_var.sd2d - results["SD2d"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_d - results["SDNNd"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_a - results["SDNNa"]).abs() < 0.0000001);
    assert!((asym_var.mean_rr - results["MEAN_RR"]).abs() < 0.0000001);
    Ok(())
}

#[test]
fn test_58_3_percent() -> io::Result<()> {
    let results = read_expected_results("tests/data/test_result17.csv")?;
    let expected_names = [
        "SDNN", "SD1", "SD2", "SD1I", "MEAN_RR", "SDNNd", "SDNNa", "SD1d", "SD1a", "SD2d", "SD2a",
        "PI",
    ];
    assert_eq!(results.len(), expected_names.len());
    for name in expected_names {
        assert!(results.contains_key(name), "missing metric {name}");
    }
    let rr_series = RRSeries::read_rr("tests/data/test17.csv")?;
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr.clone(), rr_series.annot.clone());
    asym_var.analyze_asym_var();
    assert!((asym_var.sdnn - results["SDNN"]).abs() < 0.0000001);
    assert!((asym_var.sd1 - results["SD1"]).abs() < 0.0000001);
    assert!((asym_var.sd2 - results["SD2"]).abs() < 0.0000001);
    assert!((asym_var.sd1_i - results["SD1I"]).abs() < 0.0000001);
    assert!((asym_var.sd1a - results["SD1a"]).abs() < 0.0000001);
    assert!((asym_var.sd1d - results["SD1d"]).abs() < 0.0000001);
    assert!((asym_var.sd2a - results["SD2a"]).abs() < 0.0000001);
    assert!((asym_var.sd2d - results["SD2d"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_d - results["SDNNd"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_a - results["SDNNa"]).abs() < 0.0000001);
    assert!((asym_var.mean_rr - results["MEAN_RR"]).abs() < 0.0000001);
    Ok(())
}

#[test]
fn test_66_6_percent() -> io::Result<()> {
    let results = read_expected_results("tests/data/test_result18.csv")?;
    let expected_names = [
        "SDNN", "SD1", "SD2", "SD1I", "MEAN_RR", "SDNNd", "SDNNa", "SD1d", "SD1a", "SD2d", "SD2a",
        "PI",
    ];
    assert_eq!(results.len(), expected_names.len());
    for name in expected_names {
        assert!(results.contains_key(name), "missing metric {name}");
    }
    let rr_series = RRSeries::read_rr("tests/data/test18.csv")?;
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr.clone(), rr_series.annot.clone());
    asym_var.analyze_asym_var();
    assert!((asym_var.sdnn - results["SDNN"]).abs() < 0.0000001);
    assert!((asym_var.sd1 - results["SD1"]).abs() < 0.0000001);
    assert!((asym_var.sd2 - results["SD2"]).abs() < 0.0000001);
    assert!((asym_var.sd1_i - results["SD1I"]).abs() < 0.0000001);
    assert!((asym_var.sd1a - results["SD1a"]).abs() < 0.0000001);
    assert!((asym_var.sd1d - results["SD1d"]).abs() < 0.0000001);
    assert!((asym_var.sd2a - results["SD2a"]).abs() < 0.0000001);
    assert!((asym_var.sd2d - results["SD2d"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_d - results["SDNNd"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_a - results["SDNNa"]).abs() < 0.0000001);
    assert!((asym_var.mean_rr - results["MEAN_RR"]).abs() < 0.0000001);
    Ok(())
}

#[test]
fn test_75_percent() -> io::Result<()> {
    let results = read_expected_results("tests/data/test_result19.csv")?;
    let expected_names = [
        "SDNN", "SD1", "SD2", "SD1I", "MEAN_RR", "SDNNd", "SDNNa", "SD1d", "SD1a", "SD2d", "SD2a",
        "PI",
    ];
    assert_eq!(results.len(), expected_names.len());
    for name in expected_names {
        assert!(results.contains_key(name), "missing metric {name}");
    }
    let rr_series = RRSeries::read_rr("tests/data/test19.csv")?;
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr.clone(), rr_series.annot.clone());
    asym_var.analyze_asym_var();
    assert!((asym_var.sdnn - results["SDNN"]).abs() < 0.0000001);
    assert!((asym_var.sd1 - results["SD1"]).abs() < 0.0000001);
    assert!((asym_var.sd2 - results["SD2"]).abs() < 0.0000001);
    assert!((asym_var.sd1_i - results["SD1I"]).abs() < 0.0000001);
    assert!((asym_var.sd1a - results["SD1a"]).abs() < 0.0000001);
    assert!((asym_var.sd1d - results["SD1d"]).abs() < 0.0000001);
    assert!((asym_var.sd2a - results["SD2a"]).abs() < 0.0000001);
    assert!((asym_var.sd2d - results["SD2d"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_d - results["SDNNd"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_a - results["SDNNa"]).abs() < 0.0000001);
    assert!((asym_var.mean_rr - results["MEAN_RR"]).abs() < 0.0000001);
    Ok(())
}

#[test]
fn test_83_percent() -> io::Result<()> {
    let results = read_expected_results("tests/data/test_result20.csv")?;
    let expected_names = [
        "SDNN", "SD1", "SD2", "SD1I", "MEAN_RR", "SDNNd", "SDNNa", "SD1d", "SD1a", "SD2d", "SD2a",
        "PI",
    ];
    assert_eq!(results.len(), expected_names.len());
    for name in expected_names {
        assert!(results.contains_key(name), "missing metric {name}");
    }
    let rr_series = RRSeries::read_rr("tests/data/test20.csv")?;
    let mut asym_var: AsymVarDesc = AsymVarDesc::new(rr_series.rr.clone(), rr_series.annot.clone());
    asym_var.analyze_asym_var();
    assert!((asym_var.sdnn - results["SDNN"]).abs() < 0.0000001);
    assert!((asym_var.sd1 - results["SD1"]).abs() < 0.0000001);
    assert!((asym_var.sd2 - results["SD2"]).abs() < 0.0000001);
    assert!((asym_var.sd1_i - results["SD1I"]).abs() < 0.0000001);
    assert!((asym_var.sd1a - results["SD1a"]).abs() < 0.0000001);
    assert!((asym_var.sd1d - results["SD1d"]).abs() < 0.0000001);
    assert!((asym_var.sd2a - results["SD2a"]).abs() < 0.0000001);
    assert!((asym_var.sd2d - results["SD2d"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_d - results["SDNNd"]).abs() < 0.0000001);
    assert!((asym_var.sdnn_a - results["SDNNa"]).abs() < 0.0000001);
    assert!((asym_var.mean_rr - results["MEAN_RR"]).abs() < 0.0000001);
    Ok(())
}
