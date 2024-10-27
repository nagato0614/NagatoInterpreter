use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use NagatoInterpreter::Interpreter;

fn parse_arguments() -> File
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    let source_file = &args[1];

    if let Ok(f) = File::open(source_file)
    {
        f
    } else {
        eprintln!("File not found: {}", source_file);
        std::process::exit(1);
    }
}
fn main() {

    let mut source_file = parse_arguments();

    // ファイルを読み込む
    let mut source_code = String::new();
    source_file.read_to_string(&mut source_code).expect("Failed to read file");


    let mut interpreter = Interpreter::new(&source_code);
    interpreter.run();
}
