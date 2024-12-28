use std::collections::HashMap;

/// BNFに基づく演算子の定義
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    // 算術演算子
    Plus,        // '+'
    Minus,   // '-'
    Multiply,   // '*'
    Divide,     // '/'
    Modulo,     // '%'

    // 比較演算子
    LessThan,       // '<'
    GreaterThan,    // '>'
    LessThanOrEqual, // '<='
    GreaterThanOrEqual, // '>='
    Equal,          // '=='
    NotEqual,       // '!='

    // 論理演算子
    LogicalOr,  // '||'
    LogicalAnd, // '&&'
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Minus,   // '-'
    LogicalNot, // '!'
}


impl Operator {
    /// 演算子に対応する文字列を返す
    pub fn as_str(&self) -> &'static str {
        match self {
            // 算術演算子
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Modulo => "%",

            // 比較演算子
            Operator::LessThan => "<",
            Operator::GreaterThan => ">",
            Operator::LessThanOrEqual => "<=",
            Operator::GreaterThanOrEqual => ">=",
            Operator::Equal => "==",
            Operator::NotEqual => "!=",

            // 論理演算子
            Operator::LogicalOr => "||",
            Operator::LogicalAnd => "&&",
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    Void,
    Int,
    Float,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Integer(i32),
    Float(f64),
}

/// トークン
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // 識別子やリテラル
    Identifier(String),        // 変数や関数名
    Constant(Constant),         // 定数

    // 型指定子
    Type(ValueType),                // 型指定子

    // 区切り記号やその他の構造
    Comma,                     // `,`
    Semicolon,                 // `;`
    LeftParen,                 // `(`
    RightParen,                // `)`
    LeftBracket,               // `[`
    RightBracket,              // `]`
    LeftBrace,                 // `{`
    RightBrace,                // `}`

    // 制御構造
    If,                        // `if`
    Else,                      // `else`
    While,                     // `while`
    For,                       // `for`

    // 演算子
    Operator(Operator),        // 演算子を含む
    UnaryOperator(UnaryOperator), // 単項演算子
    
    // 代入演算子
    Assign,                    // `=`

    // jump
    Return,                    // `return`
    Continue,                  // `continue`
    Break,                     // `break`

    // その他
    Unknown,                   // 不明なトークン
}

impl Token
{
    pub fn from_keyword(keyword: &str) -> Option<Token> {
        match keyword {
            "int" => Some(Token::Type(ValueType::Int)),
            "float" => Some(Token::Type(ValueType::Float)),
            "void" => Some(Token::Type(ValueType::Void)),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "while" => Some(Token::While),
            "return" => Some(Token::Return),
            "continue" => Some(Token::Continue),
            "break" => Some(Token::Break),
            "for" => Some(Token::For),

            // 数値の場合
            _ if keyword.parse::<i64>().is_ok() =>
                Some(Token::Constant(Constant::Integer(keyword.parse::<i32>().unwrap()))),
            _ if keyword.parse::<f64>().is_ok() =>
                Some(Token::Constant(Constant::Float(keyword.parse::<f64>().unwrap()))),
            _ => None,
        }
    }

    pub fn is_floating_constant(keyword: &str) -> bool {
        keyword.contains('.')
    }
}

#[derive(Debug, Clone)]
pub struct Macro
{
    name: String,
    value: String,
    start_line: usize,
    end_line: usize,
}


#[derive(Debug, Clone)]
pub struct Lexer {
    sentence: String,
    position: usize,
    tokens: Vec<Token>,
    token_str: String,
    line_num: usize, // プログラムの行数
    macros: Vec<Macro>,
}

impl Lexer
{
    pub fn new(sentence: String) -> Lexer {

        // 文字列の行数を取得
        let line_num = sentence.lines().count();

        Lexer {
            sentence,
            position: 0,
            tokens: Vec::new(),
            token_str: String::new(),
            line_num,
            macros: Vec::new(),
        }
    }

    fn reset_position(&mut self)
    {
        self.position = 0;
        self.token_str.clear();
        self.tokens.clear();
    }

