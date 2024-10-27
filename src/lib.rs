use std::collections::HashMap;
use std::fmt;
use std::env;
use std::fmt::Display;
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
const COMPARISON: &str = "==";
const NOT_EQUAL: &str = "!=";
const GREATER_THAN: &str = ">";
const LESS_THAN: &str = "<";
const GREATER_THAN_OR_EQUAL: &str = ">=";
const LESS_THAN_OR_EQUAL: &str = "<=";

#[derive(Debug, Clone)]
pub(crate) enum ComparisonOperand
{
    Equal,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqual,
    None,
}

impl Display for ComparisonOperand
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            ComparisonOperand::Equal => write!(f, "{}", COMPARISON),
            ComparisonOperand::GreaterThan => write!(f, "{}", GREATER_THAN),
            ComparisonOperand::LessThan => write!(f, "{}", LESS_THAN),
            ComparisonOperand::GreaterThanOrEqual => write!(f, "{}", GREATER_THAN_OR_EQUAL),
            ComparisonOperand::LessThanOrEqual => write!(f, "{}", LESS_THAN_OR_EQUAL),
            ComparisonOperand::NotEqual => write!(f, "{}", NOT_EQUAL),
            _ => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ArithmeticOperandTail
{
    Multiply,
    Divide,
    Mod,
    None,
}

#[derive(Debug, Clone)]
pub(crate) enum ArithmeticOperandHead
{
    Plus,
    Minus,
    None,
}

#[derive(Debug, Clone)]
pub(crate) enum ArithmeticOperandParen
{
    Left,
    Right,
    None,
}

#[derive(Debug, Clone)]
pub(crate) enum Token
{
    Value(Value),
    OperatorHead(ArithmeticOperandHead),
    OperatorTail(ArithmeticOperandTail),
    Variable(String),
    OperatorParen(ArithmeticOperandParen),
    OperatorComparison(ComparisonOperand),
    Assign,
    None,
}

impl Display for Token
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
            Token::OperatorComparison(op) => match op
            {
                ComparisonOperand::Equal => write!(f, "{}", COMPARISON),
                ComparisonOperand::GreaterThan => write!(f, "{}", GREATER_THAN),
                ComparisonOperand::LessThan => write!(f, "{}", LESS_THAN),
                ComparisonOperand::GreaterThanOrEqual => write!(f, "{}", GREATER_THAN_OR_EQUAL),
                ComparisonOperand::LessThanOrEqual => write!(f, "{}", LESS_THAN_OR_EQUAL),
                _ => write!(f, "None"),
            },
            Token::Variable(var) => write!(f, "{}", var),
            Token::Assign => write!(f, "{}", EQUAL),
            Token::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum Value
{
    Int(i32),
    Float(f32),
    Bool(bool),
}

impl Display for Value
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Value::Int(val) => write!(f, "{}", val),
            Value::Float(val) => write!(f, "{}", val),
            Value::Bool(val) => write!(f, "{}", val),
        }
    }
}

pub struct Interpreter
{
    variables: HashMap<String, Value>,
    contents: String,
}


impl Interpreter
{
    pub fn new(contents: &str) -> Self
    {
        Interpreter
        {
            variables: HashMap::new(),
            contents: contents.to_string(),
        }
    }

    pub fn get_variable(&self, var: &str) -> Value
    {
        if let Some(val) = self.variables.get(var)
        {
            val.clone()
        } else {
            panic!("変数がありません : {}", var);
        }
    }

    pub fn run(&mut self)
    {
        self.run_interpreter();
    }

    pub(crate) fn run_interpreter(&mut self)
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

            // 括弧の数が正しいか確認
            if !self.check_parentheses(line.as_str()) {
                panic!("括弧の数が正しくありません : {}", line);
            }

