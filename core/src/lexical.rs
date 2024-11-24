/// BNFに基づく演算子の定義
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    // 算術演算子
    Add,        // '+'
    Subtract,   // '_'
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
    LogicalNot, // '!'
}

impl Operator {
    /// 演算子に対応する文字列を返す
    pub fn as_str(&self) -> &'static str {
        match self {
            // 算術演算子
            Operator::Add => "+",
            Operator::Subtract => "-",
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
            Operator::LogicalNot => "!",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Void,
    Int,
    Float,
}

/// トークン
#[derive(Debug, PartialEq)]
pub enum Token {
    // 識別子やリテラル
    Identifier(String),        // 変数や関数名
    IntegerConstant(i64),      // 整数リテラル
    FloatingConstant(f64),     // 浮動小数点リテラル

    // 型指定子
    Type(Type),                // 型指定子

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

    // 演算子（委譲）
    Operator(Operator),        // 演算子を含む

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
            "int" => Some(Token::Type(Type::Int)),
            "float" => Some(Token::Type(Type::Float)),
            "void" => Some(Token::Type(Type::Void)),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "while" => Some(Token::While),
            "return" => Some(Token::Return),
            "continue" => Some(Token::Continue),
            "break" => Some(Token::Break),

            // 数値の場合
            _ if keyword.parse::<i64>().is_ok() => Some(Token::IntegerConstant(keyword.parse::<i64>().unwrap())),
            _ if keyword.parse::<f64>().is_ok() => Some(Token::FloatingConstant(keyword.parse::<f64>().unwrap())),
            _ => None,
        }
    }

    pub fn is_floating_constant(keyword: &str) -> bool {
        keyword.contains('.')
    }
}


pub struct Lexer {
    sentence: String,
    position: usize,
    tokens: Vec<Token>,
    token_str: String,
}

impl Lexer
{
    pub fn new(sentence: String) -> Lexer {
        Lexer {
            sentence,
            position: 0,
            tokens: Vec::new(),
            token_str: String::new(),
        }
    }

    pub fn tokenize(&mut self)
    {
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
                                    self.tokens.push(Token::Operator(Operator::LogicalNot));
                                }
                        }
                    }
                '+' | '-' | '*' | '/' | '%' =>
                    {
                        self.add_token();
                        self.tokens.push(Token::Operator(match c {
                            '+' => Operator::Add,
                            '-' => Operator::Subtract,
                            '*' => Operator::Multiply,
                            '/' => Operator::Divide,
                            '%' => Operator::Modulo,
                            _ => unreachable!(),
                        }));
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
            println!("token_str: {}", self.token_str);

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

    if (sum > 10) {
        sum = sum - 1;
    } else {
        sum = sum + 1;
    }

    print_numbers(5);

    if (sum > 10) {
        return 1;
    } else if (sum <= 10) {
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
            // int add(int a, int b) {
            Token::Type(Type::Int),
            Token::Identifier("add".to_string()),
            Token::LeftParen,
            Token::Type(Type::Int),
            Token::Identifier("a".to_string()),
            Token::Comma,
            Token::Type(Type::Int),
            Token::Identifier("b".to_string()),
            Token::RightParen,
            Token::LeftBrace,

            // int result;
            Token::Type(Type::Int),
            Token::Identifier("result".to_string()),
            Token::Semicolon,

            // result = a + b;
            Token::Identifier("result".to_string()),
            Token::Assign,
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::Add),
            Token::Identifier("b".to_string()),
            Token::Semicolon,

            // return result;
            Token::Return,
            Token::Identifier("result".to_string()),
            Token::Semicolon,

            // }
            Token::RightBrace,

            // float multiply(float x, float y) {
            Token::Type(Type::Float),
            Token::Identifier("multiply".to_string()),
            Token::LeftParen,
            Token::Type(Type::Float),
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Type(Type::Float),
            Token::Identifier("y".to_string()),
            Token::RightParen,
            Token::LeftBrace,

            // float product = x * y;
            Token::Type(Type::Float),
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
            Token::Type(Type::Void),
            Token::Identifier("print_numbers".to_string()),
            Token::LeftParen,
            Token::Type(Type::Int),
            Token::Identifier("n".to_string()),
            Token::RightParen,
            Token::LeftBrace,

            // int i = 0;
            Token::Type(Type::Int),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::IntegerConstant(0),
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
            Token::Operator(Operator::Add),
            Token::IntegerConstant(1),
            Token::Semicolon,

            // continue;
            Token::Continue,
            Token::Semicolon,

            // }
            Token::RightBrace,

            // }
            Token::RightBrace,

            // int main() {
            Token::Type(Type::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,

            // int sum;
            Token::Type(Type::Int),
            Token::Identifier("sum".to_string()),
            Token::Semicolon,

            // float product;
            Token::Type(Type::Float),
            Token::Identifier("product".to_string()),
            Token::Semicolon,

            // sum = add(5, 10);
            Token::Identifier("sum".to_string()),
            Token::Assign,
            Token::Identifier("add".to_string()),
            Token::LeftParen,
            Token::IntegerConstant(5),
            Token::Comma,
            Token::IntegerConstant(10),
            Token::RightParen,
            Token::Semicolon,

            // product = multiply(2.5, 4.0);
            Token::Identifier("product".to_string()),
            Token::Assign,
            Token::Identifier("multiply".to_string()),
            Token::LeftParen,
            Token::FloatingConstant(2.5),
            Token::Comma,
            Token::FloatingConstant(4.0),
            Token::RightParen,
            Token::Semicolon,

            // if (sum > 10) {
            Token::If,
            Token::LeftParen,
            Token::Identifier("sum".to_string()),
            Token::Operator(Operator::GreaterThan),
            Token::IntegerConstant(10),
            Token::RightParen,
            Token::LeftBrace,

            // sum = sum - 1;
            Token::Identifier("sum".to_string()),
            Token::Assign,
            Token::Identifier("sum".to_string()),
            Token::Operator(Operator::Subtract),
            Token::IntegerConstant(1),
            Token::Semicolon,

            // } else {
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,

            // sum = sum + 1;
            Token::Identifier("sum".to_string()),
            Token::Assign,
            Token::Identifier("sum".to_string()),
            Token::Operator(Operator::Add),
            Token::IntegerConstant(1),
            Token::Semicolon,

            // }
            Token::RightBrace,

            // print_numbers(5);
            Token::Identifier("print_numbers".to_string()),
            Token::LeftParen,
            Token::IntegerConstant(5),
            Token::RightParen,
            Token::Semicolon,

            // if (sum > 10) {
            Token::If,
            Token::LeftParen,
            Token::Identifier("sum".to_string()),
            Token::Operator(Operator::GreaterThan),
            Token::IntegerConstant(10),
            Token::RightParen,
            Token::LeftBrace,

            // return 1;
            Token::Return,
            Token::IntegerConstant(1),
            Token::Semicolon,

            // } else if (sum <= 10) {
            Token::RightBrace,
            Token::Else,
            Token::If,
            Token::LeftParen,
            Token::Identifier("sum".to_string()),
            Token::Operator(Operator::LessThanOrEqual),
            Token::IntegerConstant(10),
            Token::RightParen,
            Token::LeftBrace,

            // return 2;
            Token::Return,
            Token::IntegerConstant(2),
            Token::Semicolon,

            // } else {
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,

            // return 0;
            Token::Return,
            Token::IntegerConstant(0),
            Token::Semicolon,

            // }
            Token::RightBrace,

            // }
            Token::RightBrace,
        ];

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token, &result[i]);
        }
    }
}
