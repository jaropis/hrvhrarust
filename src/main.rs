use std::cmp;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// defining run types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RunType {
    Dec = 1,  // deceleration run
    Neu = 0,  // neutral run
    Acc = -1, // acceleration run
}

// storing run statistics and addresses
#[derive(Debug, Clone)]
pub struct RunsAccumulator {
    dec: Vec<i32>,                 // storing statistics for deceleration runs
    acc: Vec<i32>,                 // storing statistics for acceleration runs
    neu: Vec<i32>,                 // storing statistics for neutral runs
    runs_addresses: Vec<Vec<i32>>, // storing addresses of runs: [end address, length, type]
}

pub struct RRRuns {
    rr_intervals: Vec<f64>,
    annotations: Vec<i32>,
    write_last_run: bool,
    accumulator: RunsAccumulator,
    analyzed: bool,
}

// implementing rr series struct for file reading
pub struct RRSeries {
    rr: Vec<f64>,
    annot: Vec<i32>,
    column_names: Vec<String>,
    size: usize,
}

impl RRRuns {
    // creating new instance of RRRuns
    pub fn new(rr: Vec<f64>, annot: Vec<i32>, write_last_run: bool) -> Self {
        let size = rr.len();
        let accumulator = RunsAccumulator {
            dec: vec![0; size],
            acc: vec![0; size],
            neu: vec![0; size],
            runs_addresses: Vec::new(),
        };

        RRRuns {
            rr_intervals: rr,
            annotations: annot,
            write_last_run,
            accumulator,
            analyzed: false,
        }
    }

    // getting nonzero length of a vector
    fn get_nonzero_length(&self, vec: &[i32]) -> usize {
        let counter = vec.len();
        for i in (0..counter).rev() {
            if vec[i] != 0 {
                return i + 1;
            }
        }
        0
    }

    // updating runs addresses
    fn update_runs_addresses(&mut self, new_entry: Vec<i32>) {
        self.accumulator.runs_addresses.push(new_entry);
    }

