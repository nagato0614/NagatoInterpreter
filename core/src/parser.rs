use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::lexical::Token;
use crate::lexical::Operator;

#[derive(Debug, Clone)]
pub enum Leaf
{
    Token(Token),
    Node(Rc<RefCell<Node>>),
    Declaration,
    FunctionDefinition,
    UnaryExpression,
}

/// 構文木
#[derive(Debug, Clone)]
pub struct Node {
    lhs: Option<Rc<RefCell<Node>>>,
    rhs: Option<Rc<RefCell<Node>>>,
    val: Option<Leaf>,
    parent: Weak<RefCell<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            lhs: None,
            rhs: None,
            val: None,
            parent: Weak::new(),
        }
    }

    pub fn show_node(root: &Node)
    {
        if let Some(leaf) = &root.val {
            match leaf {
                Leaf::Token(token) => {
                    println!("{:?}", token);
                }
                Leaf::Declaration => {
                    println!("Declaration");
                }
                Leaf::FunctionDefinition => {
                    println!("FunctionDefinition");
                }
                _ => {
                    println!("Unknown");
                }
            }
        }

        if let Some(lhs) = &root.lhs {
            print!("lhs : ");
            Node::show_node(&lhs.borrow());
        }

        if let Some(rhs) = &root.rhs {
            print!("rhs : ");
            Node::show_node(&rhs.borrow());
        }
    }

    pub fn set_parent(&mut self, parent: &Rc<RefCell<Node>>) {
        self.parent = Rc::downgrade(&parent);
    }

    pub fn set_lhs(&mut self, node: Rc<RefCell<Node>>) {
        self.lhs = Some(node);
    }

    pub fn set_rhs(&mut self, node: Rc<RefCell<Node>>) {
        self.rhs = Some(node);
    }

    pub fn set_val(&mut self, leaf: Leaf) {
        self.val = Some(leaf);
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    roots: Vec<Rc<RefCell<Node>>>,
    token_index: usize,
}

impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            roots: Vec::new(),
            token_index: 0,
        }
    }

    fn get_next_token(&mut self) -> Option<Token>
    {
        if self.token_index < self.tokens.len() {
            println!("token_index: {}, Token : {:?}", self.token_index, self.tokens[self.token_index]);
            let result = Some(self.tokens[self.token_index].clone());
            self.token_index += 1;
            result
        } else {
            None
        }
    }

    fn get_next_token_without_increment(&self) -> Option<Token>
    {
        if self.token_index < self.tokens.len() {
            println!("token_index w: {}, Token : {:?}", self.token_index, self
                .tokens[self
                .token_index]);
            Some(self.tokens[self.token_index].clone())
        } else {
            None
        }
    }

    pub fn parse(&mut self)
    {
        self.translation_unit();
    }

    fn translation_unit(&mut self)
    {
        while self.token_index < self.tokens.len() {
            println!("TranslationUnit token_index: {}", self.token_index);
            // トークンがなくなるまで繰り返す
            self.external_declaration();
        }
    }

    /// 関数定義かグローバル変数定義かを判定する
    fn external_declaration(&mut self)
    {
        // 関数の場合は type_specifier, identify, ( となり '(' が続く場合は関数として処理する

        let type_specifier = self.tokens[self.token_index].clone();
        let identify = self.tokens[self.token_index + 1].clone();
        let next_token = self.tokens[self.token_index + 2].clone();

        if next_token == Token::LeftParen {
            println!("function_definition");
            self.function_definition();
        } else {
            println!("declaration : {}", self.token_index);
            self.declaration();
        }
    }

    fn function_definition(&mut self)
    {
        // 関数定義をパースする
        unimplemented!();
    }

    fn declaration(&mut self)
    {
        // グローバル変数定義をパースする
        let mut root = Rc::new(RefCell::new(Node::new()));
        root.borrow_mut().set_val(Leaf::Declaration);

        if let Some(left_node) = self.type_specifier(&root) {
            root.borrow_mut().set_lhs(left_node);
        }
        if let Some(right_node) = self.init_declarator(&root) {
            root.borrow_mut().set_rhs(right_node);
        }

        // 最後に ';' が来ることを確認
        if let Some(Token::Semicolon) = self.get_next_token() {
            // 何もしない
        } else {
            panic!("';' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        self.roots.push(root);
    }

    fn type_specifier(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(Token::Type(type_specifier)) = self.get_next_token()
        {
            node.borrow_mut().set_val(Leaf::Token(Token::Type(type_specifier)));
        } else {
            panic!("型が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        Some(node)
    }

    fn init_declarator(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(Token::Identifier(identify)) = self.get_next_token() {
            node.borrow_mut().set_val(Leaf::Token(Token::Identifier(identify)));
        } else {
            panic!("識別子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 次のトークンが '='の場合は initializer をパースする
        match self.get_next_token().unwrap() {
            Token::Assign => {
                // 今作成した node を 左の子ノードとして設定する
                let left_node = node.clone();
                node.borrow_mut().set_lhs(left_node);
                node.borrow_mut().set_val(Leaf::Token(Token::Assign));

                // 右の子ノードを設定する
                if let Some(right_node) = self.logical_or_expression(&node) {
                    node.borrow_mut().set_rhs(right_node);
                }
            }
            Token::Semicolon => {
                // 何もせずスキップする
                self.token_index += 1;
            }
            _ => {
                panic!("初期化子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
            }
        }

        Some(node)
    }


    fn logical_or_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(left_node) = self.logical_and_expression(&node) {

            // 次のトークンが '||' の場合は logical_or_expression をパースする
            if let Some(Token::Operator(Operator::LogicalOr)) = self.get_next_token_without_increment() {
                println!("logical_or_expression");
                node.borrow_mut().set_val(Leaf::Token(Token::Operator(Operator::LogicalOr)));
                node.borrow_mut().set_lhs(left_node);

                if let Some(right_node) = self.logical_or_expression(&node) {
                    node.borrow_mut().set_rhs(right_node);
                } else {
                    panic!("右のノードが見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }

                Some(node)
            } else {
                // 演算子がない場合は左のノードをそのまま返す
                Some(left_node)
            }
        } else {
            panic!("識別子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }
    }

    fn logical_and_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(left_node) = self.equality_expression(&node)
        {
            // 次のトークンが '&&' の場合は logical_and_expression をパースする
            if let Some(Token::Operator(Operator::LogicalAnd)) = self.get_next_token_without_increment() {
                println!("logical_and_expression");
                node.borrow_mut().set_val(Leaf::Token(Token::Operator(Operator::LogicalAnd)));
                node.borrow_mut().set_lhs(left_node);

                // 再度 logical_and_expression を呼び出す
                if let Some(right_node) = self.logical_and_expression(&mut node) {
                    node.borrow_mut().set_rhs(right_node);
                } else {
                    panic!("右のノードが見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }

                Some(node)
            } else {
                // 演算子がない場合は左のノードをそのまま返す
                Some(left_node)
            }
        } else {
            panic!("識別子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }
    }

    fn equality_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(left_node) = self.relational_expression(&node) {
            // '==' または '!=' の演算子を取得
            if let Some(operator) = self.get_next_token_without_increment()
                .and_then(|token| match token {
                    Token::Operator(Operator::Equal) => Some(Operator::Equal),
                    Token::Operator(Operator::NotEqual) => Some(Operator::NotEqual),
                    _ => None,
                })
            {
                println!("equality_expression");
                node.borrow_mut().set_val(Leaf::Token(Token::Operator(operator)));
                node.borrow_mut().set_lhs(left_node);

                // 再帰的に equality_expression を呼び出す
                if let Some(right_node) = self.equality_expression(&node) {
                    node.borrow_mut().set_rhs(right_node);
                } else {
                    panic!(
                        "右のノードが見つかりませんでした : {:?}",
                        self.tokens[self.token_index]
                    );
                }

                Some(node)
            } else {
                // 演算子がない場合は左のノードをそのまま返す
                Some(left_node)
            }
        } else {
            panic!(
                "識別子が見つかりませんでした : {:?}",
                self.tokens[self.token_index]
            );
        }
    }


    fn relational_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(left_node) = self.additive_expression(&mut node) {
            // '<' または '>' の演算子を取得
            if let Some(operator) = self.get_next_token_without_increment()
                .and_then(|token| match token {
                    Token::Operator(Operator::LessThan) => Some(Operator::LessThan),
                    Token::Operator(Operator::LessThanOrEqual) => Some(Operator::LessThanOrEqual),
                    Token::Operator(Operator::GreaterThan) => Some(Operator::GreaterThan),
                    Token::Operator(Operator::GreaterThanOrEqual) => Some(Operator::GreaterThanOrEqual),
                    _ => None,
                })
            {
                println!("relational_expression");
                node.borrow_mut().set_val(Leaf::Token(Token::Operator(operator)));
                node.borrow_mut().set_lhs(left_node);

                // 再帰的に relational_expression を呼び出す
                if let Some(right_node) = self.relational_expression(&mut node) {
                    node.borrow_mut().set_rhs(right_node);
                } else {
                    panic!("右のノードが見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }

                Some(node)
            } else {
                // 演算子がない場合は左のノードをそのまま返す
                Some(left_node)
            }
        } else {
            panic!("識別子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }
    }


    fn additive_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(left_node) = self.multiplicative_expression(&mut node) {
            // '+' または '-' の演算子を取得
            if let Some(operator) = self.get_next_token_without_increment()
                .and_then(|token| match token {
                    Token::Operator(Operator::Plus) => Some(Operator::Plus),
                    Token::Operator(Operator::Minus) => Some(Operator::Minus),
                    _ => None,
                })
            {
                println!("additive_expression");
                node.borrow_mut().set_val(Leaf::Token(Token::Operator(operator)));
                node.borrow_mut().set_lhs(left_node);

                // 再帰的に additive_expression を呼び出す
                if let Some(right_node) = self.additive_expression(&mut node) {
                    node.borrow_mut().set_rhs(right_node);
                } else {
                    panic!("右のノードが見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }

                Some(node)
            } else {
                // 演算子がない場合は左のノードをそのまま返す
                Some(left_node)
            }
        } else {
            panic!("識別子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }
    }

    fn multiplicative_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(left_node) = self.unary_expression(&mut node) {
            // '*' または '/' の演算子を取得
            if let Some(operator) = self.get_next_token_without_increment()
                .and_then(|token| match token {
                    Token::Operator(Operator::Multiply) => Some(Operator::Multiply),
                    Token::Operator(Operator::Divide) => Some(Operator::Divide),
                    _ => None,
                })
            {
                println!("multiplicative_expression");
                node.borrow_mut().set_val(Leaf::Token(Token::Operator(operator)));
                node.borrow_mut().set_lhs(left_node);

                // 再帰的に multiplicative_expression を呼び出す
                if let Some(right_node) = self.multiplicative_expression(&mut node) {
                    node.borrow_mut().set_rhs(right_node);
                } else {
                    panic!("右のノードが見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }

                Some(node)
            } else {
                // 演算子がない場合は左のノードをそのまま返す
                Some(left_node)
            }
        } else {
            panic!("識別子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }
    }

    fn unary_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        // 次のトークンを取得
        let next_token = self.get_next_token_without_increment();

        if let Some(Token::UnaryOperator(operator)) = next_token {
            // 単項演算子の場合
            node.borrow_mut().set_val(Leaf::UnaryExpression);
            self.postfix_expression(&mut node);
        } else {
            // 単項演算子でない場合は postfix_expression をパースする
            if let Some(postfix_node) = self.postfix_expression(&mut node) {
                node.borrow_mut().set_val(Leaf::Node(postfix_node));
            } else {
                panic!("識別子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
            }
        }

        None
    }

    fn postfix_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Node::new();

        None
    }

    fn primary_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Node::new();

        unimplemented!("primary_expression");
    }

    pub fn show_tree(&self)
    {
        for root in &self.roots {
            print!("root : ");
            Node::show_node(&root.borrow());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser()
    {
        let program = String::from("int a = 0;");

        let mut lexer = crate::lexical::Lexer::new(program);
        lexer.tokenize();

        lexer.show_tokens();

        println!("---");

        let tokens = lexer.tokens().clone();
        let mut parser = Parser::new(tokens);
        parser.parse();

        parser.show_tree();
    }
}
