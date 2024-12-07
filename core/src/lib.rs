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
    fn test_static_variables()
    {
        let input = String::from("
int a = 5;
int b = a + 3;
float f = b * 3.14;
int c = ((b + a) * 2) || (5 > 3);
float g = (f + 2) && (b < a);
int h = (b == a) + (c != 0);
");

        let mut lexer = Lexer::new(input);
        lexer.tokenize();

        let tokens = lexer.tokens().clone();
        let mut parser = parser::Parser::new(tokens);
        parser.parse();

        let mut interpreter = interpreter::Interpreter::new(parser.roots());
        interpreter.run();

        let variables = interpreter.variables();

        // 答え
        let answer = vec![
            Value::new("a", VariableType::Int(5)),
            Value::new("b", VariableType::Int(8)),
            Value::new("f", VariableType::Float(25.12)),
            Value::new("c", VariableType::Int(1)),
            Value::new("g", VariableType::Int(1)),
            Value::new("h", VariableType::Int(2)),
        ];

        for (index, v) in variables.iter().enumerate() {
            match v
            {
                Variable::Value(val) =>
                    {
                        assert_eq!(val.name(), answer[index].name());
                        assert_eq!(val.value(), answer[index].value());
                    }
                _ =>
                    {
                        assert!(false);
                    }
            }
        }
    }
}