            self.run_line(&mut tokens);
        }
    }

    pub(crate) fn check_parentheses(&self, line: &str) -> bool
    {
        let re = Regex::new(r"\(|\)").unwrap();
        let mut count = 0;
        for c in re.captures_iter(line) {
            match c.get(0).unwrap().as_str() {
                "(" => count += 1,
                ")" => count -= 1,
                _ => {}
            }
            if count < 0 {
                return false;
            }
        }
        count == 0
    }

    pub(crate) fn convert_token(&mut self, token: &str) -> Token
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
            COMPARISON => Token::OperatorComparison(ComparisonOperand::Equal),
            GREATER_THAN => Token::OperatorComparison(ComparisonOperand::GreaterThan),
            LESS_THAN => Token::OperatorComparison(ComparisonOperand::LessThan),
            GREATER_THAN_OR_EQUAL => Token::OperatorComparison(ComparisonOperand::GreaterThanOrEqual),
            LESS_THAN_OR_EQUAL => Token::OperatorComparison(ComparisonOperand::LessThanOrEqual),
            NOT_EQUAL => Token::OperatorComparison(ComparisonOperand::NotEqual),
            _ => {
                // 整数か浮動小数点数か判定
                if let Ok(num) = token.parse::<i32>() {
                    Token::Value(Value::Int(num))
                } else if let Ok(num) = token.parse::<f32>() {
                    Token::Value(Value::Float(num))
                } else {
                    Token::Variable(token.to_string())
                }
            }
        }
    }

    pub(crate) fn parse_line(&mut self, line: &str) -> Vec<String>
    {
        let mut result = Vec::new();
        let len = line.len();

        let mut token = String::new();

        // ラインが空の場合は空のリストを返す
        if len == 0 {
            return result;
        }

        let mut i = 0;
        while i < len {
            let c = line.chars().nth(i).unwrap();
            match c {
                '+' | '-' | '*' | '/' | '%' | '(' | ')' => {

                    // ためていたトークンを追加
                    if token.len() > 0 {
                        result.push(token.clone());
                        token.clear();
                    }

                    // 演算子を追加
                    result.push(c.to_string());
                }
                '>' | '<' | '=' | '!' =>
                    {
                        // もう一つ次の文字を取得
                        let next = line.chars().nth(i + 1).unwrap();

                        // ためていたトークンを追加
                        if token.len() > 0 {
                            result.push(token.clone());
                            token.clear();
                        }

                        // 比較演算子を追加
                        match next {
                            '=' => {
                                result.push(format!("{}{}", c, next));
                            }
                            _ => {
                                result.push(c.to_string());
                            }
                        }

                        // 次の文字をスキップ
                        i += 1;
                    }
                '#' => {
                    // コメントの場合はそれ以降の文字を無視
                    break;
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
            i += 1;
        }

        // ためていたトークンを追加
        if token.len() > 0 {
            result.push(token.clone());
        }
        result
    }

    pub(crate) fn run_line(&mut self, mut tokens: &mut Vec<Token>)
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

    pub(crate) fn equation(&mut self, tokens: &mut Vec<Token>)
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

        let mut result = self.arithmetic_equation(tokens);

        // 比較演算子が残っている場合
        if let Some(Token::OperatorComparison(op)) = tokens.pop()
        {
            let mut comparison = false;
            let left = result.clone();
            let right = self.arithmetic_equation(tokens);

            match op
            {
                ComparisonOperand::Equal =>
                    {
                        match (left, right)
                        {
                            (Value::Int(a), Value::Int(b)) => comparison = a == b,
                            (Value::Int(a), Value::Float(b)) => comparison = a as f32 == b,
                            (Value::Float(a), Value::Int(b)) => comparison = a == b as f32,
                            (Value::Float(a), Value::Float(b)) => comparison = a == b,
                            _ => panic!("数値型または浮動小数点以外の比較はできません"),
                        }
                    }
                ComparisonOperand::GreaterThan =>
                    {
                        match (left, right)
                        {
                            (Value::Int(a), Value::Int(b)) => comparison = a > b,
                            (Value::Int(a), Value::Float(b)) => comparison = a as f32 > b,
                            (Value::Float(a), Value::Int(b)) => comparison = a > b as f32,
                            (Value::Float(a), Value::Float(b)) => comparison = a > b,
                            _ => panic!("数値型または浮動小数点以外の比較はできません"),
                        }
                    }
                ComparisonOperand::LessThan =>
                    {
                        match (left, right)
                        {
                            (Value::Int(a), Value::Int(b)) => comparison = a < b,
                            (Value::Int(a), Value::Float(b)) => comparison = (a as f32) < b,
                            (Value::Float(a), Value::Int(b)) => comparison = a < (b as f32),
                            (Value::Float(a), Value::Float(b)) => comparison = a < b,
                            _ => panic!("数値型または浮動小数点以外の比較はできません"),
                        }
                    }
                ComparisonOperand::GreaterThanOrEqual =>
                    {
                        match (left, right)
                        {
                            (Value::Int(a), Value::Int(b)) => comparison = a >= b,
                            (Value::Int(a), Value::Float(b)) => comparison = a as f32 >= b,
                            (Value::Float(a), Value::Int(b)) => comparison = a >= b as f32,
                            (Value::Float(a), Value::Float(b)) => comparison = a >= b,
                            _ => panic!("数値型または浮動小数点以外の比較はできません"),
                        }
                    }
                ComparisonOperand::LessThanOrEqual =>
                    {
                        match (left, right)
                        {
                            (Value::Int(a), Value::Int(b)) => comparison = a <= b,
                            (Value::Int(a), Value::Float(b)) => comparison = a as f32 <= b,
                            (Value::Float(a), Value::Int(b)) => comparison = a <= b as f32,
                            (Value::Float(a), Value::Float(b)) => comparison = a <= b,
                            _ => panic!("数値型または浮動小数点以外の比較はできません"),
                        }
                    }
                ComparisonOperand::NotEqual =>
                    {
                        match (left, right)
                        {
                            (Value::Int(a), Value::Int(b)) => comparison = a != b,
                            (Value::Int(a), Value::Float(b)) => comparison = a as f32 != b,
                            (Value::Float(a), Value::Int(b)) => comparison = a != b as f32,
                            (Value::Float(a), Value::Float(b)) => comparison = a != b,
                            _ => panic!("数値型または浮動小数点以外の比較はできません"),
                        }
                    }
                _ =>
                    {
                        panic!("比較演算子がありません : {}", op);
                    }
            }

            if let Token::Variable(var) = first.clone()
            {
                result = Value::Int(comparison as i32);
            } else {
                panic!("変数がありません : {}", first);
            }
        }

        if let Token::Variable(var) = first
        {
            self.variables.insert(var, result);
        } else {
            panic!("変数がありません : {}", first);
        }
    }

    pub(crate) fn arithmetic_equation(&mut self, mut tokens: &mut Vec<Token>) -> Value
    {
        let mut result = self.term(&mut tokens);
        while tokens.len() > 0
        {
            let op = tokens.pop().unwrap();
            match op
            {
                Token::OperatorHead(ArithmeticOperandHead::Plus) =>
                    {
                        let term_result = self.term(&mut tokens);
                        match (result, term_result)
                        {
                            (Value::Int(a), Value::Int(b)) => result = Value::Int(a + b),
                            (Value::Int(a), Value::Float(b)) => result = Value::Float(a as f32 + b),
                            (Value::Float(a), Value::Int(b)) => result = Value::Float(a + b as f32),
                            (Value::Float(a), Value::Float(b)) => result = Value::Float(a + b),
                            _ => panic!("数値型または浮動小数点以外の演算はできません"),
                        }
                    }
                Token::OperatorHead(ArithmeticOperandHead::Minus) =>
                    {
                        let term_result = self.term(&mut tokens);
                        match (result, term_result)
                        {
                            (Value::Int(a), Value::Int(b)) => result = Value::Int(a - b),
                            (Value::Int(a), Value::Float(b)) => result = Value::Float(a as f32 - b),
                            (Value::Float(a), Value::Int(b)) => result = Value::Float(a - b as f32),
                            (Value::Float(a), Value::Float(b)) => result = Value::Float(a - b),
                            _ => panic!("数値型または浮動小数点以外の演算はできません"),
                        }
                    }
                Token::OperatorParen(ArithmeticOperandParen::Left) =>
                    {
                        tokens.push(op);
                        let term_result = self.term(&mut tokens);
                        match term_result
                        {
                            Value::Int(a) => result = Value::Int(a),
                            Value::Float(a) => result = Value::Float(a),
                            _ => panic!("数値型または浮動小数点以外の演算はできません"),
                        }
                    }
                Token::OperatorParen(ArithmeticOperandParen::Right) =>
                    {
                        tokens.push(op);
                        break;
                    }
                // 比較演算子の場合は演算子を戻し、終了
                Token::OperatorComparison(_) =>
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

    pub(crate) fn term(&mut self, tokens: &mut Vec<Token>) -> Value
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
                        match (result, s)
                        {
                            (Value::Int(a), Value::Int(b)) => result = Value::Int(a * b),
                            (Value::Int(a), Value::Float(b)) => result = Value::Float(a as f32 * b),
                            (Value::Float(a), Value::Int(b)) => result = Value::Float(a * b as f32),
                            (Value::Float(a), Value::Float(b)) => result = Value::Float(a * b),
                            _ => panic!("数値型または浮動小数点以外の演算はできません"),
                        }
                    }
                Token::OperatorTail(ArithmeticOperandTail::Divide) =>
                    {
                        let s = self.factor(tokens);

                        match (result, s)
                        {
                            (Value::Int(a), Value::Int(b)) => {
                                if b == 0 {
                                    panic!("0で割ることはできません");
                                }
                                result = Value::Int(a / b);
                            }
                            (Value::Int(a), Value::Float(b)) => {
                                if b == 0.0 {
                                    panic!("0で割ることはできません");
                                }
                                result = Value::Float(a as f32 / b);
                            }
                            (Value::Float(a), Value::Int(b)) => {
                                if b == 0 {
                                    panic!("0で割ることはできません");
                                }
                                result = Value::Float(a / b as f32);
                            }
                            (Value::Float(a), Value::Float(b)) => {
                                if b == 0.0 {
                                    panic!("0で割ることはできません");
                                }
                                result = Value::Float(a / b);
                            }
                            _ => panic!("数値型または浮動小数点以外の演算はできません"),
                        }
                    }
                Token::OperatorTail(ArithmeticOperandTail::Mod) =>
                    {
                        let s = self.factor(tokens);

                        match (result, s)
                        {
                            (Value::Int(a), Value::Int(b)) => {
                                if b == 0 {
                                    panic!("0で割ることはできません");
                                }
                                result = Value::Int(a % b);
                            }
                            _ => panic!("整数型以外の演算はできません"),
                        }
                    }
                // ( の場合は再帰的に計算
                Token::OperatorParen(ArithmeticOperandParen::Left) =>
                    {
                        tokens.push(op);
                        let factor_result = self.factor(tokens);

                        match factor_result
                        {
                            Value::Int(a) => result = Value::Int(a),
                            Value::Float(a) => result = Value::Float(a),
                            _ => panic!("数値型または浮動小数点以外の演算はできません"),
                        }
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
                // 比較演算子
                Token::OperatorComparison(_) =>
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
    pub(crate) fn factor(&mut self, tokens: &mut Vec<Token>) -> Value
    {
        let token = tokens.pop().unwrap();
        match token
        {
            Token::Value(val) => val,
            Token::Variable(var) =>
                {
                    if let Some(val) = self.variables.get(var.as_str())
                    {
                        val.clone()
                    } else {
                        panic!("変数がありません : {}", var);
                    }
                }
            Token::OperatorParen(ArithmeticOperandParen::Left) =>
                {
                    let result = self.arithmetic_equation(tokens);
                    if let Some(T) = tokens.pop() // get next token
                    {
                        match T
                        {
                            Token::OperatorParen(ArithmeticOperandParen::Right) => {
                                result
                            }
                            _ =>
                                {
                                    panic!("括弧が閉じられていません : {}", T);
                                }
                        }
                    } else {
                        panic!("括弧が閉じられていません : {}", token);
                    }
                }
            Token::OperatorParen(ArithmeticOperandParen::Right) =>
                {
                    panic!("括弧が閉じられていません : {}", token);
                }
            _ =>
                {
                    panic!("数値でも変数でもありません : {}", token);
                }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn parse_line(program: &str) -> Vec<String> {
        let mut interpreter = Interpreter::new(program);
        let line = program.lines().next().unwrap();
        interpreter.parse_line(line)
    }

    fn run_program(program: &str) -> Interpreter {
        let mut interpreter = Interpreter::new(program);
        interpreter.contents = program.to_string();
        interpreter.run_interpreter();
        interpreter
    }

    #[test]
    fn test_parse_line_with_comparison() {
        let tokens = parse_line("a = 4 >= 5");
        assert_eq!(tokens, vec!["a", "=", "4", ">=", "5"]);

        let tokens = parse_line("b = 3 < 2");
        assert_eq!(tokens, vec!["b", "=", "3", "<", "2"]);

        let tokens = parse_line("c = 1 == 1");
        assert_eq!(tokens, vec!["c", "=", "1", "==", "1"]);

        let tokens = parse_line("d = 2 <= 3");
        assert_eq!(tokens, vec!["d", "=", "2", "<=", "3"]);

        let tokens = parse_line("e = 4 > 5");
        assert_eq!(tokens, vec!["e", "=", "4", ">", "5"]);

        let tokens = parse_line("f = 5 != 6");
        assert_eq!(tokens, vec!["f", "=", "5", "!=", "6"]);

        let tokens = parse_line("g = 5.5 == 5.5");
        assert_eq!(tokens, vec!["g", "=", "5.5", "==", "5.5"]);
    }

    #[test]
    fn test_comparison_equal_true() {
        let interpreter = run_program("
            a = 5
            b = 5
            c = a == b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(1));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 5.5
            b = 5.5
            c = a == b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(1));
    }

    #[test]
    fn test_comparison_equal_false() {
        let interpreter = run_program("
            a = 5
            b = 10
            c = a == b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 5.5
            b = 5.6
            c = a == b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));
    }

    #[test]
    fn test_comparison_not_equal_true() {
        let interpreter = run_program("
            a = 5
            b = 10
            c = a != b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(1));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 5.5
            b = 5.6
            c = a != b
        ");
    }

    #[test]
    fn test_comparison_not_equal_false() {
        let interpreter = run_program("
            a = 5
            b = 5
            c = a != b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 5.5
            b = 5.5
            c = a != b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));
    }

    #[test]
    fn test_comparison_greater_than_true() {
        let interpreter = run_program("
            a = 10
            b = 5
            c = a > b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(1));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 10.5
            b = 5.5
            c = a > b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(1));
    }

    #[test]
    fn test_comparison_greater_than_false() {
        let interpreter = run_program("
            a = 5
            b = 10
            c = a > b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 5.5
            b = 10.5
            c = a > b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));
    }

    #[test]
    fn test_comparison_less_than_true() {
        let interpreter = run_program("
            a = 5
            b = 10
            c = a < b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(1));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 5.5
            b = 10.5
            c = a < b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(1));
    }

    #[test]
    fn test_comparison_less_than_false() {
        let interpreter = run_program("
            a = 10
            b = 5
            c = a < b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 10.5
            b = 5.5
            c = a < b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));
    }

    #[test]
    fn test_comparison_greater_than_or_equal_true() {
        let interpreter = run_program("
            a = 10
            b = 5
            c = a >= b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(1));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 10.5
            b = 5.5
            c = a >= b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(1));
    }

    #[test]
    fn test_comparison_greater_than_or_equal_false() {
        let interpreter = run_program("
            a = 5
            b = 10
            c = a >= b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 5.5
            b = 10.5
            c = a >= b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));
    }

    #[test]
    fn test_comparison_less_than_or_equal_true() {
        let interpreter = run_program("
            a = 5
            b = 10
            c = a <= b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(1));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 5.5
            b = 10.5
            c = a <= b
        ");
    }

    #[test]
    fn test_comparison_less_than_or_equal_false() {
        let interpreter = run_program("
            a = 10
            b = 5
            c = a <= b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));

        // 浮動小数点数の比較
        let interpreter = run_program("
            a = 10.5
            b = 5.5
            c = a <= b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(0));
    }

    #[test]
    fn test_arithmetic_addition() {
        let interpreter = run_program("
            a = 5
            b = 10
            c = a + b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(15));

        // 浮動小数点数の加算
        let interpreter = run_program("
            x = 5.5
            y = 10.5
            z = x + y
        ");
        assert_eq!(interpreter.variables.get("z").unwrap(), &Value::Float(16.0));
    }

    #[test]
    fn test_arithmetic_subtraction() {
        let interpreter = run_program("
            x = 20
            y = 8
            z = x - y
        ");
        assert_eq!(interpreter.variables.get("z").unwrap(), &Value::Int(12));

        // 浮動小数点数の減算
        let interpreter = run_program("
            x = 20.5
            y = 8.5
            z = x - y
        ");
        assert_eq!(interpreter.variables.get("z").unwrap(), &Value::Float(12.0));
    }

    #[test]
    fn test_arithmetic_multiplication() {
        let interpreter = run_program("
            m = 3
            n = 7
            p = m * n
        ");
        assert_eq!(interpreter.variables.get("p").unwrap(), &Value::Int(21));

        // 浮動小数点数の乗算
        let interpreter = run_program("
            m = 3.5
            n = 7.5
            p = m * n
        ");
        assert_eq!(interpreter.variables.get("p").unwrap(), &Value::Float(26.25));
    }

    #[test]
    fn test_arithmetic_division() {
        let interpreter = run_program("
            a = 20
            b = 4
            c = a / b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(5));

        // 浮動小数点数の除算
        let interpreter = run_program("
            a = 20.5
            b = 4.5
            c = a / b
        ");
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Float(4.5555553));
    }

    #[test]
    #[should_panic(expected = "変数がありません : undefined")]
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
        assert_eq!(interpreter.variables.get("a").unwrap(), &Value::Int(9));
    }

    #[test]
    fn test_nested_parentheses_operation() {
        let interpreter = run_program("
        a = ((2 + 3) * (4 - 1)) / 5
    ");
        assert_eq!(interpreter.variables.get("a").unwrap(), &Value::Int(3));
    }

    #[test]
    #[should_panic(expected = "括弧の数が正しくありません : a = (1 + 2 * 3")]
    fn test_unmatched_left_parenthesis() {
        let _interpreter = run_program("a = (1 + 2 * 3");
    }

    #[test]
    #[should_panic(expected = "括弧の数が正しくありません : a = 1 + 2) * 3")]
    fn test_unmatched_right_parenthesis() {
        let _interpreter = run_program("a = 1 + 2) * 3");
    }

    #[test]
    fn test_comments() {
        let interpreter = run_program("
        a = 5 # this is a comment
        b = 10
        # c = a + b
        d = a * b # this is another comment
    ");
        assert_eq!(interpreter.variables.get("a").unwrap(), &Value::Int(5));
        assert_eq!(interpreter.variables.get("b").unwrap(), &Value::Int(10));
        assert_eq!(interpreter.variables.get("d").unwrap(), &Value::Int(50));
    }

    #[test]
    fn test_float_and_int()
    {
        let interpreter = run_program("
            a = 5
            b = 5.5
            c = a + b
            d = a * b
            e = a / b
            f = a - b
        ");

        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Float(10.5));
        assert_eq!(interpreter.variables.get("d").unwrap(), &Value::Float(27.5));
        assert_eq!(interpreter.variables.get("e").unwrap(), &Value::Float(0.90909094));
        assert_eq!(interpreter.variables.get("f").unwrap(), &Value::Float(-0.5));
    }

    #[test]
    #[should_panic(expected = "整数型以外の演算はできません")]
    fn test_invalid_operation() {
        let _interpreter = run_program("
            a = 5
            b = 5.5
            c = a % b
        ");
    }

    #[test]
    fn test_int_operations() {
        let interpreter = run_program("
            a = 5
            b = 10
            c = (a + b) * 2
            d = c / 5
            e = d - 3
            f = e % 2
            g = f == 0
            h = (a * b) > (c / 2)
            i = (a + b) <= (c - d)
            j = (((1+1)))
        ");

        assert_eq!(interpreter.variables.get("a").unwrap(), &Value::Int(5));
        assert_eq!(interpreter.variables.get("b").unwrap(), &Value::Int(10));
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Int(30));
        assert_eq!(interpreter.variables.get("d").unwrap(), &Value::Int(6));
        assert_eq!(interpreter.variables.get("e").unwrap(), &Value::Int(3));
        assert_eq!(interpreter.variables.get("f").unwrap(), &Value::Int(1));
        assert_eq!(interpreter.variables.get("g").unwrap(), &Value::Int(0));
        assert_eq!(interpreter.variables.get("h").unwrap(), &Value::Int(1));
        assert_eq!(interpreter.variables.get("i").unwrap(), &Value::Int(1));
        assert_eq!(interpreter.variables.get("j").unwrap(), &Value::Int(2));
    }

    #[test]
    fn test_float_operations() {
        let interpreter = run_program("
        a = 5.5
        b = 10.2
        c = (a + b) * 2.0
        d = c / 5.1
        e = d - 3.3
        f = e / 2.2
        g = f == 0.0
        h = (a * b) > (c / 2.0)
        i = (a + b) <= (c - d)
        j = (((1.1 + 1.1)))
    ");

        assert_eq!(interpreter.variables.get("a").unwrap(), &Value::Float(5.5));
        assert_eq!(interpreter.variables.get("b").unwrap(), &Value::Float(10.2));
        assert_eq!(interpreter.variables.get("c").unwrap(), &Value::Float(31.4));
        assert_eq!(interpreter.variables.get("d").unwrap(), &Value::Float(6.156862745098039));
        assert_eq!(interpreter.variables.get("e").unwrap(), &Value::Float(2.856862745098039));
        assert_eq!(interpreter.variables.get("f").unwrap(), &Value::Float(1.2985739704991095));
        assert_eq!(interpreter.variables.get("g").unwrap(), &Value::Int(0));
        assert_eq!(interpreter.variables.get("h").unwrap(), &Value::Int(1));
        assert_eq!(interpreter.variables.get("i").unwrap(), &Value::Int(1));
        assert_eq!(interpreter.variables.get("j").unwrap(), &Value::Float(2.2));
    }
}
