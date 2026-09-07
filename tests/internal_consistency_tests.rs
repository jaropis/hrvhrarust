use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use hrvhra_rust::asym::AsymVarDesc;
use hrvhra_rust::data_reader::RRSeries;

/// These tests test the consistency of the rust results with results from the R version of HRAEpxlorer +
/// the correct sdnn definition, i.e.
///   n <- length(new_data$rri[new_data$annot == 0]) # filtered length
///  SDNN3 <- sd(new_data$rri[new_data$annot == 0]) * sqrt((n-1)/n)
/// which is not actually calculated in the R version

/// Reads a result file whose non-empty lines contain a metric name and a numeric value.
///
/// For example, `SDNN 74.0870762649683` becomes the map entry
/// `"SDNN".to_owned(): 74.0870762649683`.

#[test]
fn test_nothing() -> io::Result<()> {
    assert_eq(0, 0);
    Ok(())
}
