use std::fmt;

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


pub fn parse_line(line: &str) -> Vec<String>
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

pub fn convert_token(token: &str) -> Token
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


pub fn run_line(mut tokens: Vec<Token>)
{
    // 変数一つだけの場合はそのまま表示
    if tokens.len() == 1 {
        match tokens[0] {
            Token::Variable(ref var) => {
                println!("変数の値を表示, 未実装");
            }
            _ => {}
        }
        return;
    }

    tokens.reverse();

    equation(tokens);
}

pub fn equation(mut tokens: Vec<Token>) -> i32
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

    let result = arithmetic_equation(tokens);

    result
}

pub fn arithmetic_equation(mut tokens: Vec<Token>) -> i32
{
    let mut result = term(&mut tokens);
    while tokens.len() > 0
    {
        let op = tokens.pop().unwrap();
        match op
        {
            Token::OperatorHead(ArithmeticOperandHead::Plus) =>
                {
                    result += term(&mut tokens);
                }
            Token::OperatorHead(ArithmeticOperandHead::Minus) =>
                {
                    result -= term(&mut tokens);
                }
            _ =>
                {
                    panic!("演算子がありません : {}", op);
                }
        }
    }

    result
}

pub fn term(mut tokens: &mut Vec<Token>) -> i32
{
    let first = tokens.pop().unwrap();

    let mut result = factor(&first);
    while tokens.len() > 0
    {
        let op = tokens.pop().unwrap();
        match op
        {
            Token::OperatorTail(ArithmeticOperandTail::Multiply) =>
                {
                    let second = tokens.pop().unwrap();
                    let s = factor(&second);
                    result *= s;
                }
            Token::OperatorTail(ArithmeticOperandTail::Divide) =>
                {
                    let second = tokens.pop().unwrap();
                    let s = factor(&second);
                    result /= s;
                }
            Token::OperatorTail(ArithmeticOperandTail::Mod) =>
                {
                    let second = tokens.pop().unwrap();
                    let s = factor(&second);
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

pub fn factor(mut token: &Token) -> i32
{
    match token
    {
        Token::Value(val) => *val,
        Token::Variable(var) => 0,
        _ =>
            {
                panic!("数値でも変数でもありません : {}", token);
            }
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parse_line()
    {
        let line = "1 + 2";
        let tokens = parse_line(line);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], "1");
        assert_eq!(tokens[1], "+");
        assert_eq!(tokens[2], "2");

        let line = "a=1+2*3";
    }

    #[test]
    fn test_equation()
    {
        let mut tokens = vec![
            Token::Value(1),
            Token::Assign,
            Token::Value(2),
            Token::OperatorHead(ArithmeticOperandHead::Plus),
            Token::Value(3),
            Token::OperatorTail(ArithmeticOperandTail::Multiply),
            Token::Value(4),
        ];

        let result = equation(tokens);
        assert_eq!(result, 15);
    }
}