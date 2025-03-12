# hrvhra rust Library

## Overview

hrvhra rust is a Rust library designed for analyzing and processing RR intervals and similar data sequences. It provides efficient algorithms and data structures for working with time series, patterns, HRV and complexity parameters.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
runs_rust = "0.1.0"
```

## Usage

```rust
use runs_rust::run_analysis;

fn main() {
    let data = vec![1, 1, 0, 0, 1, 1, 1, 0];
    let analysis = run_analysis::analyze(&data);
    println!("Number of runs: {}", analysis.run_count());
    println!("Longest run: {}", analysis.longest_run());
}
```

### Sample Entropy

The library includes functionality to calculate Sample Entropy (SampEn), which measures the complexity of time series data by quantifying the unpredictability of fluctuations. This library implements Zurek's NCM algorithm:

```rust
use runs_rust::samp_en;

fn main() {
    // Create a time series signal
    let signal = vec![1.2, 1.4, 1.3, 1.5, 1.3, 1.2, 1.4, 1.5];

    // Calculate Sample Entropy with tolerance r = 0.2
    let r = 0.2;
    let entropy = samp_en::calc_samp_en(&signal, r);

    println!("Sample Entropy: {}", entropy);

    // For more detailed analysis, you can compute correlation sums
    let m = 2; // embedding dimension
    let corr_sums = samp_en::calc_correlation_sums(&signal, m, r);
    println!("Correlation Sums: {:?}", corr_sums);
}
```

Sample Entropy is particularly useful for analyzing physiological signals, financial time series, and other complex datasets where measuring randomness and regularity is important.

## Features

- Fast run identification and counting
- Statistical analysis of run distributions
- Support for various data types
- Thread-safe implementations
- Minimal dependencies
- Sample Entropy calculation for time series complexity analysis

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contact

Project Link: [https://github.com/yourusername/runs_rust](https://github.com/yourusername/runs_rust)
