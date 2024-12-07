use core::tree_viewer::TreeViewer;
use core::lexical::Lexer;
use core::parser::Parser;
use core::interpreter::Interpreter;

fn main() {
    let program = String::from("int main(void) { return 0; }");

    let mut lexer = Lexer::new(program);
    lexer.tokenize();

    lexer.show_tokens();

    println!("----------------------");

    let tokens = lexer.tokens().clone();
    let mut parser = Parser::new(tokens);
    parser.parse();

    println!("----------------------");
    parser.show_tree();


    let mut tree_viewer = TreeViewer::new();
    tree_viewer.make_tree(parser.root());
    tree_viewer.output_dot();

    println!("----------------------");
    let mut interpreter = Interpreter::new(parser.roots());
    interpreter.run();
    interpreter.show_variables();
}
