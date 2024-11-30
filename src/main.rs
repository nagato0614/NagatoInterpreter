use core::lexical::Lexer;
use core::parser::Parser;

fn main() {
    let program = String::from("int a = b();");

    let mut lexer = Lexer::new(program);
    lexer.tokenize();

    lexer.show_tokens();

    println!("-----------");

    let tokens = lexer.tokens().clone();
    let mut parser = Parser::new(tokens);
    parser.parse();

    println!("----------------------");
    parser.show_tree();
}