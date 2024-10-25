use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use NagatoInterpreter::Interpreter;

fn main() {

    let mut interpreter = Interpreter::new();
    interpreter.run();
}
