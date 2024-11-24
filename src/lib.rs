use std::collections::HashMap;
use std::fmt;
use std::env;
use std::fmt::{Arguments, Display};
use std::fs::File;
use std::io::prelude::*;
use std::ptr::null;
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
const FUNCTION: &str = "func";
const RETURN: &str = "return";
const BLOCK_LEFT_PAREN: &str = "{";
const BLOCK_RIGHT_PAREN: &str = "}";
const COMMA: &str = ",";
const END_OF_EXPRESSION: &str = ";";
const IF: &str = "if";
const ELSE: &str = "else";

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ArithmeticOperandTail
{
    Multiply,
    Divide,
    Mod,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ArithmeticOperandHead
{
    Plus,
    Minus,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ArithmeticOperandParen
{
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, PartialEq)]
enum BlockParen
{
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, PartialEq)]
enum Identifier
{
    Variable(Value),
    Function(Function),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token
{
    Value(Value),
    OperatorHead(ArithmeticOperandHead),
    OperatorTail(ArithmeticOperandTail),
    Identifier(String),
    OperatorParen(ArithmeticOperandParen),
    OperatorComparison(ComparisonOperand),
    Assign,
    Function,
    Return,
    BlockParen(BlockParen),
    COMMA,
    EndOfExpression,
    If,
    Else,
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
            Token::Identifier(var) => write!(f, "{}", var),
            Token::Assign => write!(f, "{}", EQUAL),
            Token::None => write!(f, "None"),
            Token::Function => write!(f, "{}", FUNCTION),
            Token::Return => write!(f, "{}", RETURN),
            Token::BlockParen(p) =>
                {
                    match p
                    {
                        BlockParen::Left => write!(f, "{}", BLOCK_LEFT_PAREN),
                        BlockParen::Right => write!(f, "{}", BLOCK_RIGHT_PAREN),
                        BlockParen::None => write!(f, "None"),
                    }
                }
            Token::COMMA => write!(f, "{}", COMMA),
            Token::EndOfExpression => write!(f, "{}", END_OF_EXPRESSION),
            Token::If => write!(f, "{}", IF),
            Token::Else => write!(f, "{}", ELSE),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
struct Function
{
    name: String,
    args: Vec<String>,
    body: Vec<Token>,
}

impl Function
{
    pub fn new(name: &str, args: Vec<String>, body: Vec<Token>) -> Self
    {
        Function
        {
            name: name.to_string(),
            args,
            body,
        }
    }

    fn run(&mut self, arguments: Vec<Value>) -> Value
    {
        let mut tokens_list = Vec::new();
        // ; 区切りでトークン列を作成
        let mut tokens = Vec::new();
        for token in self.body.clone() {
            match token {
                Token::EndOfExpression => {
                    tokens.push(token);
                    tokens_list.push(tokens.clone());
                    tokens.clear();
                }
                _ => {
                    tokens.push(token);
                }
            }
        }

        let mut interpreter = Interpreter::new(tokens_list);
        // 仮引数を関数内の変数に代入
        for (i, arg) in arguments.iter().enumerate() {
            let arg = Identifier::Variable(arg.clone());
            interpreter.identifiers.insert(self.args[i].clone(), arg);
        }

        // 関数を実行
        let result = interpreter.run();

        result
    }
}
fn check_parentheses(tokens: &Vec<Token>) -> bool
{
    let mut count = 0;
    for token in tokens.clone() {
        match token {
            Token::OperatorParen(ArithmeticOperandParen::Left) => count += 1,
            Token::OperatorParen(ArithmeticOperandParen::Right) => count -= 1,
            _ => {}
        }
    }
    count == 0
}

fn check_block_parentheses(tokens: &Vec<Token>) -> bool
{
    let mut count = 0;
    for token in tokens.clone() {
        match token {
            Token::BlockParen(BlockParen::Left) => count += 1,
            Token::BlockParen(BlockParen::Right) => count -= 1,
            _ => {}
        }
    }

    count == 0
}

pub struct Runtime
{
    parser: Parser,
    interpreter: Interpreter,
}

pub fn runtime(contents: &str)
{
    let mut parser = Parser::new(contents);
    let mut tokens_list = parser.convert_token_list();

    let mut interpreter = Interpreter::new(tokens_list);
    let result = interpreter.run();
}

pub struct Parser
{
    contents: String,
}

impl Parser
{
    pub fn new(contents: &str) -> Self
    {
        Parser
        {
            contents: contents.to_string(),
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
                ';' => {
                    // ためていたトークンを追加
                    if token.len() > 0 {
                        result.push(token.clone());
                        token.clear();
                    }

                    // ; を追加
                    result.push(END_OF_EXPRESSION.to_string());
                }
                ',' =>
                    {
                        // ためていたトークンを追加
                        if token.len() > 0 {
                            result.push(token.clone());
                            token.clear();
                        }

                        // , を追加
                        result.push(COMMA.to_string());
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
            COMPARISON => Token::OperatorComparison(ComparisonOperand::Equal),
            GREATER_THAN => Token::OperatorComparison(ComparisonOperand::GreaterThan),
            LESS_THAN => Token::OperatorComparison(ComparisonOperand::LessThan),
            GREATER_THAN_OR_EQUAL => Token::OperatorComparison(ComparisonOperand::GreaterThanOrEqual),
            LESS_THAN_OR_EQUAL => Token::OperatorComparison(ComparisonOperand::LessThanOrEqual),
            NOT_EQUAL => Token::OperatorComparison(ComparisonOperand::NotEqual),
            BLOCK_LEFT_PAREN => Token::BlockParen(BlockParen::Left),
            BLOCK_RIGHT_PAREN => Token::BlockParen(BlockParen::Right),
            COMMA => Token::COMMA,
            END_OF_EXPRESSION => Token::EndOfExpression,
            FUNCTION => Token::Function,
            RETURN => Token::Return,
            _ => {
                // 整数か浮動小数点数か判定
                if let Ok(num) = token.parse::<i32>() {
                    Token::Value(Value::Int(num))
                } else if let Ok(num) = token.parse::<f32>() {
                    Token::Value(Value::Float(num))
                } else {
                    Token::Identifier(token.to_string())
                }
            }
        }
    }
    fn convert_token_list(&mut self) -> Vec<Vec<Token>>
    {
        let mut tokens_list = Vec::new();
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
            if !check_parentheses(&tokens) {
                panic!("括弧の数が正しくありません : {}", line);
            }

            tokens_list.push(tokens);
        }

        tokens_list
    }
}

enum EquationResult
{
    Continue,
    Return(Value),
}

pub struct Interpreter
{
    tokens_list: Vec<Vec<Token>>,
    identifiers: HashMap<String, Identifier>,
}


impl Interpreter
{
    pub fn new(tokens_list: Vec<Vec<Token>>) -> Self
    {
        Interpreter
        {
            tokens_list,
            identifiers: HashMap::new(),
        }
    }

    pub fn get_variable(&self, var: &str) -> Value
    {
        if let Some(Identifier::Variable(val)) = self.identifiers.get(var)
        {
            val.clone()
        } else {
            panic!("変数がありません : {}", var);
        }
    }

    fn extract_functions(&mut self)
    {
        // 新しいトークン列を作成し最終的にはこれを元のトークン列に置き換える
        let mut new_tokens_list = Vec::new();
        let mut function_tokens: Vec<Token> = Vec::new();
        let mut i = 0;

        // トークン列を走査して関数を抽出
        while i < self.tokens_list.len() {
            let tokens = self.tokens_list[i].clone();

            // token列にfuncが含まれている場合は関数を抽出し, 行を削除
            if let Some(Token::Function) = tokens.first() {
                for line in tokens.clone() {
                    function_tokens.push(line);
                }

                // この列に } が含まれている行を見つけるまで関数のトークンを追加
                while i < self.tokens_list.len() {
                    i += 1;

                    if i >= self.tokens_list.len() { // ここで範囲外アクセスを回避するためにチェックを追加
                        panic!("関数の終わりである }} が見つけられませんでした");
                    }

                    // 関数の終了を確認
                    if function_tokens.contains(&Token::BlockParen(BlockParen::Right)) {
                        break;
                    }

                    let tokens = self.tokens_list[i].clone();
                    let mut line = tokens.clone();
                    for line in line {
                        function_tokens.push(line);
                    }

                    // 関数の終了を確認
                    if function_tokens.contains(&Token::BlockParen(BlockParen::Right)) {
                        break;
                    }
                }

                // 関数を変換
                let function = self.convert_function(&mut function_tokens);
                self.identifiers.insert(function.name.clone(), Identifier::Function(function));

                function_tokens.clear();
            } else {
                new_tokens_list.push(tokens);
            }

            i += 1;
        }

        self.tokens_list = new_tokens_list;
    }

    fn convert_function(&mut self, mut tokens: &mut Vec<Token>) -> Function
    {
        let mut name = String::new();
        let mut args = Vec::new();
        let mut body = Vec::new();

        tokens.reverse();

        // {, } の数が正しいか確認
        if !check_block_parentheses(tokens) {
            panic!("ブロックの数が正しくありません");
        }

        // (, ) の数が正しいか確認
        if !check_parentheses(tokens) {
            panic!("括弧の数が正しくありません");
        }

        // １つ目のtokenが func であることを確認し, 除去
        if let Some(Token::Function) = tokens.pop() {} else {
            panic!("関数がありません");
        }

        // 関数名を取得
        if let Some(Token::Identifier(var)) = tokens.pop() {
            name = var.clone();
        } else {
            panic!("関数名がありません");
        }

        // 関数の引数を取得. ) が見つかるまで繰り返す
        while let Some(token) = tokens.pop() {
            match token {
                Token::OperatorParen(ArithmeticOperandParen::Left) => {}
                Token::OperatorParen(ArithmeticOperandParen::Right) => {
                    break;
                }
                Token::Identifier(var) => {
                    args.push(var.clone());
                }
                Token::COMMA => {}
                _ => {
                    panic!("引数がありません : {}", token);
                }
            }
        }

        // } が見つかるまで関数の本体を取得
        while let Some(token) = tokens.pop() {
            match token {
                Token::BlockParen(BlockParen::Left) => {}
                Token::BlockParen(BlockParen::Right) => {
                    break;
                }
                _ => {
                    body.push(token);
                }
            }
        }

        // 関数の引数を取得
        Function::new(name.as_str(), args, body)
    }

    pub fn run(&mut self) -> Value
    {
        let mut result = Value::Int(0);

        // 関数を抽出
        self.extract_functions();

        let mut i = 0;
        for mut tokens in self.tokens_list.clone() {
            i += 1;
            let return_state = self.equation(&mut tokens);
            if let EquationResult::Return(val) = return_state {
                result = val;
                break;
            }
        }
        result
    }
    
    fn statement(&mut self, tokens: Vec<Token>) -> Value
    {
        match tokens.first()
        {
            Some(Token::If) => self.if_statement(tokens),
            _ => panic!("不明なステートメント : {}", tokens.first().unwrap()),
        }
    }
    
    fn if_statement(&mut self, tokens: Vec<Token>)
    {

    }

    pub(crate) fn equation(&mut self, tokens: &mut Vec<Token>) -> EquationResult
    {
        // ; で終わっていることを確認
        if let Some(Token::EndOfExpression) = tokens.last() {
            tokens.remove(tokens.len() - 1);
        } else {
            panic!("式が ';' で終わっていません");
        }

        // 変数一つだけの場合はそのまま表示
        if tokens.len() == 1 {
            // Identifier ';' の場合
            self.variable(tokens.pop().unwrap());
            EquationResult::Continue
        } else {
            // Assignment or ReturnStatement

            // 先頭の token が return の場合
            if let Some(Token::Return) = tokens.first() {
                let mut return_tokens = tokens.clone();
                return_tokens.remove(0);
                return_tokens.reverse();

                let result = self.expression(&mut return_tokens);

                EquationResult::Return(result)
            } else {
                self.assignment(tokens);
                EquationResult::Continue
            }
        }
    }


    fn check_end_of_expression(&self, t: Option<Token>) -> bool
    {
        match t
        {
            Some(Token::EndOfExpression) => true,
            _ => panic!("式が ';' 終わっていません"),
        }
    }


    fn assignment(&mut self, tokens: &mut Vec<Token>)
    {
        tokens.reverse();
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

        let result = self.expression(tokens);

        // 代入先が変数であることを確認
        match first
        {
            Token::Identifier(var) =>
                {
                    self.identifiers.insert(var, Identifier::Variable(result));
                }
            _ =>
                {
                    panic!("代入先が変数ではありません : {}", first);
                }
        }
    }

    fn expression(&mut self, tokens: &mut Vec<Token>) -> Value
    {
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

            result = Value::Int(comparison as i32);
            result
        } else {
            result
        }
    }

    fn variable(&mut self, token: Token)
    {
        match token {
            Token::Identifier(ref var) => {
                if let Some(val) = self.identifiers.get(var.as_str()) {
                    match val {
                        Identifier::Variable(val) => println!("{}", val),
                        Identifier::Function(_) => panic!("関数は変数として使用できません"),
                    }
                } else {
                    panic!("変数がありません : {}", var);
                }
            }
            _ => {}
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
                Token::EndOfExpression =>
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

    fn function_call(&mut self, mut function: Function, token: &mut Vec<Token>) -> Value
    {
        let mut arguments = Vec::new();
        // ) が見つかるまで Token を除去
        while let Some(t) = token.pop()
        {
            match t
            {
                Token::OperatorParen(ArithmeticOperandParen::Right) => break,
                Token::COMMA => {}
                Token::Value(val) => arguments.push(val),
                Token::Identifier(var) =>
                    {
                        if let Some(val) = self.identifiers.get(&var)
                        {
                            match val
                            {
                                Identifier::Variable(val) => arguments.push(val.clone()),
                                _ => panic!("関数の引数に関数を使用することはできません : {:?}", val),
                            }
                        } else {
                            panic!("変数がありません : {}", var);
                        }
                    }
                _ => {}
            }
        }

        // 関数を実行
        function.run(arguments)
    }
    pub(crate) fn factor(&mut self, tokens: &mut Vec<Token>) -> Value
    {
        let token = tokens.pop().unwrap();
        match token
        {
            Token::Value(val) => val,
            Token::Identifier(var) =>
                {
                    if let Some(val) = self.identifiers.get(&var)
                    {
                        match val
                        {
                            Identifier::Variable(val) => val.clone(),
                            Identifier::Function(func) => {
                                // 関数を実行して結果を返す
                                self.function_call(func.clone(), tokens)
                            }
                        }
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
        let mut parser = Parser::new(program);
        parser.parse_line(program)
    }

    fn run_program(program: &str) -> Interpreter {
        let mut parser = Parser::new(program);
        let tokens_list = parser.convert_token_list();
        let mut interpreter = Interpreter::new(tokens_list);
        interpreter.run();
        interpreter
    }

    #[test]
    fn test_extract_function()
    {
        let source_code = "
            func add(a, b) {
                c = a + b;
                return c;
            }
        ";

        let mut parser = Parser::new(source_code);
        let mut tokens_list = parser.convert_token_list();

        let flat_tokens: Vec<Token> = tokens_list.iter().flat_map(|x| x.clone()).collect();
        assert_eq!(
            flat_tokens,
            vec![
                Token::Function,
                Token::Identifier("add".to_string()),
                Token::OperatorParen(ArithmeticOperandParen::Left),
                Token::Identifier("a".to_string()),
                Token::COMMA,
                Token::Identifier("b".to_string()),
                Token::OperatorParen(ArithmeticOperandParen::Right),
                Token::BlockParen(BlockParen::Left),
                Token::Identifier("c".to_string()),
                Token::Assign,
                Token::Identifier("a".to_string()),
                Token::OperatorHead(ArithmeticOperandHead::Plus),
                Token::Identifier("b".to_string()),
                Token::EndOfExpression,
                Token::Return,
                Token::Identifier("c".to_string()),
                Token::EndOfExpression,
                Token::BlockParen(BlockParen::Right),
            ]
        );

        let mut interpreter = Interpreter::new(tokens_list);
        interpreter.extract_functions();

        if let Some(Identifier::Function(function)) = interpreter.identifiers.get_mut("add")
        {
            assert_eq!(function.name, "add");
            assert_eq!(function.args, vec!["a", "b"]);
            assert_eq!(function.body, vec![
                Token::Identifier("c".to_string()),
                Token::Assign,
                Token::Identifier("a".to_string()),
                Token::OperatorHead(ArithmeticOperandHead::Plus),
                Token::Identifier("b".to_string()),
                Token::EndOfExpression,
                Token::Return,
                Token::Identifier("c".to_string()),
                Token::EndOfExpression,
            ]);

            // 関数を実行
            let result = function.run(vec![Value::Int(5), Value::Int(10)]);
            assert_eq!(result, Value::Int(15));
        } else {
            panic!("関数がありません");
        }
    }

    #[test]
    fn test_run_function()
    {
        let source_code = "
            func average(a, b, c, d) {
                sum = a + b + c + d;
                return sum / 4;
            }
            a = 10;
            b = 15;
            c = 20;
            d = 25;
            result = average(a, b, c, d);
        ";

        let interpreter = run_program(source_code);
        assert_eq!(interpreter.get_variable("a"), Value::Int(10));
        assert_eq!(interpreter.get_variable("b"), Value::Int(15));
        assert_eq!(interpreter.get_variable("c"), Value::Int(20));
        assert_eq!(interpreter.get_variable("d"), Value::Int(25));

        assert_eq!(interpreter.get_variable("result"), Value::Int(17));

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn run_program(program: &str) -> Interpreter {
            let mut parser = Parser::new(program);
            let tokens_list = parser.convert_token_list();
            let mut interpreter = Interpreter::new(tokens_list);
            interpreter.run();
            interpreter
        }

        #[test]
        fn test_comprehensive_scenarios() {
            let interpreter = run_program("
            # 四則演算
            a = 5 + 3 * 2;      # a = 11
            b = (4 + 6) / 2;    # b = 5
            c = 5.5 + 2.5;      # c = 8.0
            d = 10 % 3;         # d = 1

            # 比較演算
            e = 10 > 5;         # e = 1 (true)
            f = 10.0 <= 10.0;   # f = 1 (true)
            g = 3 != 3;         # g = 0 (false)

            # 変数代入と更新
            h = a + b;          # h = 16
            a = 100;            # a = 100 (上書き)

            # 関数定義と呼び出し
            func add(x, y) {
                a = x + y;
                return a;
            }
            i = add(1, 2);      # i = 3

            # 関数の引数に変数を使用
            j = add(a, h);      # j = 100 + 16 = 116

            k = (1 + 2 + 3 + 4) / 4;

            # 最後に評価した値を取得
            return k;

            # ここには到達しない
            l = 100;
        ");

            // テスト結果の検証
            assert_eq!(interpreter.get_variable("a"), Value::Int(100));
            assert_eq!(interpreter.get_variable("b"), Value::Int(5));
            assert_eq!(interpreter.get_variable("c"), Value::Float(8.0));
            assert_eq!(interpreter.get_variable("d"), Value::Int(1));
            assert_eq!(interpreter.get_variable("e"), Value::Int(1));
            assert_eq!(interpreter.get_variable("f"), Value::Int(1));
            assert_eq!(interpreter.get_variable("g"), Value::Int(0));
            assert_eq!(interpreter.get_variable("h"), Value::Int(16));
            assert_eq!(interpreter.get_variable("i"), Value::Int(3));
            assert_eq!(interpreter.get_variable("j"), Value::Int(116));
            assert_eq!(interpreter.get_variable("k"), Value::Int(2));

            // k は見つからないことを確認
            assert_eq!(interpreter.identifiers.get("l"), None);
        }
    }
}

#[cfg(test)]
mod error_tests {
    use super::*;

    fn run_program(program: &str) -> Interpreter {
        let mut parser = Parser::new(program);
        let tokens_list = parser.convert_token_list();
        let mut interpreter = Interpreter::new(tokens_list);
        interpreter.run();
        interpreter
    }

    #[test]
    #[should_panic(expected = "0で割ることはできません")]
    fn test_division_by_zero() {
        let _interpreter = run_program("
            a = 10;
            b = 0;
            c = a / b;
        ");
    }

    #[test]
    #[should_panic(expected = "変数がありません : undefined")]
    fn test_undefined_variable() {
        let _interpreter = run_program("a = undefined + 1;");
    }

    #[test]
    #[should_panic(expected = "代入演算子がありません")]
    fn test_invalid_operator_usage() {
        let _interpreter = run_program("
            a = 5;
            a +;
        ");
    }

    #[test]
    #[should_panic(expected = "括弧の数が正しくありません : a = (1 + 2 * 3;")]
    fn test_unmatched_parentheses() {
        let _interpreter = run_program("a = (1 + 2 * 3;");
    }

    #[test]
    #[should_panic(expected = "整数型以外の演算はできません")]
    fn test_invalid_modulo_operation() {
        let _interpreter = run_program("
            a = 5;
            b = 2.5;
            c = a % b;
        ");
    }

    #[test]
    #[should_panic(expected = "関数の終わりである } が見つけられませんでした")]
    fn test_invalid_block_structure() {
        let _interpreter = run_program("
            func invalid() {
                a = 10;
            ");
    }
}
