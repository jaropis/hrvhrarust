# hrvhra_rust

`hrvhra_rust` is a Rust library for analysing ECG RR-interval series. It provides RR-series input handling, runs analysis, Sample Entropy, and Poincaré plot-based heart rate asymmetry (HRA) metrics.

## Features

- Typed annotations for normal, ventricular, supraventricular, and artefact beats
- Acceleration, deceleration, and neutral runs analysis
- Sample Entropy using the NCM algorithm
- Poincaré plot-based HRV and HRA metrics for normal RR intervals
- Mean and population or sample standard-deviation helpers

## Installation

Add the Git dependency to your `Cargo.toml`:

```toml
[dependencies]
hrvhra_rust = { git = "https://github.com/jaropis/hrvhrarust.git" }
```

When developing against a local checkout, use a path dependency instead:

```toml
[dependencies]
hrvhra_rust = { path = "../hrvhrarust" }
```

## RR-series input

`RRSeries::read_rr` reads a whitespace-delimited text file. The first line is a header, and each subsequent row must begin with an RR interval and an annotation code:

```text
RR annot
800.0 0
810.0 0
795.0 1
805.0 0
```

| Code | `Annotations` variant | Meaning               |
| ---- | --------------------- | --------------------- |
| `0`  | `N`                   | Normal beat           |
| `1`  | `V`                   | Ventricular beat      |
| `2`  | `S`                   | Supraventricular beat |
| `3`  | `X`                   | Artefact beat         |

The reader does not convert RR units; use a single unit consistently when selecting parameters and interpreting metrics.

## Heart rate asymmetry

Use `AsymVarDesc` to calculate Poincaré plot-based HRV and HRA metrics. The analysis uses adjacent pairs for which both beats are annotated as normal.

```rust
use hrvhra_rust::{
    asym::AsymVarDesc,
    common::Annotations,
};

fn main() {
    let rr = vec![800.0, 810.0, 795.0, 805.0, 820.0];
    let annotations = Annotations::to_vec_of_annot(vec![0, 0, 0, 0, 0]);

    let mut analysis = AsymVarDesc::new(rr, annotations);
    analysis.analyze_asym_var();

    println!("Mean RR: {}", analysis.mean_rr);
    println!("SDNN: {}", analysis.sdnn);
    println!("SD1: {}", analysis.sd1);
    println!("SD2: {}", analysis.sd2);
}
```

After calling `analyze_asym_var`, the following public fields contain the calculated metrics:

- Overall metrics: `mean_rr`, `sdnn`, `sd1`, and `sd2`
- Short-term asymmetry: `sd1_i`, `sd1a`, and `sd1d`
- Long-term asymmetry: `sd2a` and `sd2d`
- Combined asymmetry: `sdnn_a` and `sdnn_d`

Provide RR intervals and annotations of equal length, with at least two consecutive normal beats for a meaningful Poincaré analysis.

## Runs analysis

`RRRuns` identifies acceleration, deceleration, and neutral runs among normal beats. `get_runs_summary` returns rows ordered by run length, with columns in the order acceleration, deceleration, and neutral.

```rust
use hrvhra_rust::{
    data_reader::RRSeries,
    runs::RRRuns,
};

fn main() -> std::io::Result<()> {
    let series = RRSeries::read_rr("rr_intervals.txt")?;
    let mut runs = RRRuns::new(series.rr, series.annot, true);

    for (index, counts) in runs.get_runs_summary().iter().enumerate() {
        let run_length = index + 1;
        println!(
            "length {run_length}: acceleration={}, deceleration={}, neutral={}",
            counts[0], counts[1], counts[2]
        );
    }

    Ok(())
}
```

The third argument to `RRRuns::new` controls whether a run that reaches the end of the input is included. Inputs without a valid pair of consecutive normal beats return a single zero row.

## Sample Entropy

`calc_samp_en` calculates Sample Entropy for a signal, embedding dimension `m`, and comparison tolerance `r`.

```rust
use hrvhra_rust::samp_en::calc_samp_en;

fn main() {
    let signal = vec![800.0, 810.0, 795.0, 805.0, 820.0, 815.0];
    let entropy = calc_samp_en(&signal, 2, 20.0);

    println!("Sample Entropy: {entropy}");
}
```

If no matching template pairs are found, the result may be non-finite. Check `entropy.is_finite()` when consuming the result.

## Testing

Run the complete test suite with:

```shell
cargo test --all-targets
```

The suite covers numerical helpers, runs edge cases, HRA metric partitions, and consistency cases spanning multiple ectopy levels.

## Contributing

Issues and pull requests are welcome.
