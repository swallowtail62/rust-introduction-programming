use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

use anyhow::Result;
use clap::Clap;

use crate::calculator::RpnCalculator;
use std::path::PathBuf;

mod calculator;

#[derive(Clap)]
#[clap(
name = "My RPN program",
version = "1.0.0",
author = "swallowtail62",
about = "Super awesome sample RPN calculator",
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("answer: {}", answer),
            Err(e) => eprintln!("err: {:#?}", e),
        }
    }
    Ok(())
}
