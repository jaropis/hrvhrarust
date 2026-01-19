# Rust Modules and Crates: A Practical Tutorial

## Using the `runs.rs` Refactoring as a Learning Example

This tutorial will teach you how Rust organizes code using **crates** and **modules**, using the refactoring of `runs.rs` as a hands-on example.

---

## Table of Contents

1. [Crates vs Modules: The Big Picture](#1-crates-vs-modules-the-big-picture)
2. [How Modules Work](#2-how-modules-work)
3. [Visibility: `pub` and Privacy](#3-visibility-pub-and-privacy)
4. [Two Ways to Organize Modules](#4-two-ways-to-organize-modules)
5. [Step-by-Step: Splitting `runs.rs`](#5-step-by-step-splitting-runsrs)
6. [The `use` Keyword and Paths](#6-the-use-keyword-and-paths)
7. [Common Patterns and Best Practices](#7-common-patterns-and-best-practices)
8. [Adding and Using External Crates](#8-adding-and-using-external-crates)

---

## 1. Crates vs Modules: The Big Picture

### What is a Crate?

A **crate** is Rust's compilation unit—the smallest amount of code the Rust compiler considers at a time. There are two types:

| Type              | Description                    | Entry Point   |
| ----------------- | ------------------------------ | ------------- |
| **Binary crate**  | An executable program          | `src/main.rs` |
| **Library crate** | Code meant to be shared/reused | `src/lib.rs`  |

Your project `hrvhrarust` is a **library crate** because it has `src/lib.rs` as its root.

```shell
hrvhrarust/           # This is ONE crate
├── Cargo.toml        # Crate configuration
└── src/
    ├── lib.rs        # Crate root (entry point)
    ├── runs.rs       # A module
    ├── samp_en.rs    # Another module
    └── data_reader.rs
```

### What is a Module?

A **module** is a way to organize code _within_ a crate. Modules:

- Group related functionality together
- Control visibility (public vs private)
- Create namespaces to avoid name collisions

Think of it this way:

- **Crate** = a book (the whole compilation unit)
- **Module** = a chapter (organizational unit within the book)

---

## 2. How Modules Work

### Declaring Modules

In Rust, you must explicitly declare modules. In your `lib.rs`:

```rust
// lib.rs
pub mod data_reader;  // Declares a module named "data_reader"
pub mod runs;         // Declares a module named "runs"
pub mod samp_en;      // Declares a module named "samp_en"
```

The `mod` keyword tells Rust: "Look for this module's code somewhere."

### Where Does Rust Look for Module Code?

When you write `mod runs;`, Rust looks in two places (in order):

1. **`runs.rs`** - A file named after the module
2. **`runs/mod.rs`** - A directory with a `mod.rs` file inside

```
src/
├── lib.rs          # contains: mod runs;
├── runs.rs         # Option 1: Single file
└── runs/           # Option 2: Directory
    └── mod.rs      # Module root
```

**You cannot have both `runs.rs` AND `runs/` directory—Rust will complain!**

### Inline Modules

You can also define modules directly in a file:

```rust
// lib.rs
mod my_inline_module {
    pub fn hello() {
        println!("Hello from inline module!");
    }
}
```

This is useful for small, tightly-coupled code but doesn't help with file organization.

---

## 3. Visibility: `pub` and Privacy

### The Default is Private

In Rust, **everything is private by default**. This is different from many languages!

```rust
// runs.rs
struct RRRuns { ... }          // Private - only visible within this module
pub struct RunType { ... }     // Public - visible to other modules
```

### Visibility Levels

| Syntax         | Meaning                                            |
| -------------- | -------------------------------------------------- |
| (nothing)      | Private to the current module                      |
| `pub`          | Public to everyone                                 |
| `pub(crate)`   | Public within the crate, but not to external users |
| `pub(super)`   | Public to the parent module                        |
| `pub(in path)` | Public to a specific module path                   |

### Why `pub(crate)` Matters for Splitting Files

When you split a struct's implementation across multiple files in the same crate, the files need access to the struct's fields. But you might not want external users to access those fields directly.

```rust
// runs/mod.rs
pub struct RRRuns {
    pub(crate) rr_intervals: Vec<f64>,  // Accessible within the crate
    pub(crate) analyzed: bool,          // But not to external users
}
```

---

## 4. Two Ways to Organize Modules

### Option A: Single File Module

Simple and good for small modules:

```shell
src/
├── lib.rs
└── runs.rs    # All code for the "runs" module
```

### Option B: Directory Module (What We'll Use)

Better for larger modules that need to be split:

```
src/
├── lib.rs
└── runs/
    ├── mod.rs        # Module root - declares submodules
    ├── types.rs      # Submodule for types
    ├── analysis.rs   # Submodule for analysis logic
    └── output.rs     # Submodule for output functions
```

In `runs/mod.rs`:

```rust
mod types;      // Private submodule
mod analysis;   // Private submodule
mod output;     // Private submodule

// Re-export what should be public
pub use types::{RunType, RunsAccumulator};
```

---

## 5. Step-by-Step: Splitting `runs.rs`

Let's split your `runs.rs` (currently ~450 lines) into logical pieces.

### Step 1: Plan the Split

Looking at your code, I see these logical groupings:

| File          | Contents                                | ~Lines |
| ------------- | --------------------------------------- | ------ |
| `mod.rs`      | Types, struct definitions, constructor  | 70     |
| `analysis.rs` | `analyze_runs()`, the core algorithm    | 200    |
| `output.rs`   | All `print_*` methods                   | 80     |
| `summary.rs`  | `get_runs_summary()`, `get_full_runs()` | 50     |
| `variance.rs` | `calculate_runs_variances()`            | 40     |

### Step 2: Create the Directory Structure

```bash
# From your project root
mkdir src/runs
touch src/runs/mod.rs
touch src/runs/analysis.rs
touch src/runs/output.rs
touch src/runs/summary.rs
touch src/runs/variance.rs
```

### Step 3: Move Types to `mod.rs`

```rust
// src/runs/mod.rs

// Declare submodules (Rust will look for analysis.rs, output.rs, etc.)
mod analysis;
mod output;
mod summary;
mod variance;

use std::collections::HashMap;

// ============== Types ==============

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunType {
    Dec = 1,
    Neu = 0,
    Acc = -1,
}

#[derive(Debug, Clone)]
pub struct RunsAccumulator {
    // pub(crate) means: public within this crate, private to outside
    pub(crate) dec: HashMap<usize, i32>,
    pub(crate) acc: HashMap<usize, i32>,
    pub(crate) neu: HashMap<usize, i32>,
    pub(crate) runs_addresses: Vec<Vec<i32>>,
}

pub struct RRRuns {
    pub(crate) rr_intervals: Vec<f64>,
    pub(crate) mean_rr: f64,
    pub(crate) rr_length: usize,
    pub(crate) annotations: Vec<i32>,
    pub(crate) write_last_run: bool,
    pub(crate) accumulator: RunsAccumulator,
    pub(crate) runs_variances: HashMap<RunType, Vec<f64>>,
    pub(crate) analyzed: bool,
    pub(crate) max_dec: usize,
    pub(crate) max_acc: usize,
    pub(crate) max_neu: usize,
}

// ============== Constructor ==============

impl RRRuns {
    pub fn new(rr: Vec<f64>, annot: Vec<i32>, write_last_run: bool) -> Self {
        let size = rr.len();
        let accumulator = RunsAccumulator {
            dec: HashMap::new(),
            acc: HashMap::new(),
            neu: HashMap::new(),
            runs_addresses: Vec::new(),
        };
        let runs_variances: HashMap<RunType, Vec<f64>> = HashMap::new();
        let mean_rr: f64 = rr.iter().sum::<f64>() / size as f64;

        RRRuns {
            rr_intervals: rr,
            mean_rr,
            rr_length: size,
            annotations: annot,
            runs_variances,
            write_last_run,
            accumulator,
            analyzed: false,
            max_acc: 0,
            max_dec: 0,
            max_neu: 0,
        }
    }

    // Helper that other modules need
    pub(crate) fn get_nonzero_length(&self, map: &HashMap<usize, i32>) -> usize {
        map.keys().max().copied().unwrap_or(0)
    }

    pub(crate) fn update_runs_addresses(&mut self, new_entry: Vec<i32>) {
        self.accumulator.runs_addresses.push(new_entry);
    }

    pub(crate) fn set_max(&mut self) {
        self.max_dec = self.get_nonzero_length(&self.accumulator.dec);
        self.max_acc = self.get_nonzero_length(&self.accumulator.acc);
        self.max_neu = self.get_nonzero_length(&self.accumulator.neu);
    }
}
```

### Step 4: Move Analysis to `analysis.rs`

```rust
// src/runs/analysis.rs

use super::{RRRuns, RunType};  // Import from parent module (mod.rs)

// This is a SEPARATE impl block for the SAME struct
// Rust allows this - even across files!
impl RRRuns {
    pub(crate) fn analyze_runs(&mut self) {
        let mut flag_dec = false;
        let mut flag_acc = false;
        let mut flag_neu = false;
        // ... rest of the analyze_runs code ...

        self.set_max();
        self.analyzed = true;
    }
}
```

**Key insight:** The `super` keyword means "parent module". Since `analysis.rs` is inside the `runs/` directory, its parent is `runs/mod.rs`.

### Step 5: Move Output Functions to `output.rs`

```rust
// src/runs/output.rs

use super::{RRRuns, RunType};
use std::cmp;

impl RRRuns {
    pub fn print_runs(&mut self) {
        if !self.analyzed {
            self.analyze_runs();
        }
        // ... rest of print_runs ...
    }

    pub fn print_addresses(&mut self, run_type: RunType, run_length: i32, reference_beat: bool) {
        // ... code ...
    }

    pub fn print_runs_addresses(&self) {
        // ... code ...
    }

    pub fn print_runs_accumulator(&self) {
        // ... code ...
    }
}
```

### Step 6: Move Summary Functions to `summary.rs`

```rust
// src/runs/summary.rs

use super::{RRRuns, RunsAccumulator};
use std::cmp;

impl RRRuns {
    pub fn get_runs_summary(&mut self) -> Vec<Vec<i32>> {
        if !self.analyzed {
            self.analyze_runs();
        }
        // ... rest of get_runs_summary ...
    }

    pub fn get_full_runs(&mut self) -> &RunsAccumulator {
        if !self.analyzed {
            self.analyze_runs();
        }
        &self.accumulator
    }
}
```

### Step 7: Move Variance Functions to `variance.rs`

```rust
// src/runs/variance.rs

use super::{RRRuns, RunType};

impl RRRuns {
    pub fn calculate_runs_variances(&mut self) {
        if !self.analyzed {
            self.analyze_runs();
        }
        // ... rest of calculate_runs_variances ...
    }

    pub fn print_runs_variances(&self) {
        println!("{:?}", self.runs_variances)
    }
}
```

### Step 8: Delete the Old `runs.rs`

```bash
rm src/runs.rs
```

### Step 9: Verify `lib.rs` (No Changes Needed!)

```rust
// src/lib.rs
pub mod data_reader;
pub mod runs;        // Now points to runs/mod.rs automatically
pub mod samp_en;
```

The beauty is that `lib.rs` doesn't change at all. Rust automatically looks for `runs/mod.rs` when `runs.rs` doesn't exist.

---

## 6. The `use` Keyword and Paths

### Absolute vs Relative Paths

```rust
// Absolute path (from crate root)
use crate::runs::RunType;

// Relative path using `super` (parent module)
use super::RunType;

// Relative path using `self` (current module)
use self::submodule::Something;
```

### The `super` Keyword

`super` refers to the parent module. It's like `..` in file paths:

```
runs/
├── mod.rs           # This is the "runs" module
├── analysis.rs      # super:: refers to mod.rs
└── output.rs        # super:: refers to mod.rs
```

In `analysis.rs`:

```rust
use super::RRRuns;   // Gets RRRuns from runs/mod.rs
use super::RunType;  // Gets RunType from runs/mod.rs
```

### Re-exporting with `pub use`

Sometimes you want to expose items from submodules at a higher level:

```rust
// runs/mod.rs
mod types;  // private module

// Re-export so users can do `runs::RunType` instead of `runs::types::RunType`
pub use types::RunType;
```

---

## 7. Common Patterns and Best Practices

### Pattern 1: The Facade Pattern

Keep implementation details private, expose a clean API:

```rust
// runs/mod.rs
mod analysis;   // Private - users can't access directly
mod output;     // Private

pub use types::{RunType, RRRuns};  // Only expose what users need
```

### Pattern 2: Prelude Module

For libraries with many exports, create a prelude:

```rust
// lib.rs
pub mod prelude {
    pub use crate::runs::{RRRuns, RunType};
    pub use crate::samp_en::SampleEntropy;
}

// Users can then do:
// use hrvhrarust::prelude::*;
```

### Pattern 3: Testing in Modules

Each module can have its own tests:

```rust
// runs/analysis.rs

impl RRRuns {
    pub(crate) fn analyze_runs(&mut self) {
        // ... implementation ...
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_runs() {
        // Test code here
    }
}
```

### Best Practices Summary

1. **One concept per file** - If a file does too many things, split it
2. **Use `pub(crate)` generously** - It's the right visibility for internal helpers
3. **Keep the public API small** - Only `pub` what users actually need
4. **Group related items** - Types with their implementations
5. **Use re-exports** - Clean up your public API with `pub use`

---

## 8. Adding and Using External Crates

Now let's say you want to add a new crate called `hra` that provides HRA (Heart Rate Asymmetry) calculations, and you want to use its functions (`sd1a`, `sd1d`, `sd2a`, `sd2d`) in your `variance.rs` module.

### Crate vs Module: Which is `hra`?

This depends on where the code lives:

| Scenario                                        | What `hra` is      | How to use it              |
| ----------------------------------------------- | ------------------ | -------------------------- |
| Code lives in a **separate repository/package** | External **crate** | Add to `Cargo.toml`        |
| Code lives in **your same project**             | **Module**         | Add `mod hra;` to `lib.rs` |

Let's explore both options.

---

### Option A: `hra` as an External Crate (Separate Package)

This is the right choice if:

- The HRA code is reusable across multiple projects
- It's published to crates.io or a private registry
- It lives in a separate Git repository

#### Step 1: Add the Dependency to `Cargo.toml`

```toml
# Cargo.toml
[package]
name = "hrvhrarust"
version = "0.1.0"
edition = "2021"

[dependencies]
hra = "0.1.0"                           # From crates.io
# OR
hra = { path = "../hra" }               # Local path to another crate
# OR
hra = { git = "https://github.com/you/hra" }  # From Git
```

#### Step 2: Use it in `variance.rs`

```rust
// src/runs/variance.rs

use super::{RRRuns, RunType};
use hra::{sd1a, sd1d, sd2a, sd2d};  // Import from external crate

impl RRRuns {
    pub fn calculate_runs_variances(&mut self) {
        if !self.analyzed {
            self.analyze_runs();
        }

        // Now you can use the hra functions
        let asymmetry_1a = sd1a(&self.rr_intervals);
        let asymmetry_1d = sd1d(&self.rr_intervals);
        // ... etc
    }
}
```

#### The `hra` Crate Structure

The external `hra` crate would look like:

```
hra/                      # Separate project folder
├── Cargo.toml
└── src/
    └── lib.rs            # Exports sd1a, sd1d, sd2a, sd2d
```

```rust
// hra/src/lib.rs
pub fn sd1a(rr_intervals: &[f64]) -> f64 {
    // implementation
}

pub fn sd1d(rr_intervals: &[f64]) -> f64 {
    // implementation
}

pub fn sd2a(rr_intervals: &[f64]) -> f64 {
    // implementation
}

pub fn sd2d(rr_intervals: &[f64]) -> f64 {
    // implementation
}
```

---

### Option B: `hra` as a Module (Same Project)

This is the right choice if:

- The HRA code is tightly coupled to this project
- You don't need to share it with other projects
- You want simpler project management

#### Step 1: Create the Module

```
src/
├── lib.rs
├── hra.rs              # New module (or hra/mod.rs for directory)
├── data_reader.rs
├── samp_en.rs
└── runs/
    └── ...
```

#### Step 2: Declare it in `lib.rs`

```rust
// src/lib.rs
pub mod data_reader;
pub mod runs;
pub mod samp_en;
pub mod hra;            // Add this line
```

#### Step 3: Implement the Module

```rust
// src/hra.rs
pub fn sd1a(rr_intervals: &[f64]) -> f64 {
    // implementation
}

pub fn sd1d(rr_intervals: &[f64]) -> f64 {
    // implementation
}

pub fn sd2a(rr_intervals: &[f64]) -> f64 {
    // implementation
}

pub fn sd2d(rr_intervals: &[f64]) -> f64 {
    // implementation
}
```

#### Step 4: Use it in `variance.rs`

```rust
// src/runs/variance.rs

use super::{RRRuns, RunType};
use crate::hra::{sd1a, sd1d, sd2a, sd2d};  // Import from sibling module

impl RRRuns {
    pub fn calculate_runs_variances(&mut self) {
        if !self.analyzed {
            self.analyze_runs();
        }

        // Now you can use the hra functions
        let asymmetry_1a = sd1a(&self.rr_intervals);
        let asymmetry_1d = sd1d(&self.rr_intervals);
        // ... etc
    }
}
```

**Key difference:** For a module within the same crate, you use `crate::hra` (absolute path from crate root) instead of just `hra`.

---

### Option C: Workspace with Multiple Crates (Best of Both Worlds)

If you want `hra` to be a separate crate but still live in the same repository, use a **Cargo workspace**:

```
hrvhra-workspace/           # Workspace root
├── Cargo.toml              # Workspace manifest
├── hrvhrarust/             # Your existing crate
│   ├── Cargo.toml
│   └── src/
│       └── ...
└── hra/                    # New crate in same repo
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

```toml
# hrvhra-workspace/Cargo.toml (workspace root)
[workspace]
members = ["hrvhrarust", "hra"]
```

```toml
# hrvhra-workspace/hrvhrarust/Cargo.toml
[package]
name = "hrvhrarust"
version = "0.1.0"
edition = "2021"

[dependencies]
hra = { path = "../hra" }   # Reference sibling crate
```

This gives you:

- **Separate compilation** - Changes to `hra` don't recompile `hrvhrarust` unnecessarily
- **Shared dependencies** - Workspace members share a single `Cargo.lock`
- **Independent versioning** - Each crate can have its own version
- **Optional publishing** - You can publish them separately to crates.io

---

### Which Option Should You Choose?

| Choose...          | When...                                         |
| ------------------ | ----------------------------------------------- |
| **External crate** | HRA logic is general-purpose and reusable       |
| **Module**         | HRA is specific to this project, simpler setup  |
| **Workspace**      | You want separation but single-repo convenience |

For your case, if `sd1a`, `sd1d`, `sd2a`, `sd2d` are standard HRA calculations that could be useful in other heart rate analysis projects, I'd recommend **Option C (Workspace)** — it keeps things organized while allowing future reuse.

If it's just helper code specific to this analysis, **Option B (Module)** is simpler and perfectly fine.

---

## Quick Reference

| Concept                | Syntax                    | Meaning                           |
| ---------------------- | ------------------------- | --------------------------------- |
| Declare module         | `mod foo;`                | Look for `foo.rs` or `foo/mod.rs` |
| Public item            | `pub fn`                  | Visible outside module            |
| Crate-public           | `pub(crate) fn`           | Visible within crate only         |
| Import from parent     | `use super::Item`         | Get `Item` from parent module     |
| Import from crate root | `use crate::module::Item` | Absolute path                     |
| Re-export              | `pub use inner::Item`     | Expose nested item at this level  |

---

## Final Directory Structure

After splitting:

```
src/
├── lib.rs              # Crate root (unchanged)
├── data_reader.rs
├── samp_en.rs
└── runs/               # Module directory
    ├── mod.rs          # Types, constructor, declares submodules
    ├── analysis.rs     # analyze_runs()
    ├── output.rs       # print_* methods
    ├── summary.rs      # get_runs_summary(), get_full_runs()
    └── variance.rs     # calculate_runs_variances()
```

Each file is focused, readable, and maintainable!
