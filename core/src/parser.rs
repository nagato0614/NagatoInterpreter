use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::lexical::{Constant, Token, UnaryOperator};
use crate::lexical::Operator;

#[derive(Debug, Clone)]
pub struct FunctionCall {
    // 呼び出す関数名
    name: String,

    // 引数のリスト, 0個以上の logical_or_expression が入る
    arguments: Vec<Rc<RefCell<Node>>>,
}

impl FunctionCall
{
    pub fn new(name: String) -> Self {
        FunctionCall {
            name,
            arguments: Vec::new(),
        }
    }

    pub fn add_argument(&mut self, argument: Rc<RefCell<Node>>) {
        self.arguments.push(argument);
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct Declaration {
    // 型
    type_specifier: Token,

    // 識別子
    identify: Token,

    // 初期化子
    initializer: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug, Clone)]
pub enum Leaf
{
    Token(Token),
    Node(Rc<RefCell<Node>>),
    Declaration(Token),
    FunctionDefinition,
    UnaryExpression(UnaryOperator),
    FunctionCall(FunctionCall),
    ArrayAccess,
    ParenthesizedExpression,
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

    pub fn val(&self) -> Option<&Leaf> {
        self.val.as_ref()
    }

    pub fn lhs(&self) -> Option<&Rc<RefCell<Node>>> {
        self.lhs.as_ref()
    }

    pub fn rhs(&self) -> Option<&Rc<RefCell<Node>>> {
        self.rhs.as_ref()
    }


