pub mod lexical;
pub mod parser;
pub mod interpreter;
pub mod tree_viewer;

#[cfg(test)]
mod test
{
    use super::*;
    use lexical::Lexer;
    use interpreter::Variable;
    use interpreter::VariableType;
    use interpreter::Value;

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

        let variables = interpreter.variables();

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
        //   = 88 * 131.5 + 2.5
        //   = 11572.0 + 2.5
        //   = 11574.5

        let answer = vec![
            Value::new("x", VariableType::Int(88)),
            Value::new("y", VariableType::Float(133.5)),
            Value::new("z", VariableType::Int(0)),
            Value::new("u", VariableType::Int(2)),
            Value::new("v", VariableType::Float(11574.5)),
        ];

        for (index, v) in variables.iter().enumerate() {
            match v {
                Variable::Value(val) => {
                    assert_eq!(val.name(), answer[index].name());
                    assert_eq!(val.value(), answer[index].value());
                }
                _ => {
                    panic!("Unexpected variable type");
                }
            }
        }
    }

}