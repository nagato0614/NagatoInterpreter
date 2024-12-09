use core::tree_viewer::TreeViewer;
use core::lexical::Lexer;
use core::parser::Parser;
use core::interpreter::Interpreter;

fn main() {
    // let program = String::from(
    //     "int main(void) { \
    //         int a = 10;\
    //         int b = 20;\
    //         a = a + b;\
    //         return a;\
    //     }\
    //     int add(int a, int b) { return a + b; }");
    let program = String::from("
int x = (10 + 20) * 3 - 4 / 2;
float y = (x + 1) * 1.5;
int z = ((x > 15) && (y < 50.0)) || (x == 26);
int u = ((z != 0) && ((y - 10.0) >= 0.0)) + ((x != y) * 2);
float v = ((x + z) * (y - 2.0)) + ((u + 3) / 2);
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
