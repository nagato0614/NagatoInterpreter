use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use anyhow::{bail, ensure, Context, Result};
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[clap(
name = "My Lang Execute",
version = "0.0.1",
author = "nagai toru",
about = "my lang parser"
)]
struct Program
{
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>,
}

fn main() {
    println!("Hello, world!");
}