    // analyzing runs
    fn analyze_runs(&mut self) {
        let mut flag_dec = false;
        let mut flag_acc = false;
        let mut flag_neu = false;
        let mut index_dec = 0;
        let mut index_acc = 0;
        let mut index_neu = 0;
        let mut running_rr_number = 0;

        // rewinding to first good flag
        while running_rr_number < self.rr_intervals.len()
            && self.annotations[running_rr_number] != 0
            && self.annotations[running_rr_number + 1] != 0
        {
            running_rr_number += 1;
        }

        // initializing flags
        if self.rr_intervals[running_rr_number] < self.rr_intervals[running_rr_number + 1] {
            flag_dec = true;
            index_dec += 1;
        }
        if self.rr_intervals[running_rr_number] > self.rr_intervals[running_rr_number + 1] {
            flag_acc = true;
            index_acc += 1;
        }
        if self.rr_intervals[running_rr_number] == self.rr_intervals[running_rr_number + 1] {
            flag_neu = true;
            index_neu += 1;
        }
        running_rr_number += 1;
        while running_rr_number < (self.rr_intervals.len() - 1) {
            if self.annotations[running_rr_number + 1] != 0 {
                if flag_dec {
                    self.accumulator.dec[index_dec] += 1;
                    self.update_runs_addresses(vec![
                        running_rr_number as i32,
                        index_dec as i32,
                        RunType::Dec as i32,
                    ]);
                }
                if flag_acc {
                    self.accumulator.acc[index_acc] += 1;
                    self.update_runs_addresses(vec![
                        running_rr_number as i32,
                        index_acc as i32,
                        RunType::Acc as i32,
                    ]);
                }
                if flag_neu {
                    self.accumulator.neu[index_neu] += 1;
                    self.update_runs_addresses(vec![
                        running_rr_number as i32,
                        index_neu as i32,
                        RunType::Neu as i32,
                    ]);
                }

                index_dec = 0;
                index_acc = 0;
                index_neu = 0;
                flag_acc = false;
                flag_dec = false;
                flag_neu = false;

                // rewinding to last bad beat
                while self.annotations[running_rr_number + 1] != 0 {
                    running_rr_number += 1;
                    if running_rr_number >= self.rr_intervals.len() {
                        break;
                    }
                }
                running_rr_number += 1; // rewinding to the first good, which will become the reference
                                        // restarting the flags
                if running_rr_number < self.rr_intervals.len() - 1 {
                    if self.rr_intervals[running_rr_number]
                        < self.rr_intervals[running_rr_number + 1]
                        && self.annotations[running_rr_number + 1] == 0
                    {
                        flag_dec = true;
                        index_dec += 1;
                    }
                    if self.rr_intervals[running_rr_number]
                        > self.rr_intervals[running_rr_number + 1]
                        && self.annotations[running_rr_number + 1] == 0
                    {
                        flag_acc = true;
                        index_acc += 1;
                    }
                    if self.rr_intervals[running_rr_number]
                        == self.rr_intervals[running_rr_number + 1]
                        && self.annotations[running_rr_number + 1] == 0
                    {
                        flag_neu = true;
                        index_neu += 1;
                    }
                }
                running_rr_number += 1; // for the next turn of the loop, because we are continuing
                continue;
            }

            if running_rr_number >= self.rr_intervals.len() - 1 {
                // TODO: Do I need this?
                break;
            }
            let current = self.rr_intervals[running_rr_number];
            let next = self.rr_intervals[running_rr_number + 1];
            if current < next
                && self.annotations[running_rr_number] == 0
                && self.annotations[running_rr_number + 1] == 0
            {
                index_dec += 1;
                if flag_dec {
                    running_rr_number += 1;
                } else {
                    if flag_acc {
                        self.accumulator.acc[index_acc] += 1;
                        self.update_runs_addresses(vec![
                            running_rr_number as i32 - 1, // -1 because at i a new run starts, so the previous ends at -1
                            index_acc as i32,
                            RunType::Acc as i32,
                        ]);
                        running_rr_number += 1;
                        index_acc = 0;
                        flag_acc = false;
                        flag_dec = true;
                    }
                    if flag_neu {
                        self.accumulator.neu[index_neu] += 1;
                        self.update_runs_addresses(vec![
                            running_rr_number as i32 - 1,
                            index_neu as i32,
                            RunType::Neu as i32,
                        ]);
                        running_rr_number += 1;
                        index_neu = 0;
                        flag_neu = false;
                        flag_dec = true;
                    }
                }
            }

            if current > next
                && self.annotations[running_rr_number] == 0
                && self.annotations[running_rr_number + 1] == 0
            {
                index_acc += 1;
                if flag_acc {
                    running_rr_number += 1;
                } else {
                    if flag_dec {
                        self.accumulator.dec[index_dec] += 1;
                        self.update_runs_addresses(vec![
                            running_rr_number as i32 - 1,
                            index_dec as i32,
                            RunType::Dec as i32,
                        ]);
                        running_rr_number += 1;
                        index_dec = 0;
                        flag_dec = false;
                        flag_acc = true;
                    }
                    if flag_neu {
                        self.accumulator.neu[index_neu] += 1;
                        self.update_runs_addresses(vec![
                            running_rr_number as i32 - 1,
                            index_neu as i32,
                            RunType::Neu as i32,
                        ]);
                        running_rr_number += 1;
                        index_neu = 0;
                        flag_neu = false;
                        flag_acc = true;
                    }
                }
            }

            if current == next
                && self.annotations[running_rr_number] == 0
                && self.annotations[running_rr_number + 1] == 0
            {
                index_neu += 1;
                if flag_neu {
                    running_rr_number += 1;
                } else {
                    if flag_dec {
                        self.accumulator.dec[index_dec] += 1;
                        self.update_runs_addresses(vec![
                            running_rr_number as i32 - 1,
                            index_dec as i32,
                            RunType::Dec as i32,
                        ]);
                        running_rr_number += 1;
                        index_dec = 0;
                        flag_dec = false;
                        flag_neu = true;
                    }
                    if flag_acc {
                        self.accumulator.acc[index_acc] += 1;
                        self.update_runs_addresses(vec![
                            running_rr_number as i32 - 1,
                            index_acc as i32,
                            RunType::Acc as i32,
                        ]);
                        running_rr_number += 1;
                        index_acc = 0;
                        flag_acc = false;
                        flag_neu = true;
                    }
                }
            }
        }

        // writing last run if needed
        if self.write_last_run {
            if index_acc > 0 {
                self.accumulator.acc[index_acc] += 1;
                self.update_runs_addresses(vec![
                    running_rr_number as i32 + 1, // +1 i loops from running_rr_number + 1, so the loop ends at running_rr_number - 1
                    index_acc as i32,
                    RunType::Acc as i32,
                ]);
            }
            if index_dec > 0 {
                self.accumulator.dec[index_dec] += 1;
                self.update_runs_addresses(vec![
                    running_rr_number as i32 + 1,
                    index_dec as i32,
                    RunType::Dec as i32,
                ]);
            }
            if index_neu > 0 {
                self.accumulator.neu[index_neu] += 1;
                self.update_runs_addresses(vec![
                    running_rr_number as i32 + 1,
                    index_neu as i32,
                    RunType::Neu as i32,
                ]);
            }
        } else {
            println!("the last run not needed");
        }

        self.analyzed = true;
    }