    fn remove_comments(&mut self)
    {
        let mut new_sentence = String::new();
        let chars: Vec<char> = self.sentence.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // 1. "//" コメントの検知
            if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
                // "//" が始まったら、改行か入力末まで読み飛ばす
                i += 2;
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            // 2. "/* ... */" コメントの検知
            else if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '*' {
                // "/*" が始まったら、"*/" が出るか入力末まで読み飛ばす
                i += 2;
                while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                    i += 1;
                }
                // "*/" の分も読み飛ばす (まだ入力が続いていれば)
                if i + 1 < chars.len() {
                    i += 2;
                }
            }
            // 3. 通常の文字として読み込む
            else {
                new_sentence.push(chars[i]);
                i += 1;
            }
        }

        self.sentence = new_sentence;
    }

    /// マクロ定義
    /// #define マクロ名 マクロの定義
    fn macro_define(&mut self, line_count: usize)
    {
        // '#' は既に読み込まれているので、define を読み込む
        let mut define = String::new();
        loop
        {
            let c = match self.next_char()
            {
                Some(c) => c,
                None => break,
            };

            if c == ' '
            {
                break;
            }

            define.push(c);
        }

        if define != "define"
        {
            panic!("Unknown macro: {:?}", define);
        }

        // マクロ名を取得する
        let mut macro_name = String::new();
        loop
        {
            let c = match self.next_char()
            {
                Some(c) => c,
                None => break,
            };

            if c == ' '
            {
                break;
            }

            macro_name.push(c);
        }

        // マクロの定義を取得する
        let mut macro_value = String::new();
        loop
        {
            let c = match self.next_char()
            {
                Some(c) => c,
                None => break,
            };

            if c == '\n'
            {
                break;
            }

            macro_value.push(c);
        }

        self.macros.push(Macro {
            name: macro_name,
            value: macro_value,
            start_line: line_count,
            end_line: self.line_num,
        });
    }

    // 定義したマクロを元に置換
    fn macro_replace(&mut self)
    {
        let mut new_sentence = String::new();

        // マクロの定義後から undef までの行を取得し置換する
        let lines = self.sentence.lines().collect::<Vec<&str>>();
        for (i, line) in lines.iter().enumerate()
        {
            let mut is_replaced = false;
            for m in &self.macros
            {
                if line.contains(&m.name)
                {
                    let replaced_line = line.replace(&m.name, &m.value);
                    new_sentence.push_str(&replaced_line);
                    new_sentence.push('\n');
                    is_replaced = true;
                    break;
                }
            }

            if !is_replaced
            {
                new_sentence.push_str(line);
                new_sentence.push('\n');
            }
        }

        self.sentence = new_sentence;
    }

    // マクロを見つけて置換する
    fn preprocess(&mut self)
    {
        // マクロは行の先頭に書かれていない場合はエラーとする
        let mut is_first = true;
        let mut line_count = 0;

        loop
        {
            let c = match self.next_char()
            {
                Some(c) => c,
                None => break,
            };

            match c
            {
                '\n' => {
                    // 行末まで読み込んだら値をクリアする.
                    self.token_str.clear();
                    is_first = true;
                    line_count += 1;
                }
                '#' => {
                    // マクロの定義
                    if self.token_str.len() == 0
                    {
                        // 行の先頭の場合はマクロ定義
                        self.macro_define(line_count);
                    }

                    is_first = false;
                }
                'a'..='z' | 'A'..='Z' | '_' =>
                    {
                        is_first = false;
                    }
                '0'..='9' =>
                    {
                        is_first = false;
                    }
                _ => {}
            }
        }

        // マクロを置換
        self.macro_replace();

        // # で始まる行をすべて削除
        self.sentence = self.sentence.lines().filter(|line| !line.starts_with("#")).collect::<Vec<&str>>().join("\n");
    }

    pub fn tokenize(&mut self)
    {
        // コメントを削除
        self.reset_position();
        self.remove_comments();

        // マクロを置換
        self.reset_position();
        self.preprocess();

        // 処理をはじめから行うために位置をリセット
        self.reset_position();

        loop {
            let c = match self.next_char() {
                Some(c) => c,
                None => break,
            };

            match c
            {
                'a'..='z' | 'A'..='Z' | '_' =>
                    {
                        self.add_char(c);
                    }
                '0'..='9' =>
                    {
                        self.add_char(c);
                    }
                '.' =>
                    {
                        self.add_char(c);
                    }
                ' ' =>
                    {
                        self.add_token();
                    }
                '(' =>
                    {
                        self.add_token();

                        self.tokens.push(Token::LeftParen);
                    }
                ')' =>
                    {
                        self.add_token();
                        self.tokens.push(Token::RightParen);
                    }
                '{' =>
                    {
                        self.add_token();
                        self.tokens.push(Token::LeftBrace);
                    }
                '}' =>
                    {
                        self.add_token();
                        self.tokens.push(Token::RightBrace);
                    }
                ';' =>
                    {
                        self.add_token();
                        self.tokens.push(Token::Semicolon);
                    }
                ',' =>
                    {
                        self.add_token();
                        self.tokens.push(Token::Comma);
                    }
                '[' =>
                    {
                        self.add_token();
                        self.tokens.push(Token::LeftBracket);
                    }
                ']' =>
                    {
                        self.add_token();
                        self.tokens.push(Token::RightBracket);
                    }
                '\n' =>
                    {
                        self.add_token();
                    }
                '=' =>
                    {
                        // 次のトークンを取得して、'=' かどうか判定
                        let next_char = self.next_char();
                        match next_char {
                            Some('=') =>
                                {
                                    self.add_token();
                                    self.tokens.push(Token::Operator(Operator::Equal));
                                }
                            _ =>
                                {
                                    self.add_token();
                                    self.tokens.push(Token::Assign);
                                    self.back_char();
                                }
                        }
                    }
                '|' =>
                    {
                        // もう一文字取得して、'|' かどうか判定
                        let next_char = self.next_char();
                        match next_char {
                            Some('|') =>
                                {
                                    self.add_token();
                                    self.tokens.push(Token::Operator(Operator::LogicalOr));
                                }
                            _ =>
                                {
                                    panic!("Unknown character: {:?}", c);
                                }
                        }
                    }
                '!' =>
                    {
                        // もう一文字取得して、'=' かどうか判定
                        let next_char = self.next_char();
                        match next_char {
                            Some('=') =>
                                {
                                    self.add_token();
                                    self.tokens.push(Token::Operator(Operator::NotEqual));
                                }
                            _ =>
                                {
                                    self.add_token();
                                    self.tokens.push(Token::UnaryOperator(UnaryOperator::LogicalNot));
                                }
                        }
                    }
                '+' | '*' | '/' | '%' =>
                    {
                        self.add_token();

                        self.tokens.push(Token::Operator(match c {
                            '+' => Operator::Plus,
                            '*' => Operator::Multiply,
                            '/' => Operator::Divide,
                            '%' => Operator::Modulo,
                            _ => unreachable!(),
                        }));
                    }
                '-' =>
                    {
                        self.add_token();

                        // 一個前のトークンが Identifier か定数の場合は Operator::Minus
                        if let Some(token) = self.tokens.last() {
                            match token {
                                Token::Identifier(_) | Token::Constant(_) => {
                                    self.tokens.push(Token::Operator(Operator::Minus));
                                }
                                _ => {
                                    self.tokens.push(Token::UnaryOperator(UnaryOperator::Minus));
                                }
                            }
                        } else {
                            self.tokens.push(Token::UnaryOperator(UnaryOperator::Minus));
                        }
                    }
                '&' =>
                    {
                        // もう一文字取得して、'&' かどうか判定
                        let next_char = self.next_char();
                        match next_char {
                            Some('&') =>
                                {
                                    self.add_token();
                                    self.tokens.push(Token::Operator(Operator::LogicalAnd));
                                }
                            _ =>
                                {
                                    panic!("Unknown character: {:?}", c);
                                }
                        }
                    }
                '>' | '<' =>
                    {
                        // もう一文字取得して、'=' かどうか判定
                        let next_char = self.next_char();
                        match next_char {
                            Some('=') =>
                                {
                                    self.add_token();
                                    self.tokens.push(Token::Operator(match c {
                                        '>' => Operator::GreaterThanOrEqual,
                                        '<' => Operator::LessThanOrEqual,
                                        _ => unreachable!(),
                                    }));
                                }
                            _ =>
                                {
                                    self.add_token();
                                    self.tokens.push(Token::Operator(match c {
                                        '>' => Operator::GreaterThan,
                                        '<' => Operator::LessThan,
                                        _ => unreachable!(),
                                    }));
                                }
                        }
                    }
                _ =>
                    {
                        panic!("Unknown character: {:?}", c);
                    }
            }
        }
    }

    fn add_token(&mut self)
    {
        // トークンを追加
        if !self.token_str.is_empty() {
            if let Some(token) = Token::from_keyword(&self.token_str) {
                self.tokens.push(token);
            } else {
                self.tokens.push(Token::Identifier(self.token_str.clone()));
            }

            // トークン文字列をクリア
            self.token_str.clear();
        }
    }

    fn add_char(&mut self, c: char)
    {
        self.token_str.push(c);
    }

    pub fn show_tokens(&self)
    {
        for token in &self.tokens {
            println!("{:?}", token);
        }
    }

    fn next_char(&mut self) -> Option<char>
    {
        // 文字列の最後まで読み込んだらNoneを返す
        if self.position >= self.sentence.len() {
            return None;
        }

        let result = self.sentence.chars().nth(self.position);
        self.position += 1;

        result
    }

    /// 次の文字を取得するが、文字列を進めない
    fn peek_char(&self) -> Option<char>
    {
        if self.position >= self.sentence.len() {
            return None;
        }

        self.sentence.chars().nth(self.position)
    }

    /// 文字を一つ戻す
    fn back_char(&mut self)
    {
        self.position -= 1;
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::lexical::Token::Identifier;
    use super::*;

    #[test]
    fn test_lexer() {
        let sentence = "

#define N 10
int x = -0;
/**
 * Add two integers
 */
int add(int a, int b) {
    int result;
    result = a + b;
    return result;
}

float multiply(float x, float y) {
    float product = x * y;
    return product;
}

void print_numbers(int n) {
    int i = 0;
    while (i < n) {
        i = i + 1;
        continue;
    }
}

int main() {
    int sum;
    float product;
    sum = add(5, 10);
    product = multiply(2.5, 4.0);

    if (sum > N) {
        sum = sum - 1;
    } else {
        sum = sum + 1;
    }
    /// これもコメント

    print_numbers(5);

    if (sum > -N) {
        return 1;
    } else if (sum <= -N) {
        return 2;
    } else {
        return 0;
    } 
}
".to_string();

        let mut lexer = Lexer::new(sentence);
        lexer.tokenize();
        let tokens = lexer.tokens();

        let result = vec![
            // int x = 0;
            Token::Type(ValueType::Int),
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,

            // int add(int a, int b) {
            Token::Type(ValueType::Int),
            Token::Identifier("add".to_string()),
            Token::LeftParen,
            Token::Type(ValueType::Int),
            Token::Identifier("a".to_string()),
            Token::Comma,
            Token::Type(ValueType::Int),
            Token::Identifier("b".to_string()),
            Token::RightParen,
            Token::LeftBrace,

            // int result;
            Token::Type(ValueType::Int),
            Token::Identifier("result".to_string()),
            Token::Semicolon,

            // result = a + b;
            Token::Identifier("result".to_string()),
            Token::Assign,
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::Plus),
            Token::Identifier("b".to_string()),
            Token::Semicolon,

            // return result;
            Token::Return,
            Token::Identifier("result".to_string()),
            Token::Semicolon,

            // }
            Token::RightBrace,

            // float multiply(float x, float y) {
            Token::Type(ValueType::Float),
            Token::Identifier("multiply".to_string()),
            Token::LeftParen,
            Token::Type(ValueType::Float),
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Type(ValueType::Float),
            Token::Identifier("y".to_string()),
            Token::RightParen,
            Token::LeftBrace,

            // float product = x * y;
            Token::Type(ValueType::Float),
            Token::Identifier("product".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Multiply),
            Token::Identifier("y".to_string()),
            Token::Semicolon,

            // return product;
            Token::Return,
            Token::Identifier("product".to_string()),
            Token::Semicolon,

            // }
            Token::RightBrace,

            // void print_numbers(int n) {
            Token::Type(ValueType::Void),
            Token::Identifier("print_numbers".to_string()),
            Token::LeftParen,
            Token::Type(ValueType::Int),
            Token::Identifier("n".to_string()),
            Token::RightParen,
            Token::LeftBrace,

            // int i = 0;
            Token::Type(ValueType::Int),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,

            // while (i < n) {
            Token::While,
            Token::LeftParen,
            Token::Identifier("i".to_string()),
            Token::Operator(Operator::LessThan),
            Token::Identifier("n".to_string()),
            Token::RightParen,
            Token::LeftBrace,

            // i = i + 1;
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::Identifier("i".to_string()),
            Token::Operator(Operator::Plus),
            Token::Constant(Constant::Integer(1)),
            Token::Semicolon,

            // continue;
            Token::Continue,
            Token::Semicolon,

            // }
            Token::RightBrace,

            // }
            Token::RightBrace,

            // int main() {
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,

            // int sum;
            Token::Type(ValueType::Int),
            Token::Identifier("sum".to_string()),
            Token::Semicolon,

            // float product;
            Token::Type(ValueType::Float),
            Token::Identifier("product".to_string()),
            Token::Semicolon,

            // sum = add(5, 10);
            Token::Identifier("sum".to_string()),
            Token::Assign,
            Token::Identifier("add".to_string()),
            Token::LeftParen,
            Token::Constant(Constant::Integer(5)),
            Token::Comma,
            Token::Constant(Constant::Integer(10)),
            Token::RightParen,
            Token::Semicolon,

            // product = multiply(2.5, 4.0);
            Token::Identifier("product".to_string()),
            Token::Assign,
            Token::Identifier("multiply".to_string()),
            Token::LeftParen,
            Token::Constant(Constant::Float(2.5)),
            Token::Comma,
            Token::Constant(Constant::Float(4.0)),
            Token::RightParen,
            Token::Semicolon,

            // if (sum > 10) {
            Token::If,
            Token::LeftParen,
            Token::Identifier("sum".to_string()),
            Token::Operator(Operator::GreaterThan),
            Token::Constant(Constant::Integer(10)),
            Token::RightParen,
            Token::LeftBrace,

            // sum = sum - 1;
            Token::Identifier("sum".to_string()),
            Token::Assign,
            Token::Identifier("sum".to_string()),
            Token::Operator(Operator::Minus),
            Token::Constant(Constant::Integer(1)),
            Token::Semicolon,

            // } else {
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,

            // sum = sum + 1;
            Token::Identifier("sum".to_string()),
            Token::Assign,
            Token::Identifier("sum".to_string()),
            Token::Operator(Operator::Plus),
            Token::Constant(Constant::Integer(1)),
            Token::Semicolon,

            // }
            Token::RightBrace,

            // print_numbers(5);
            Token::Identifier("print_numbers".to_string()),
            Token::LeftParen,
            Token::Constant(Constant::Integer(5)),
            Token::RightParen,
            Token::Semicolon,

            // if (sum > -10) {
            Token::If,
            Token::LeftParen,
            Token::Identifier("sum".to_string()),
            Token::Operator(Operator::GreaterThan),
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Constant(Constant::Integer(10)),
            Token::RightParen,
            Token::LeftBrace,

            // return 1;
            Token::Return,
            Token::Constant(Constant::Integer(1)),
            Token::Semicolon,

            // } else if (sum <= -10) {
            Token::RightBrace,
            Token::Else,
            Token::If,
            Token::LeftParen,
            Token::Identifier("sum".to_string()),
            Token::Operator(Operator::LessThanOrEqual),
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Constant(Constant::Integer(10)),
            Token::RightParen,
            Token::LeftBrace,

            // return 2;
            Token::Return,
            Token::Constant(Constant::Integer(2)),
            Token::Semicolon,

            // } else {
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,

            // return 0;
            Token::Return,
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,

            // }
            Token::RightBrace,

            // }
            Token::RightBrace,
        ];

        for (i, token) in tokens.iter().enumerate() {
            println!("{:?} : {:?}", token, result[i]);
            assert_eq!(token, &result[i]);
        }
    }
}
