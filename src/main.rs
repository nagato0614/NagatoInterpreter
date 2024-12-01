use core::tree_viewer::TreeViewer;
use core::lexical::Lexer;
use core::parser::Parser;

fn main() {
    let program = String::from("int a = -(1 + 2 * 3 / 4) + 5 || 1 + b(1);");

    let mut lexer = Lexer::new(program);
    lexer.tokenize();

    lexer.show_tokens();

    println!("-----------");

    let tokens = lexer.tokens().clone();
    let mut parser = Parser::new(tokens);
    parser.parse();

    println!("----------------------");
    parser.show_tree();


    let mut tree_viewer = TreeViewer::new();
    tree_viewer.make_tree(parser.root());
    tree_viewer.output_dot();
}
