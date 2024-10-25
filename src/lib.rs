use std::collections::HashMap;
use std::fmt;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

// 定数
const PLUS: &str = "+";
const MINUS: &str = "-";
const MULTIPLY: &str = "*";
const DIVIDE: &str = "/";
const MOD: &str = "%";
const EQUAL: &str = "=";
const LEFT_PAREN: &str = "(";
const RIGHT_PAREN: &str = ")";

#[derive(Debug, Clone)]
enum ArithmeticOperandTail
{
    Multiply,
    Divide,
    Mod,
    None,
}

#[derive(Debug, Clone)]
enum ArithmeticOperandHead
{
    Plus,
    Minus,
    None,
}

#[derive(Debug, Clone)]
enum ArithmeticOperandParen
{
    Left,
    Right,
    None,
}

#[derive(Debug, Clone)]
enum Token
{
    Value(i32),
    OperatorHead(ArithmeticOperandHead),
    OperatorTail(ArithmeticOperandTail),
    Variable(String),
    OperatorParen(ArithmeticOperandParen),
    Assign,
    None,
}

impl fmt::Display for Token
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Token::Value(val) => write!(f, "{}", val),
            Token::OperatorHead(op) => match op
            {
                ArithmeticOperandHead::Plus => write!(f, "{}", PLUS),
                ArithmeticOperandHead::Minus => write!(f, "{}", MINUS),
                _ => write!(f, "None"),
            },
            Token::OperatorTail(op) => match op
            {
                ArithmeticOperandTail::Multiply => write!(f, "{}", MULTIPLY),
                ArithmeticOperandTail::Divide => write!(f, "{}", DIVIDE),
                ArithmeticOperandTail::Mod => write!(f, "{}", MOD),
                _ => write!(f, "None"),
            },
            Token::OperatorParen(op) => match op
            {
                ArithmeticOperandParen::Left => write!(f, "{}", LEFT_PAREN),
                ArithmeticOperandParen::Right => write!(f, "{}", RIGHT_PAREN),
                _ => write!(f, "None"),
            },
            Token::Variable(var) => write!(f, "{}", var),
            Token::Assign => write!(f, "{}", EQUAL),
            Token::None => write!(f, "None"),
        }
    }
}


pub struct Interpreter
{
    variables: HashMap<String, i32>,
    contents: String,
}


impl Interpreter
{
    pub fn new() -> Self
    {
        Interpreter
        {
            variables: HashMap::new(),
            contents: String::new(),
        }
    }

    fn parse_arguments(&mut self) -> File
    {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            eprintln!("Usage: {} <source_file>", args[0]);
            std::process::exit(1);
        }

        let source_file = &args[1];

