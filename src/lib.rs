use std::collections::HashMap;
use std::fmt;
use std::env;
use std::fs::File;
use std::io::prelude::*;
// 定数
const PLUS: &str = "+";
const MINUS: &str = "-";
const MULTIPLY: &str = "*";
const DIVIDE: &str = "/";
const MOD: &str = "%";
const EQUAL: &str = "=";

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
pub enum Token
{
    Value(i32),
    OperatorHead(ArithmeticOperandHead),
    OperatorTail(ArithmeticOperandTail),
    Variable(String),
    Assign,
    None,
}

#[derive(Debug, Clone)]
enum Term
{
    Factor(i32),
    Operator(ArithmeticOperandTail),
    None,
}


enum ArithmeticEquation
{
    Term(i32),
    Operator(ArithmeticOperandHead),
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
            Token::Variable(var) => write!(f, "{}", var),
            Token::Assign => write!(f, "{}", EQUAL),
            Token::None => write!(f, "None"),
        }
    }
}


pub struct Interpreter
{
    variables: HashMap<String, i32>,
}


impl Interpreter
{
    pub fn new() -> Self
    {
        Interpreter
        {
            variables: HashMap::new()
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

        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Failed to read file");


        for line in contents.lines() {
            // 1行をトークンに分割する
            let tokens = self.parse_line(line);

            // 文字列をtoken型の列に変換する
            let tokens: Vec<Token> = tokens.iter().map(|token| self.convert_token(token)).collect();

            // トークン列が空の場合は次の行へ
            if tokens.len() == 0 {
                continue;
            }

            self.run_line(tokens);
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
                '+' | '-' | '*' | '/' | '%' | '=' => {

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

    fn factor(&mut self, mut token: &Token) -> i32
    {
        match token
        {
            Token::Value(val) => *val,
            Token::Variable(var) =>
                {
                    if let Some(val) = self.variables.get(var)
                    {
                        *val
                    }
                    else
                    {
                        panic!("変数がありません : {}", var);
                    }
                }
            _ =>
                {
                    panic!("数値でも変数でもありません : {}", token);
                }
        }
    }

    pub fn term(&mut self, mut tokens: &mut Vec<Token>) -> i32
    {
        let first = tokens.pop().unwrap();

        let mut result = self.factor(&first);
        while tokens.len() > 0
        {
            let op = tokens.pop().unwrap();
            match op
            {
                Token::OperatorTail(ArithmeticOperandTail::Multiply) =>
                    {
                        let second = tokens.pop().unwrap();
                        let s = self.factor(&second);
                        result *= s;
                    }
                Token::OperatorTail(ArithmeticOperandTail::Divide) =>
                    {
                        let second = tokens.pop().unwrap();
                        let s = self.factor(&second);
                        result /= s;
                    }
                Token::OperatorTail(ArithmeticOperandTail::Mod) =>
                    {
                        let second = tokens.pop().unwrap();
                        let s = self.factor(&second);
                        result %= s;
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
    pub fn run_line(&mut self, mut tokens: Vec<Token>)
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

    pub fn equation(&mut self, mut tokens: Vec<Token>) -> i32
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
        }
        else
        {
            panic!("変数がありません : {}", first);
        }

        result
    }

    pub fn arithmetic_equation(&mut self, mut tokens: Vec<Token>) -> i32
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
                _ =>
                    {
                        panic!("演算子がありません : {}", op);
                    }
            }
        }

        result
    }

    pub fn convert_token(&mut self, token: &str) -> Token
    {
        match token {
            PLUS => Token::OperatorHead(ArithmeticOperandHead::Plus),
            MINUS => Token::OperatorHead(ArithmeticOperandHead::Minus),
            MULTIPLY => Token::OperatorTail(ArithmeticOperandTail::Multiply),
            DIVIDE => Token::OperatorTail(ArithmeticOperandTail::Divide),
            MOD => Token::OperatorTail(ArithmeticOperandTail::Mod),
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
mod tests
{
    use super::*;
}