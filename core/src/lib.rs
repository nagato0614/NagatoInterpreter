pub mod lexical;
pub mod parser;
pub mod interpreter;
pub mod tree_viewer;
pub mod llvm_ir;

#[cfg(test)]
mod test
{
    use std::collections::HashMap;
    use super::*;
    use lexical::Lexer;
    use interpreter::Variable;
    use interpreter::VariableType;

    #[test]
    fn test_static_variable() {
        let input = String::from("
int x = (10 + 20) * 3 - 4 / 2;
float y = (x + 1) * 1.5;
int z = ((x > 15) && (y < 50.0)) || (x == 26);
int u = ((z != 0) && ((y - 10.0) >= 0.0)) + ((x != y) * 2);
float v = ((x + z) * (y - 2.0)) + ((u + 3) / 2);
");

        let mut lexer = Lexer::new(input);
        lexer.tokenize();

        let tokens = lexer.tokens().clone();
        let mut parser = parser::Parser::new(tokens);
        parser.parse();

        let mut interpreter = interpreter::Interpreter::new(parser.roots());
        interpreter.run();

        let variables = interpreter.global_variables();

        // 値の計算:
        // x = (10+20)*3 - 4/2
        //   = 30*3 - 2
        //   = 90 - 2
        //   = 88

        // y = (x+1)*1.5
        //   = (88+1)*1.5
        //   = 89 * 1.5
        //   = 133.5

        // z = ((x>15)&&(y<50.0)) || (x==26)
        //   = ((88>15)&&(133.5<50.0)) || (88==26)
        //   = (true && false) || false
        //   = false || false
        //   = 0 (intで表現)

        // u = ((z!=0)&&((y-10.0)>=0.0)) + ((x!=y)*2)
        //   = ((0!=0)&&((133.5-10.0)>=0.0)) + ((88!=133.5)*2)
        //   = (false && (123.5>=0.0)) + (true*2)
        //   = (false && true) + 2
        //   = false + 2
        //   = 0 + 2
        //   = 2

        // v = ((x+z)*(y-2.0)) + ((u+3)/2)
        //   = ((88+0)*(133.5-2.0)) + ((2+3)/2)
        //   = (88*131.5) + (5/2)
        //   = 88 * 131.5 + 2
        //   = 11572.0 + 2
        //   = 11574.5

        let mut answer = HashMap::new();
        answer.insert("v", Variable::Value(VariableType::Float(11574.0)));
        answer.insert("u", Variable::Value(VariableType::Int(2)));
        answer.insert("z", Variable::Value(VariableType::Int(0)));
        answer.insert("y", Variable::Value(VariableType::Float(133.5)));
        answer.insert("x", Variable::Value(VariableType::Int(88)));

        
        
        for (i, (name, variable)) in variables.iter().enumerate() {
            match answer.get(name.as_str()) {
                Some(ans) => {
                    println!("[{}] {}: {:?}", i, name, variable);
                    assert_eq!(variable, ans);
                }
                None => {
                    panic!("Variable {} is not found", name);
                }
            }
        }
    }

}