        if let Ok(f) = File::open(source_file)
        {
            f
        } else {
            eprintln!("File not found: {}", source_file);
            std::process::exit(1);
        }
    }


    pub fn run(&mut self)
    {
        let mut f = self.parse_arguments();


        f.read_to_string(&mut self.contents).expect("Failed to read file");


        self.run_interpreter();
    }

    fn run_interpreter(&mut self)
    {
        let lines: Vec<String> = self.contents.lines().map(
            |line| line.to_string()
        ).collect();
        for line in lines {
            // 1行をトークンに分割する
            let tokens = &mut self.parse_line(line.as_str());

            // 文字列をtoken型の列に変換する
            let mut tokens: Vec<Token> = tokens.iter().map(|token| self.convert_token(token))
                .collect();

            // トークン列が空の場合は次の行へ
            if tokens.len() == 0 {
                continue;
            }

            self.run_line(&mut tokens);
        }
    }

    fn parse_line(&mut self, line: &str) -> Vec<String>
    {
        let mut result = Vec::new();
        let len = line.len();

        let mut token = String::new();

        // ラインが空の場合は空のリストを返す
        if len == 0 {
            return result;
        }

        for i in 0..len {
            let c = line.chars().nth(i).unwrap();
            match c {
                '+' | '-' | '*' | '/' | '%' | '=' | '(' | ')' => {

                    // ためていたトークンを追加
                    if token.len() > 0 {
                        result.push(token.clone());
                        token.clear();
                    }

                    // 演算子を追加
                    result.push(c.to_string());
                }
                ' ' => {
                    // ためていたトークンを追加
                    if token.len() > 0 {
                        result.push(token.clone());
                        token.clear();
                    }
                }
                _ => {
                    token.push(c);
                }
            }
        }

        // ためていたトークンを追加
        if token.len() > 0 {
            result.push(token.clone());
        }
        result
    }

    fn factor(&mut self, tokens: &mut Vec<Token>) -> i32
    {
        let token = tokens.pop().unwrap();
        match token
        {
            Token::Value(val) => val,
            Token::Variable(var) =>
                {
                    if let Some(val) = self.variables.get(var.as_str())
                    {
                        *val
                    } else {
                        panic!("変数がありません : {}", var);
                    }
                }
            Token::OperatorParen(ArithmeticOperandParen::Left) =>
                {
                    let result = self.arithmetic_equation(tokens);

                    let next = tokens.pop().unwrap();
                    match next
                    {
                        Token::OperatorParen(ArithmeticOperandParen::Right) => {
                            result
                        }
                        _ =>
                            {
                                panic!("括弧が閉じられていません : {}", next);
                            }
                    }
                }
            _ =>
                {
                    panic!("数値でも変数でもありません : {}", token);
                }
        }
    }

    fn term(&mut self, tokens: &mut Vec<Token>) -> i32
    {
        if tokens.len() == 0
        {
            panic!("トークンがありません");
        }

        let mut result = self.factor(tokens);
        while tokens.len() > 0
        {
            let op = tokens.pop().unwrap();
            match op
            {
                Token::OperatorTail(ArithmeticOperandTail::Multiply) =>
                    {
                        let s = self.factor(tokens);
                        result *= s;
                    }
                Token::OperatorTail(ArithmeticOperandTail::Divide) =>
                    {
                        let s = self.factor(tokens);

                        if s == 0
                        {
                            panic!("0で割ることはできません");
                        }

                        result /= s;
                    }
                Token::OperatorTail(ArithmeticOperandTail::Mod) =>
                    {
                        let s = self.factor(tokens);
                        result %= s;
                    }
                // ( の場合は再帰的に計算
                Token::OperatorParen(ArithmeticOperandParen::Left) =>
                    {
                        tokens.push(op);
                        result *= self.factor(tokens);
                    }
                // ) の場合は終了
                Token::OperatorParen(ArithmeticOperandParen::Right) =>
                    {
                        tokens.push(op);
                        return result;
                    }
                // +, - の場合は演算子を戻し、終了
                Token::OperatorHead(_) =>
                    {
                        tokens.push(op);
                        break;
                    }
                _ =>
                    {
                        panic!("演算子がありません : {}", op);
                    }
            }
        }

        result
    }
    fn run_line(&mut self, mut tokens: &mut Vec<Token>)
    {
        // 変数一つだけの場合はそのまま表示
        if tokens.len() == 1 {
            match tokens[0] {
                Token::Variable(ref var) => {
                    if let Some(val) = self.variables.get(var) {
                        println!("{}", val);
                    } else {
                        panic!("変数がありません : {}", var);
                    }
                }
                _ => {}
            }
            return;
        }

        tokens.reverse();

        self.equation(tokens);
    }

    fn equation(&mut self, tokens: &mut Vec<Token>) -> i32
    {
        let first = tokens.pop().unwrap();
        let second = tokens.pop().unwrap();

        // 二番目に取り出したトークンが代入演算子であることを確認する
        match second
        {
            Token::Assign => {}
            _ =>
                {
                    panic!("代入演算子がありません : {}", second);
                }
        }

        let result = self.arithmetic_equation(tokens);

        if let Token::Variable(var) = first
        {
            self.variables.insert(var, result);
        } else {
            panic!("変数がありません : {}", first);
        }

        result
    }

    fn arithmetic_equation(&mut self, mut tokens: &mut Vec<Token>) -> i32
    {
        let mut result = self.term(&mut tokens);
        while tokens.len() > 0
        {
            let op = tokens.pop().unwrap();
            match op
            {
                Token::OperatorHead(ArithmeticOperandHead::Plus) =>
                    {
                        result += self.term(&mut tokens);
                    }
                Token::OperatorHead(ArithmeticOperandHead::Minus) =>
                    {
                        result -= self.term(&mut tokens);
                    }
                Token::OperatorParen(ArithmeticOperandParen::Left) =>
                    {
                        tokens.push(op);
                        result += self.term(&mut tokens);
                    }
                Token::OperatorParen(ArithmeticOperandParen::Right) =>
                    {
                        tokens.push(op);
                        return result;
                    }
                _ =>
                    {
                        panic!("演算子がありません : {}", op);
                    }
            }
        }

        result
    }

    fn convert_token(&mut self, token: &str) -> Token
    {
        match token {
            PLUS => Token::OperatorHead(ArithmeticOperandHead::Plus),
            MINUS => Token::OperatorHead(ArithmeticOperandHead::Minus),
            MULTIPLY => Token::OperatorTail(ArithmeticOperandTail::Multiply),
            DIVIDE => Token::OperatorTail(ArithmeticOperandTail::Divide),
            MOD => Token::OperatorTail(ArithmeticOperandTail::Mod),
            LEFT_PAREN => Token::OperatorParen(ArithmeticOperandParen::Left),
            RIGHT_PAREN => Token::OperatorParen(ArithmeticOperandParen::Right),
            EQUAL => Token::Assign,
            _ => {
                if let Ok(num) = token.parse::<i32>() {
                    Token::Value(num)
                } else {
                    Token::Variable(token.to_string())
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn run_program(program: &str) -> Interpreter {
        let mut interpreter = Interpreter::new();
        interpreter.contents = program.to_string();
        interpreter.run_interpreter();
        interpreter
    }

    #[test]
    fn test_arithmetic_addition() {
        let interpreter = run_program("
            a = 5
            b = 10
            c = a + b
        ");
        assert_eq!(interpreter.variables.get("c"), Some(&15));
    }

    #[test]
    fn test_arithmetic_subtraction() {
        let interpreter = run_program("
            x = 20
            y = 8
            z = x - y
        ");
        assert_eq!(interpreter.variables.get("z"), Some(&12));
    }

    #[test]
    fn test_arithmetic_multiplication() {
        let interpreter = run_program("
            m = 3
            n = 7
            p = m * n
        ");
        assert_eq!(interpreter.variables.get("p"), Some(&21));
    }

    #[test]
    fn test_arithmetic_division() {
        let interpreter = run_program("
            a = 20
            b = 4
            c = a / b
        ");
        assert_eq!(interpreter.variables.get("c"), Some(&5));
    }

    #[test]
    #[should_panic(expected = "変数がありません")]
    fn test_variable_not_found() {
        let _interpreter = run_program("undefined");
    }

    #[test]
    #[should_panic(expected = "代入演算子がありません")]
    fn test_invalid_operator() {
        let _interpreter = run_program("
            a = 5
            a +
        ");
    }

    #[test]
    #[should_panic(expected = "0で割ることはできません")]
    fn test_division_by_zero() {
        let _interpreter = run_program("
            a = 10
            b = 0
            c = a / b
        ");
    }

    #[test]
    fn test_parentheses_operation() {
        let interpreter = run_program("
        a = (1 + 2) * 3
    ");
        assert_eq!(interpreter.variables.get("a"), Some(&9));
    }

    #[test]
    fn test_nested_parentheses_operation() {
        let interpreter = run_program("
        a = ((2 + 3) * (4 - 1)) / 5
    ");
        assert_eq!(interpreter.variables.get("a"), Some(&3));
    }

    #[test]
    #[should_panic(expected = "括弧が閉じられていません")]
    fn test_unmatched_left_parenthesis() {
        let _interpreter = run_program("
        a = (1 + 2 * 3
    ");
    }

    #[test]
    #[should_panic(expected = "括弧が閉じられていません")]
    fn test_unmatched_right_parenthesis() {
        let _interpreter = run_program("
        a = 1 + 2) * 3
    ");
    }
}