    pub fn show_node(root: &Node)
    {
        if let Some(leaf) = &root.val {
            match leaf {
                Leaf::Token(token) => {
                    println!("{:?}", token);
                }
                Leaf::Declaration(declaration) => {
                    println!("Declaration [{:?}]", declaration);
                }
                Leaf::FunctionDefinition => {
                    println!("FunctionDefinition");
                }
                Leaf::UnaryExpression(operator) => {
                    println!("UnaryExpression [{:?}]", operator);
                }
                Leaf::FunctionCall(function_call) => {
                    println!("FunctionCall [{:?}]", function_call);
                }
                Leaf::ArrayAccess => {
                    println!("ArrayAccess [{:?}]", leaf);
                }
                Leaf::ParenthesizedExpression => {
                    println!("ParenthesizedExpression");
                }
                Leaf::Node(node) => {
                    Node::show_node(&node.borrow());
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

    pub fn root(&self) -> &Rc<RefCell<Node>> {
        self.roots.first().unwrap()
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
            println!("token_index without: {}, Token : {:?}", self.token_index, self
                .tokens[self
                .token_index]);
            Some(self.tokens[self.token_index].clone())
        } else {
            None
        }
    }

    fn token_index_increment(&mut self)
    {
        self.token_index += 1;
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

        // declaration の値として型が入る
        if let Some(type_specifier) = self.get_next_token() {
            root.borrow_mut().set_val(Leaf::Declaration(type_specifier));
        } else {
            panic!("型が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // declaration の左辺として識別子が入る
        if let Some(Token::Identifier(identifier)) = self.get_next_token() {
            let left_node = Rc::new(RefCell::new(Node::new()));
            left_node.borrow_mut().set_val(Leaf::Token(Token::Identifier(identifier)));
            root.borrow_mut().set_lhs(left_node);
        } else {
            panic!("識別子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 次のトークンが '=' かどうか
        if let Some(next_token) = self.get_next_token()
        {
            match next_token
            {
                Token::Assign => {
                    // '=' の場合は initializer をパースする
                    if let Some(initializer) = self.logical_or_expression(&root) {
                        println!("initializer");
                        root.borrow_mut().set_rhs(initializer);
                    }

                    // ';' が来ることを確認
                    if let Some(Token::Semicolon) = self.get_next_token() {
                        println!("semicolon");
                        // 何もしない
                    } else {
                        panic!("';' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
                    }
                }
                Token::Semicolon => {
                    // 何もしない
                }
                _ => {
                    panic!("初期化子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }
            }
        } else {
            panic!("トークンがありません");
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

        if let Some(left_node) = self.logical_and_expression(&parent) {

            // 次のトークンが '||' の場合は logical_or_expression をパースする
            if let Some(Token::Operator(Operator::LogicalOr)) = self.get_next_token_without_increment() {
                println!("logical_or_expression");
                node.borrow_mut().set_val(Leaf::Token(Token::Operator(Operator::LogicalOr)));
                node.borrow_mut().set_lhs(left_node);
                self.token_index_increment();

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
                self.token_index_increment();
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
                self.token_index_increment();
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

    // 最終的にはpostfix_expression を呼び出すが関数呼び出しと配列は現状無視する.
    fn unary_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        // 次のトークンを取得
        let next_token = self.get_next_token_without_increment();

        if let Some(Token::UnaryOperator(operator)) = next_token {
            // 単項演算子の場合
            node.borrow_mut().set_val(Leaf::UnaryExpression(operator));
            self.token_index_increment();
           let left_node = self.postfix_expression(&mut node);
            if let Some(left_node) = left_node {
                node.borrow_mut().set_lhs(left_node);
            }
        } else {
            // 単項演算子でない場合は postfix_expression をパースする
            let postfix_node = self.postfix_expression(&mut node);
            if let Some(postfix_node) = postfix_node {
                node = postfix_node;
            }
        }

        Some(node)
    }

    fn postfix_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);


        if let Some(next_token) = self.get_next_token_without_increment()
        {
            match next_token
            {
                Token::Identifier(identify) => {
                    // index を進める
                    self.token_index_increment();

                    if let Some(next_identify) = self.get_next_token()
                    {
                        match next_identify
                        {
                            Token::LeftParen => {
                                // 関数呼び出しの場合
                                let mut function_call = FunctionCall::new(identify);

                                // ')' が来るまで argument_expression_list を呼び出す
                                loop {
                                    let next_token = self.get_next_token_without_increment();
                                    
                                    // 引数がない場合は ')' が来る
                                    if let Some(Token::RightParen) = next_token {
                                        self.token_index_increment();
                                        break;
                                    }

                                    let logical_or_expression_node
                                        = self.logical_or_expression(&node);

                                    if let Some(arg) = logical_or_expression_node {
                                        println!("arg : {:?}", arg);
                                        function_call.add_argument(arg);
                                    }

                                    // ',' が来ることを確認
                                    match self.get_next_token_without_increment()
                                    {
                                        Some(Token::Comma) => {
                                            self.token_index_increment();
                                        }
                                        None => {
                                            panic!("次のトークンがありません");
                                        }
                                        _ => {
                                            // 何もしない
                                            println!("skip : {:?}", self.tokens[self.token_index]);
                                        }
                                    }
                                }

                                node.borrow_mut().set_val(Leaf::FunctionCall(function_call));
                            }
                            Token::LeftBracket => {
                                // 配列の場合
                                node.borrow_mut().set_val(Leaf::ArrayAccess);
                                
                                // 左側に識別子を設定
                                let left_node = Rc::new(RefCell::new(Node::new()));
                                left_node.borrow_mut().set_val(Leaf::Token(Token::Identifier(identify)));
                                node.borrow_mut().set_lhs(left_node);

                                // ']' のときはからの配列として扱う
                                if let Some(Token::RightBracket) = self.get_next_token_without_increment() {
                                    // 何もしない
                                    self.token_index_increment();
                                } else {
                                    // 配列の index を取得
                                    let logical_or_expression_node = self.logical_or_expression(&node);
                                    if let Some(index) = logical_or_expression_node {
                                        node.borrow_mut().set_rhs(index);
                                    }
                                    
                                    // ']' が来ることを確認
                                    if let Some(Token::RightBracket) = self.get_next_token() {
                                        // 何もしない
                                    } else {
                                        panic!("']' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
                                    }
                                }
                            }
                            _ => {
                                // それ以外の場合は identifier として処理する
                                node.borrow_mut().set_val(Leaf::Token(Token::Identifier(identify)));
                            }
                        }

                        return Some(node);
                    }
                }
                // それ以外の場合
                _ => {
                    // primary_expression を呼び出す
                    let primary_node = self.primary_expression(&node);
                    if let Some(primary_node) = primary_node {
                        node = primary_node;
                    }
                }
            }

            return Some(node);
        }

        None
    }

    fn argument_expression_list(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        Some(node)
    }

    fn primary_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        // 次のトークンを取得
        if let Some(next_token) = self.get_next_token()
        {
            match next_token
            {
                Token::Constant(constant) => {
                    // 定数の場合
                    node.borrow_mut().set_val(Leaf::Token(Token::Constant(constant)));
                }
                Token::LeftParen => {
                    node.borrow_mut().set_val(Leaf::ParenthesizedExpression);

                    // '(' が来た場合は logical_or_expression を呼び出す
                    let logical_or_expression_node = self.logical_or_expression(&node);
                    if let Some(logical_or_expression_node) = logical_or_expression_node {
                        node.borrow_mut().set_lhs(logical_or_expression_node);
                    }

                    // ')' が来ることを確認
                    if let Some(Token::RightParen) = self.get_next_token() {
                        // 何もしない
                    } else {
                        panic!("')' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
                    }

                    return Some(node);
                }
                _ => {
                    // 何もしない
                    // semicolon はここで処理しない
                }
            }
            return Some(node);
        }

        None
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
        let program = String::from("int a = b();");

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