    // getting full runs
    pub fn get_full_runs(&mut self) -> &RunsAccumulator {
        if !self.analyzed {
            self.analyze_runs();
        }
        &self.accumulator
    }

    // printing runs
    pub fn print_runs(&mut self) {
        if !self.analyzed {
            self.analyze_runs();
        }

        let dec_size = self.get_nonzero_length(&self.accumulator.dec);
        let acc_size = self.get_nonzero_length(&self.accumulator.acc);
        let neu_size = self.get_nonzero_length(&self.accumulator.neu);
        //println!("ful neu accumulator size: {:?}", self.accumulator.neu);
        let max_length = cmp::max(cmp::max(acc_size, dec_size), neu_size);

        println!("i  Ar - DR - N");
        for i in 1..max_length {
            println!(
                "{} {} - {} - {}",
                i,
                if i < acc_size {
                    self.accumulator.acc[i]
                } else {
                    0
                },
                if i < dec_size {
                    self.accumulator.dec[i]
                } else {
                    0
                },
                if i < neu_size {
                    self.accumulator.neu[i]
                } else {
                    0
                }
            );
        }
    }

    // printing addresses
    pub fn print_addresses(&mut self, run_type: RunType, run_length: i32, reference_beat: bool) {
        println!(
            "run type: {} run length: {}",
            match run_type {
                RunType::Dec => "DEC",
                RunType::Acc => "ACC",
                _ => "NEU",
            },
            run_length
        );

        if !self.analyzed {
            self.analyze_runs();
        }

        let reference_offset = if reference_beat { 1 } else { 0 };
        for run in &self.accumulator.runs_addresses {
            if run[2] == run_type as i32 && run[1] == run_length {
                let end_idx = run[0] as usize + reference_beat as usize;
                let length = (run[1] + reference_offset) as usize;
                if length <= end_idx + 1 {
                    let start_idx = end_idx - length;
                    println!("start_idx: {}, end_idx: {}", start_idx, end_idx);
                    for idx in start_idx..=end_idx {
                        // inclusive range
                        print!("{} ", self.rr_intervals[idx]);
                    }
                    println!();
                }
            }
        }
    }
}

// implementing file reading functionality
impl RRSeries {
    pub fn read_rr(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // reading header line
        let header = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "empty file"))??;
        let column_names: Vec<String> = header.split_whitespace().map(String::from).collect();

        let mut rr = Vec::new();
        let mut annot = Vec::new();

        // processing data rows
        for line in lines {
            let line = line?;
            let values: Vec<&str> = line.split_whitespace().collect();
            if values.len() >= 2 {
                rr.push(values[0].parse::<f64>().unwrap());
                annot.push(values[1].parse::<i32>().unwrap());
            }
        }

        Ok(RRSeries {
            size: rr.len(),
            column_names,
            rr,
            annot,
        })
    }
}

// fn main() -> io::Result<()> {
//     // example usage with hardcoded data
//     let rr_data = vec![1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0];
//     let annot_data = vec![0, 0, 1, 0, 0, 1, 0, 0];

//     // let rr_data = vec![   1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0];
//     // let annot_data = vec![0,   0,   1,   0,   0,   1,   0,   0];

//     let mut rr = RRRuns::new(rr_data, annot_data, true);
//     rr.print_runs();

//     rr.print_addresses(RunType::Dec, 1, false);
//     rr.print_addresses(RunType::Acc, 1, false);
//     //rr.print_addresses(RunType::Acc, 1, true);

//     Ok(())
// }

fn main() -> io::Result<()> {
    // reading from file
    let rr_series = RRSeries::read_rr("test1.csv")?;
    let mut rr = RRRuns::new(rr_series.rr, rr_series.annot, true);

    // Get and print the full analysis
    rr.get_full_runs();
    rr.print_runs();
    println!("expected output:\n1 2 0");

    let rr_series = RRSeries::read_rr("test2.csv")?;
    let mut rr = RRRuns::new(rr_series.rr, rr_series.annot, true);

    // Get and print the full analysis
    rr.get_full_runs();
    rr.print_runs();

    //rr.print_addresses(RunType::Neu, 2, true);
    println!("expected output: \n2 2 1\n 0 0 1");

    let rr_series = RRSeries::read_rr("test3.csv")?;
    let mut rr = RRRuns::new(rr_series.rr, rr_series.annot, true);

    // Get and print the full analysis
    rr.get_full_runs();
    rr.print_runs();

    println!("expected output: \n1 1 1\n 0 0 1");
    // rr.print_addresses(RunType::Dec, 2, true);
    Ok(())
}
