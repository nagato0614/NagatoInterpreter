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


pub struct Lexer {
    sentence: String,
    position: usize,
    tokens: Vec<Token>,
}

impl Lexer
{
    pub fn new(sentence: String) -> Lexer {
        Lexer {
            sentence,
            position: 0,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self)
    {}

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let sentence = "int main() { return 0; }".to_string();

        let mut lexer = Lexer::new(sentence);
        lexer.tokenize();
        let tokens = lexer.tokens();

        let result = vec![
            Token::Type(Type::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::IntegerConstant(0),
            Token::Semicolon,
            Token::RightBrace,
        ];

        assert_eq!(tokens, &result);
    }
}
