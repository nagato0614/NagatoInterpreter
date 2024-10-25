use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use NagatoInterpreter::*;

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


    for line in contents.lines() {
        println!("Line: {}", line);

        // 1行をトークンに分割する
        let tokens = parse_line(line);

        // 文字列をtoken型の列に変換する
        let tokens: Vec<Token> = tokens.iter().map(|token| convert_token(token)).collect();

        // トークン列が空の場合は次の行へ
        if tokens.len() == 0 {
            continue;
        }

        run_line(tokens);
    }
}
