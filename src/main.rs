use core::tree_viewer::TreeViewer;
use core::lexical::Lexer;
use core::parser::Parser;
use core::interpreter::Interpreter;

fn main() {
    let program = String::from("
#define N 10
#define S sum
int main(void) {
    int count = N; // こめんと
    int S = 0;

    /* ブロックコメント */

    /// これもコメント
    while (count > 0) {
        S = S + count;
        count = count - 1;
    }
    return sum;
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
    // 時間計測スタート
    let start = std::time::Instant::now();
    let val = interpreter.run();
    // 時間計測終了
    let end = std::time::Instant::now();
    interpreter.show_variables();
    println!("----------------------");

    println!("calculation time: {:?}", end.duration_since(start));
    println!("result: {:?}", val);
    let answer = 75025;
}
