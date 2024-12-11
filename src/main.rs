use core::tree_viewer::TreeViewer;
use core::lexical::Lexer;
use core::parser::Parser;
use core::interpreter::Interpreter;

fn main() {
    let program = String::from("
        int add(int a, int b) { return a + b; }
        int main(void) {
            int a = 10;
            int b = 20;
            a = add(a * 2, (b + 10) / 2);
            return a;
        }
        ");
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

    for (i, root) in parser.roots().iter().enumerate() {
        tree_viewer.make_tree(root);
    }
    tree_viewer.output_dot("trees/output.dot");
    
    println!("----------------------");
    let mut interpreter = Interpreter::new(parser.roots());
    interpreter.run();
    interpreter.show_variables();
}
