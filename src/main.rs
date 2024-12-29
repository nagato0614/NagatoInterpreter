use core::tree_viewer::TreeViewer;
use core::lexical::Lexer;
use core::parser::Parser;
use core::interpreter::Interpreter;

fn main() {
    let program = String::from("
int result[10];
void main()
{   
    int i;
    for (i = 0; i < 10; i = i + 1)
    {
        if (i == 0)
        {
            result[i] = 0;
        }
        else if (i == 1)
        {
            result[i] = 1;
        }
        else
        {
            result[i] = result[i - 1] + result[i - 2];
        }
    }
    return 0;
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
