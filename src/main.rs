use std::env;
use std::fs::File;
use std::io::prelude::*;
use MyLangExecuter::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    let source_file = &args[1];

    let mut f = File::open(source_file).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Failed to read file");

    // 1行読み込む
    let mut line = contents.lines().next().unwrap();
    println!("Line: {}", line);
}
