use crate::lexical::Operator;
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};
use crate::lexical::{Constant, Token, ValueType, UnaryOperator};

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

    pub fn arguments(&self) -> &Vec<Rc<RefCell<Node>>> {
        &self.arguments
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
pub struct Argument {
    // 型
    type_specifier: ValueType,

    // 識別子
    identify: String,
}

impl Argument {
    pub fn new(type_specifier: ValueType, identify: String) -> Self {
        Argument {
            type_specifier,
            identify,
        }
    }

    pub fn type_specifier(&self) -> &ValueType {
        &self.type_specifier
    }

    pub fn identify(&self) -> &String {
        &self.identify
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    // 型
    type_specifier: ValueType,

    // 識別子
    identify: String,

    // 引数のリスト
    arguments: Vec<Argument>,

    // 関数の中身
    body: Vec<Rc<RefCell<Node>>>,
}

impl FunctionDefinition {
    pub fn new() -> Self {
        FunctionDefinition {
            type_specifier: ValueType::Void,
            identify: String::new(),
            arguments: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn body(&self) -> &Vec<Rc<RefCell<Node>>> {
        &self.body
    }

    pub fn name(&self) -> &String {
        &self.identify
    }

    pub fn arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }

    pub fn type_specifier(&self) -> &ValueType {
        &self.type_specifier
    }

    pub fn set_type_specifier(&mut self, type_specifier: ValueType) {
        self.type_specifier = type_specifier;
    }

    pub fn set_identify(&mut self, identify: String) {
        self.identify = identify;
    }

    pub fn add_argument(&mut self, type_specifier: ValueType, identify: String) {
        self.arguments.push(Argument::new(type_specifier, identify));
    }

    pub fn add_body(&mut self, body: Rc<RefCell<Node>>) {
        self.body.push(body);
    }
}

#[derive(Debug, Clone)]
pub struct ForStatement {
    // 初期化式
    initializer: Rc<RefCell<Node>>,

    // 条件式
    condition: Rc<RefCell<Node>>,

    // 更新式
    update: Rc<RefCell<Node>>,

    // for の中身
    statement: Rc<RefCell<Node>>,
}

impl ForStatement
{
    pub fn new(initializer: Rc<RefCell<Node>>, condition: Rc<RefCell<Node>>, update: Rc<RefCell<Node>>, statement: Rc<RefCell<Node>>) -> Self {
        ForStatement {
            initializer,
            condition,
            update,
            statement,
        }
    }

    pub fn initializer(&self) -> &Rc<RefCell<Node>> {
        &self.initializer
    }

    pub fn condition(&self) -> &Rc<RefCell<Node>> {
        &self.condition
    }

    pub fn update(&self) -> &Rc<RefCell<Node>> {
        &self.update
    }

    pub fn statement(&self) -> &Rc<RefCell<Node>> {
        &self.statement
    }
}

#[derive(Debug, Clone)]
pub enum Leaf
{
    Node(Rc<RefCell<Node>>),
    Declaration(ValueType),
    FunctionDefinition(FunctionDefinition),
    UnaryExpression(UnaryOperator),
    FunctionCall(FunctionCall),
    ArrayAccess,
    ParenthesizedExpression,
    Array(usize),

    // {, }
    BlockItem(Vec<Rc<RefCell<Node>>>),

    // 分岐
    IfStatement(Rc<RefCell<Node>>),

    // ループ
    WhileStatement,
    ForStatement(ForStatement),

    // 代入
    Assignment,
    ArrayAssignment(Rc<RefCell<Node>>),

    // jump系の文
    Return,
    Break,
    Continue,

    // 識別子
    Identifier(String),

    // 演算子
    Operator(Operator),

    // 定数
    Constant(Constant),
}

// Leaf の format 出力
impl std::fmt::Display for Leaf
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Leaf::Node(node) => write!(f, "Node [{:?}]", node.borrow().val()),
            Leaf::Declaration(declaration) => write!(f, "Declaration [{:?}]", declaration),
            Leaf::FunctionDefinition(function_definition) => write!(f, "FunctionDefinition [{:?}]", function_definition.name()),
            Leaf::UnaryExpression(operator) => write!(f, "UnaryExpression [{:?}]", operator),
            Leaf::FunctionCall(function_call) => write!(f, "FunctionCall [{:?}]", function_call),
            Leaf::ArrayAccess => write!(f, "ArrayAccess"),
            Leaf::ParenthesizedExpression => write!(f, "ParenthesizedExpression"),
            Leaf::BlockItem(_) => write!(f, "BlockItem"),
            Leaf::IfStatement(_) => write!(f, "IfStatement"),
            Leaf::Assignment => write!(f, "Assignment"),
            Leaf::Return => write!(f, "Return"),
            Leaf::Break => write!(f, "Break"),
            Leaf::Continue => write!(f, "Continue"),
            Leaf::Identifier(identifier) => write!(f, "Identifier [{:?}]", identifier),
            Leaf::Operator(operator) => write!(f, "Operator [{:?}]", operator),
            Leaf::Constant(constant) => write!(f, "Constant [{:?}]", constant),
            Leaf::WhileStatement => write!(f, "WhileStatement"),
            Leaf::ForStatement(_) => write!(f, "ForStatement"),
            Leaf::Array(size) => write!(f, "Array [{:?}]", size),
            Leaf::ArrayAssignment(size) => write!(f, "ArrayAssignment"),
        }
    }
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

    pub fn get_lhs_and_rhs(&self)
                           -> Option<(&Rc<RefCell<Node>>, &Rc<RefCell<Node>>)>
    {
        self.lhs().zip(self.rhs())
    }

    pub fn show_node(root: &Node)
    {
        if let Some(leaf) = &root.val {
            match leaf {
                Leaf::Declaration(declaration) => {
                    println!("Declaration [{:?}]", declaration);
                }
                Leaf::FunctionDefinition(function_definition) => {
                    println!("FunctionDefinition [{:?}]", function_definition.name());
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
                Leaf::Identifier(identifier) => {
                    println!("Identifier [{:?}]", identifier);
                }
                Leaf::Operator(operator) => {
                    println!("Operator [{:?}]", operator);
                }
                Leaf::Constant(constant) => {
                    println!("Constant [{:?}]", constant);
                }
                Leaf::Return => {
                    println!("Return");
                }
                Leaf::Break => {
                    println!("Break");
                }
                Leaf::Continue => {
                    println!("Continue");
                }
                Leaf::Assignment => {
                    println!("Assignment");
                }
                Leaf::IfStatement(_) => {
                    println!("IfStatement");
                }
                Leaf::BlockItem(_) => {
                    println!("BlockItem");
                }
                Leaf::WhileStatement => {
                    println!("WhileStatement");
                }
                Leaf::ForStatement(_) => {
                    println!("ForStatement");
                }
                Leaf::Array(size) => {
                    println!("Array [{:?}]", size);
                }
                Leaf::ArrayAssignment(size) => {
                    println!("ArrayAssignment");
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

    pub fn roots(&self) -> &Vec<Rc<RefCell<Node>>> {
        &self.roots
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

        let token = self.tokens[self.token_index].clone();
        match token {
            Token::Type(_) => {
                let next_token = self.tokens[self.token_index + 2].clone();

                if next_token == Token::LeftParen {
                    println!("function_definition");
                    self.function_definition();
                } else {
                    println!("declaration : {}", self.token_index);
                    let root = self.declaration();
                    self.roots.push(root);
                }
            }
            _ => {
                let root = self.statement();
                self.roots.push(root);
            }
        }
    }

    fn function_definition(&mut self)
    {
        let mut function_definition = FunctionDefinition::new();

        // 関数定義の型を取得
        if let Some(Token::Type(type_specifier)) = self.get_next_token() {
            function_definition.set_type_specifier(type_specifier);
        } else {
            panic!("型が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 関数定義の識別子を取得
        if let Some(Token::Identifier(identifier)) = self.get_next_token() {
            function_definition.set_identify(identifier);
        } else {
            panic!("識別子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 関数定義の引数リストを取得
        if let Some(Token::LeftParen) = self.get_next_token() {
            // 引数がない場合は ')' が来る
            if let Some(Token::RightParen) = self.get_next_token_without_increment() {
                self.token_index_increment();
            } else {
                self.parameter_list(&mut function_definition);
                // ')' が来ることを確認
                if let Some(Token::RightParen) = self.get_next_token() {
                    // 何もしない
                } else {
                    panic!("')' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }
            }
        }

        // 関数定義の本体を取得. '{', '}' の処理は compound_statement 内部で行う
        let roots = self.compound_statement();
        function_definition.body = roots;

        let root = Rc::new(RefCell::new(Node::new()));
        root.borrow_mut().set_val(Leaf::FunctionDefinition(function_definition));

        self.roots.push(root);
    }

    fn compound_statement(&mut self) -> Vec<Rc<RefCell<Node>>>
    {
        let mut roots: Vec<Rc<RefCell<Node>>> = Vec::new();
        println!("compound_statement");
        // '{' が来ることを確認
        if let Some(Token::LeftBrace) = self.get_next_token() {
            // 何もしない
        } else {
            panic!("'{{' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // '}' が来るまで繰り返す
        loop {
            if let Some(Token::RightBrace) = self.get_next_token_without_increment() {
                break;
            }

            let root = self.block_item();
            roots.push(root);
        }

        // '}' が来ることを確認
        if let Some(Token::RightBrace) = self.get_next_token() {
            // 何もしない
        } else {
            panic!("'}}' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        roots
    }

    fn block_item(&mut self) -> Rc<RefCell<Node>>
    {
        let mut root = Rc::new(RefCell::new(Node::new()));

        if let Some(next_token) = self.get_next_token_without_increment()
        {
            match next_token
            {
                // 変数定義の場合
                Token::Type(type_specifier) => {
                    root = self.declaration();
                }
                _ => {
                    root = self.statement();
                }
            }
        }

        root
    }

    fn statement(&mut self) -> Rc<RefCell<Node>>
    {
        let mut root = Rc::new(RefCell::new(Node::new()));

        if let Some(next_token) = self.get_next_token_without_increment()
        {
            match next_token
            {
                Token::LeftBrace => {
                    // compound_statement の場合
                    let roots = self.compound_statement();
                    root.borrow_mut().set_val(Leaf::BlockItem(roots));
                }
                Token::If => {
                    // if_statement の場合
                    root = self.selection_statement();
                }
                Token::While => {
                    // while_statement の場合
                    root = self.iteration_statement();
                }
                Token::Return => {
                    // jump_statement の場合
                    root = self.jump_statement();
                }
                Token::Identifier(identifier) => {
                    // expression_statement の場合
                    root = self.expression_statement();

                    // ';' が来ることを確認
                    self.semicolon();
                }
                Token::Break => {
                    // jump_statement の場合
                    root = self.jump_statement();
                }
                Token::For => {
                    // iteration_statement の場合
                    root = self.iteration_statement();
                }
                _ => {
                    // expression_statement の場合
                    unimplemented!("expression_statement");
                }
            }
        }

        root
    }

    fn iteration_statement(&mut self) -> Rc<RefCell<Node>>
    {
        let mut root = Rc::new(RefCell::new(Node::new()));

        // 最初の トークンを取得して while_statement かどうかを判定
        if let Some(token) = self.get_next_token_without_increment()
        {
            match token
            {
                Token::While => {
                    root = self.while_statement();
                }
                Token::For => {
                    root = self.for_statement();
                }
                _ => {
                    panic!("while_statement が見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }
            }
        }

        root
    }

    fn for_statement(&mut self) -> Rc<RefCell<Node>>
    {
        let mut root = Rc::new(RefCell::new(Node::new()));

        // 最初の for トークンを取得
        if let Some(Token::For) = self.get_next_token()
        {
            // 何もしない
        } else {
            panic!("'for' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 次のトークンが '(' かどうか
        if let Some(Token::LeftParen) = self.get_next_token()
        {
            // 何もしない
        } else {
            panic!("'(' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 初期化式を取得.
        let initializer = self.expression_statement();
        self.semicolon();


        // 条件式を取得
        let condition = self.logical_or_expression(&root).unwrap();
        self.semicolon();


        // 更新式を取得
        let update = self.expression_statement();

        // 次のトークンが ')' かどうか
        if let Some(Token::RightParen) = self.get_next_token()
        {
            // 何もしない
        } else {
            panic!("')' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // for の中身を取得
        let statement = self.statement();

        // ForStatement を作成
        let for_statement = ForStatement::new(initializer, condition, update, statement);
        root.borrow_mut().set_val(Leaf::ForStatement(for_statement));

        root
    }

    fn while_statement(&mut self) -> Rc<RefCell<Node>>
    {
        let mut root = Rc::new(RefCell::new(Node::new()));
        root.borrow_mut().set_val(Leaf::WhileStatement);

        // 最初の while トークンを取得
        if let Some(Token::While) = self.get_next_token()
        {
            // 何もしない
        } else {
            panic!("'while' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 次のトークンが '(' かどうか
        if let Some(Token::LeftParen) = self.get_next_token()
        {
            // 何もしない
        } else {
            panic!("'(' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 条件式を取得
        if let Some(condition) = self.logical_or_expression(&root)
        {
            root.borrow_mut().set_lhs(condition);
        } else {
            panic!("条件式が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 次のトークンが ')' かどうか
        if let Some(Token::RightParen) = self.get_next_token()
        {
            // 何もしない
        } else {
            panic!("')' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // while の中身を取得
        let statement = self.statement();
        root.borrow_mut().set_rhs(statement);

        root
    }

    fn selection_statement(&mut self) -> Rc<RefCell<Node>>
    {
        let mut root = Rc::new(RefCell::new(Node::new()));

        // 最初の if トークンを取得
        if let Some(Token::If) = self.get_next_token()
        {
            // 何もしない
        } else {
            panic!("'if' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 次のトークンが '(' かどうか
        if let Some(Token::LeftParen) = self.get_next_token()
        {
            // 何もしない
        } else {
            panic!("'(' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 条件式を取得
        if let Some(condition) = self.logical_or_expression(&root)
        {
            root.borrow_mut().set_val(Leaf::IfStatement(condition));
        } else {
            panic!("条件式が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // 次のトークンが ')' かどうか
        if let Some(Token::RightParen) = self.get_next_token()
        {
            // 何もしない
        } else {
            panic!("')' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // if の中身を取得
        let true_statement = self.statement();
        root.borrow_mut().set_lhs(true_statement);

        // else がある場合
        if let Some(Token::Else) = self.get_next_token_without_increment()
        {
            self.token_index_increment();

            // else の中身を取得
            let false_statement = self.statement();
            root.borrow_mut().set_rhs(false_statement);
        }

        root
    }

    fn semicolon(&mut self) {
        if let Some(Token::Semicolon) = self.get_next_token() {
            // 何もしない
        } else {
            panic!("';' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }
    }
    fn expression_statement(&mut self) -> Rc<RefCell<Node>>
    {
        let mut root = Rc::new(RefCell::new(Node::new()));

        let identifier = self.tokens[self.token_index].clone();
        let next_token = self.tokens[self.token_index + 1].clone();

        // next_token が '=' なら assignment として処理する
        match next_token
        {
            Token::Assign => {
                root = self.assignment();
            }
            Token::LeftBracket => {
                root = self.array_assignment();
            }
            _ => {
                // それ以外は expression として処理する
                root = self.logical_or_expression(&root).unwrap();
            }
        }

        root
    }

    fn assignment(&mut self) -> Rc<RefCell<Node>>
    {
        let mut root = Rc::new(RefCell::new(Node::new()));
        root.borrow_mut().set_val(Leaf::Assignment);

        // 左辺に識別子を設定
        if let Some(Token::Identifier(identifier)) = self.get_next_token()
        {
            let left_node = Rc::new(RefCell::new(Node::new()));
            left_node.borrow_mut().set_val(Leaf::Identifier(identifier));
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
                }
                _ => {
                    panic!("初期化子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }
            }
        } else {
            panic!("トークンがありません");
        }

        root
    }

    fn array_assignment(&mut self) -> Rc<RefCell<Node>>
    {
        let mut root = Rc::new(RefCell::new(Node::new()));

        // identifier を取得
        if let Some(Token::Identifier(identifier)) = self.get_next_token()
        {
            let left_node = Rc::new(RefCell::new(Node::new()));
            left_node.borrow_mut().set_val(Leaf::Identifier(identifier));
            root.borrow_mut().set_lhs(left_node);
        } else {
            panic!("識別子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // '[' が来ることを確認
        if let Some(Token::LeftBracket) = self.get_next_token()
        {
            // 何もしない
        } else {
            panic!("'[' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // アクセスするindex (添字) を取得
        if let Some(index) = self.logical_or_expression(&root)
        {
            root.borrow_mut().set_val(Leaf::ArrayAssignment(index));
        } else {
            panic!("添字が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // ']' が来ることを確認
        if let Some(Token::RightBracket) = self.get_next_token()
        {
            // 何もしない
        } else {
            panic!("']' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
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
                }
                _ => {
                    panic!("初期化子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }
            }
        } else {
            panic!("トークンがありません");
        }


        root
    }

    fn jump_statement(&mut self) -> Rc<RefCell<Node>>
    {
        let mut root = Rc::new(RefCell::new(Node::new()));

        // 次のトークンを取得
        if let Some(jump_token) = self.get_next_token()
        {
            match jump_token
            {
                Token::Return => {
                    println!("return");
                    // valにReturnを設定
                    root.borrow_mut().set_val(Leaf::Return);

                    // return の場合は expression が続く
                    if let Some(expression) = self.logical_or_expression(&root) {
                        root.borrow_mut().set_lhs(expression);
                    } else {
                        panic!("return の後に式がありませんでした : {:?}", self.tokens[self.token_index]);
                    }
                }
                Token::Break => {
                    println!("break");
                    root.borrow_mut().set_val(Leaf::Break);
                }
                Token::Continue => {
                    unimplemented!("continue");
                }
                _ => {
                    panic!("ジャンプステートメントが見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }
            }
        }

        // ';' が来ることを確認
        self.semicolon();

        root
    }

    /// 関数の引数リストを取得する. ')' が来るまで繰り返す
    fn parameter_list(&mut self, function_definition: &mut FunctionDefinition)
    {
        // ')' が来る場合は何もしない
        if let Some(Token::RightParen) = self.get_next_token_without_increment() {
            return;
        }

        // 一個目の型が void の場合は何もせずに終了
        if let Some(Token::Type(type_specifier)) = self.get_next_token_without_increment()
        {
            if type_specifier == ValueType::Void {
                self.token_index_increment();
                return;
            }
        }

        loop {
            if let Some(Token::Type(type_specifier)) = self.get_next_token_without_increment()
            {
                self.token_index_increment();

                // 型がある場合は識別子が続く
                if let Some(Token::Identifier(identifier)) = self.get_next_token()
                {
                    function_definition.add_argument(type_specifier, identifier);
                } else {
                    panic!("関数の引数の識別子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }
            } else {
                panic!("関数の型が見つかりませんでした : {:?}", self.tokens[self.token_index]);
            }

            // 次のトークンが ',' か ')' かを調べて ',' なら次の引数を取得する
            match self.get_next_token_without_increment()
            {
                Some(Token::Comma) => {
                    self.token_index_increment();
                }
                Some(Token::RightParen) => {
                    break;
                }
                _ => {
                    panic!("次のトークンがありません : {:?}", self.tokens[self.token_index]);
                }
            }
        }
    }

    fn declaration(&mut self) -> Rc<RefCell<Node>>
    {
        println!("declaration");

        // グローバル変数定義をパースする
        let mut root = Rc::new(RefCell::new(Node::new()));

        // declaration の値として型が入る
        if let Some(Token::Type(type_specifier)) = self.get_next_token() {
            root.borrow_mut().set_val(Leaf::Declaration(type_specifier));
        } else {
            panic!("型が見つかりませんでした : {:?}", self.tokens[self.token_index]);
        }

        // declaration の左辺として識別子が入る
        if let Some(Token::Identifier(identifier)) = self.get_next_token() {
            let left_node = Rc::new(RefCell::new(Node::new()));
            left_node.borrow_mut().set_val(Leaf::Identifier(identifier));
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
                Token::LeftBracket => {
                    println!("array");
                    // 配列の場合
                    if let Some(Token::Constant(Constant::Integer(size))) = self.get_next_token() {
                        let right_node = Rc::new(RefCell::new(Node::new()));
                        right_node.borrow_mut().set_val(Leaf::Array(size as usize));
                        root.borrow_mut().set_rhs(right_node);
                    } else {
                        panic!("配列のサイズが見つかりませんでした : {:?}", self.tokens[self.token_index]);
                    }

                    // 正しく配列のサイズが取得できた場合
                    if let Some(Token::RightBracket) = self.get_next_token() {
                        // 何もしない
                    } else {
                        panic!("']' が見つかりませんでした : {:?}", self.tokens[self.token_index]);
                    }

                    self.semicolon();
                }
                _ => {
                    panic!("初期化子が見つかりませんでした : {:?}", self.tokens[self.token_index]);
                }
            }
        } else {
            panic!("トークンがありません");
        }
        root
    }


    fn logical_or_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        println!("logical_or_expression");
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(left_node) = self.logical_and_expression(&parent) {

            // 次のトークンが '||' の場合は logical_or_expression をパースする
            if let Some(Token::Operator(Operator::LogicalOr)) = self.get_next_token_without_increment() {
                node.borrow_mut().set_val(Leaf::Operator(Operator::LogicalOr));
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
        println!("logical_and_expression");
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(left_node) = self.equality_expression(&node)
        {
            // 次のトークンが '&&' の場合は logical_and_expression をパースする
            if let Some(Token::Operator(Operator::LogicalAnd)) = self.get_next_token_without_increment() {
                node.borrow_mut().set_val(Leaf::Operator(Operator::LogicalAnd));
                node.borrow_mut().set_lhs(left_node);
                self.token_index_increment();

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
        println!("equality_expression");
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
                node.borrow_mut().set_val(Leaf::Operator(operator));
                node.borrow_mut().set_lhs(left_node);
                self.token_index_increment();

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
        println!("relational_expression");
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
                node.borrow_mut().set_val(Leaf::Operator(operator));
                node.borrow_mut().set_lhs(left_node);
                self.token_index_increment();

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
        println!("additive_expression");
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
                node.borrow_mut().set_val(Leaf::Operator(operator));
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
        println!("multiplicative_expression");
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(left_node) = self.unary_expression(&mut node) {
            // '*', '/', '%' の演算子を取得
            if let Some(operator) = self.get_next_token_without_increment()
                .and_then(|token| match token {
                    Token::Operator(Operator::Multiply) => Some(Operator::Multiply),
                    Token::Operator(Operator::Divide) => Some(Operator::Divide),
                    Token::Operator(Operator::Modulo) => Some(Operator::Modulo),
                    _ => None,
                })
            {
                self.token_index_increment();
                node.borrow_mut().set_val(Leaf::Operator(operator));
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
        println!("unary_expression");
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
        println!("postfix_expression");
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        if let Some(next_token) = self.get_next_token_without_increment()
        {
            match next_token
            {
                Token::Identifier(identify) => {
                    // index を進める
                    self.token_index_increment();

                    if let Some(next_identify) = self.get_next_token_without_increment()
                    {
                        match next_identify
                        {
                            Token::LeftParen => {
                                self.token_index_increment();

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
                                self.token_index_increment();
                                // 配列の場合
                                node.borrow_mut().set_val(Leaf::ArrayAccess);

                                // 左側に識別子を設定
                                let left_node = Rc::new(RefCell::new(Node::new()));
                                left_node.borrow_mut().set_val(Leaf::Identifier(identify));
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
                                println!("postfix_expression : {:?}", identify);
                                // それ以外の場合は identifier として処理する
                                node.borrow_mut().set_val(Leaf::Identifier(identify));
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
        println!("argument_expression_list");
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        Some(node)
    }

    fn primary_expression(&mut self, parent: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>>
    {
        println!("primary_expression");
        let mut node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_parent(parent);

        // 次のトークンを取得
        if let Some(next_token) = self.get_next_token()
        {
            match next_token
            {
                Token::Identifier(identifier) => {
                    node.borrow_mut().set_val(Leaf::Identifier(identifier));
                }
                Token::Constant(constant) => {
                    // 定数の場合
                    node.borrow_mut().set_val(Leaf::Constant(constant));
                }
                Token::LeftParen => {
                    node.borrow_mut().set_val(Leaf::ParenthesizedExpression);

                    // '(' が来た場合は logical_or_expression を呼び出す
                    let logical_or_expression_node = self.logical_or_expression(&node);
                    if let Some(logical_or_expression_node) = logical_or_expression_node {
                        node.borrow_mut().set_lhs(logical_or_expression_node);
                    } else {
                        panic!("空の括弧が見つかりました : {:?}", self.tokens[self.token_index]);
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
                    panic!("primary_expression でエラーが発生しました : {:?}", self.tokens[self.token_index]);
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
    use crate::lexical::{Token, ValueType};
    use crate::tree_viewer::TreeViewer;

    #[test]
    fn test_empty_function() {
        // int main() {}
        let tokens = vec![
            Token::Type(ValueType::Int),          // int
            Token::Identifier("main".to_string()), // main
            Token::LeftParen,                     // (
            Token::RightParen,                    // )
            Token::LeftBrace,                     // {
            Token::RightBrace,                    // }
        ];
        let answer = String::from(
            "digraph {\n    0 [ label = \"\\\"0: Function Definition [main]\\\"\" ]\n}\n");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");


        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_global_var_and_return() {
        // int globalVar = 10; 
        // int main() 
        // { 
        //      return globalVar; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("globalVar".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(10)),
            Token::Semicolon,
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Identifier("globalVar".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];
        let answer = String::from("digraph{0[label=\"\\\"0:Declaration(Int)\\\"\"]1[label=\"\\\"1:Identifier(\\\\\\\"globalVar\\\\\\\")\\\"\"]2[label=\"\\\"2:Constant(Integer(10))\\\"\"]3[label=\"\\\"3:FunctionDefinition[main]\\\"\"]4[label=\"\\\"4:Return\\\"\"]5[label=\"\\\"5:Identifier(\\\\\\\"globalVar\\\\\\\")\\\"\"]0->1[]0->2[]4->5[]3->4[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }


        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_function_definition_with_two_args() {
        // int add(int a, int b) 
        // { 
        //     return a + b; 
        // } 
        // int main() 
        // {
        //     int result = add(1, 2);
        //     return result; 
        // }
        let tokens = vec![
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
            Token::Return,
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::Plus),
            Token::Identifier("b".to_string()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("result".to_string()),
            Token::Assign,
            Token::Identifier("add".to_string()),
            Token::LeftParen,
            Token::Constant(Constant::Integer(1)),
            Token::Comma,
            Token::Constant(Constant::Integer(2)),
            Token::RightParen,
            Token::Semicolon,
            Token::Return,
            Token::Identifier("result".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[add]\\\"\"]1[label=\"\\\"1:Return\\\"\"]2[label=\"\\\"2:Operator(Plus)\\\"\"]3[label=\"\\\"3:Identifier(\\\\\\\"a\\\\\\\")\\\"\"]4[label=\"\\\"4:Identifier(\\\\\\\"b\\\\\\\")\\\"\"]5[label=\"\\\"5:FunctionDefinition[main]\\\"\"]6[label=\"\\\"6:Declaration(Int)\\\"\"]7[label=\"\\\"7:Identifier(\\\\\\\"result\\\\\\\")\\\"\"]8[label=\"\\\"8:FunctionCall[add]\\\"\"]9[label=\"\\\"9:Constant(Integer(1))\\\"\"]10[label=\"\\\"10:Constant(Integer(2))\\\"\"]11[label=\"\\\"11:Return\\\"\"]12[label=\"\\\"12:Identifier(\\\\\\\"result\\\\\\\")\\\"\"]2->3[]2->4[]1->2[]0->1[]6->7[]8->9[]8->10[]6->8[]5->6[]11->12[]5->11[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_if_statement_block() {
        // int main() 
        // { 
        //      int x = 0; 
        //      if (x == 0) 
        //      { 
        //          x = 1; 
        //      } 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,
            Token::If,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Equal),
            Token::Constant(Constant::Integer(0)),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(1)),
            Token::Semicolon,
            Token::RightBrace,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]3[label=\"\\\"3:Constant(Integer(0))\\\"\"]4[label=\"\\\"4:IfStatement\\\"\"]5[label=\"\\\"5:BlockItem\\\"\"]6[label=\"\\\"6:Assignment\\\"\"]7[label=\"\\\"7:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]8[label=\"\\\"8:Constant(Integer(1))\\\"\"]1->2[]1->3[]0->1[]6->7[]6->8[]5->6[]4->5[]0->4[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_if_else_statement() {
        // int main() 
        // { 
        //      int x = 0; 
        //      if (x == 0) 
        //      { 
        //          x = 1; 
        //      } 
        //      else 
        //      { 
        //          x = 2; 
        //      } 
        //      return x; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,
            Token::If,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Equal),
            Token::Constant(Constant::Integer(0)),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(1)),
            Token::Semicolon,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(2)),
            Token::Semicolon,
            Token::RightBrace,
            Token::Return,
            Token::Identifier("x".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]3[label=\"\\\"3:Constant(Integer(0))\\\"\"]4[label=\"\\\"4:IfStatement\\\"\"]5[label=\"\\\"5:BlockItem\\\"\"]6[label=\"\\\"6:Assignment\\\"\"]7[label=\"\\\"7:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]8[label=\"\\\"8:Constant(Integer(1))\\\"\"]9[label=\"\\\"9:BlockItem\\\"\"]10[label=\"\\\"10:Assignment\\\"\"]11[label=\"\\\"11:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]12[label=\"\\\"12:Constant(Integer(2))\\\"\"]13[label=\"\\\"13:Return\\\"\"]14[label=\"\\\"14:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]1->2[]1->3[]0->1[]6->7[]6->8[]5->6[]4->5[]10->11[]10->12[]9->10[]4->9[]0->4[]13->14[]0->13[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");
        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_for_loop_increment_and_update() {
        // int main() 
        // { 
        //      int i = 0; 
        //      for (i = 0; i < 10; i = i + 1) 
        //      { 
        //          i = i * 2; 
        //      } 
        //      return i; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,
            Token::For,
            Token::LeftParen,
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,
            Token::Identifier("i".to_string()),
            Token::Operator(Operator::LessThan),
            Token::Constant(Constant::Integer(10)),
            Token::Semicolon,
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::Identifier("i".to_string()),
            Token::Operator(Operator::Plus),
            Token::Constant(Constant::Integer(1)),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::Identifier("i".to_string()),
            Token::Operator(Operator::Multiply),
            Token::Constant(Constant::Integer(2)),
            Token::Semicolon,
            Token::RightBrace,
            Token::Return,
            Token::Identifier("i".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];
        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]3[label=\"\\\"3:Constant(Integer(0))\\\"\"]4[label=\"\\\"4:ForStatement\\\"\"]5[label=\"\\\"5:Assignment\\\"\"]6[label=\"\\\"6:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]7[label=\"\\\"7:Constant(Integer(0))\\\"\"]8[label=\"\\\"8:Operator(LessThan)\\\"\"]9[label=\"\\\"9:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]10[label=\"\\\"10:Constant(Integer(10))\\\"\"]11[label=\"\\\"11:Assignment\\\"\"]12[label=\"\\\"12:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]13[label=\"\\\"13:Operator(Plus)\\\"\"]14[label=\"\\\"14:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]15[label=\"\\\"15:Constant(Integer(1))\\\"\"]16[label=\"\\\"16:BlockItem\\\"\"]17[label=\"\\\"17:Assignment\\\"\"]18[label=\"\\\"18:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]19[label=\"\\\"19:Operator(Multiply)\\\"\"]20[label=\"\\\"20:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]21[label=\"\\\"21:Constant(Integer(2))\\\"\"]22[label=\"\\\"22:Return\\\"\"]23[label=\"\\\"23:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]1->2[]1->3[]0->1[]5->6[]5->7[]4->5[]8->9[]8->10[]4->8[]11->12[]13->14[]13->15[]11->13[]4->11[]17->18[]19->20[]19->21[]17->19[]16->17[]4->16[]0->4[]22->23[]0->22[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_while_loop_basic() {
        // int main() 
        // { 
        //      int i = 0; 
        //      while (i < 5) 
        //      { 
        //          i = i + 1; 
        //      } 
        //      return i; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,
            Token::While,
            Token::LeftParen,
            Token::Identifier("i".to_string()),
            Token::Operator(Operator::LessThan),
            Token::Constant(Constant::Integer(5)),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::Identifier("i".to_string()),
            Token::Operator(Operator::Plus),
            Token::Constant(Constant::Integer(1)),
            Token::Semicolon,
            Token::RightBrace,
            Token::Return,
            Token::Identifier("i".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];
        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]3[label=\"\\\"3:Constant(Integer(0))\\\"\"]4[label=\"\\\"4:WhileStatement\\\"\"]5[label=\"\\\"5:Operator(LessThan)\\\"\"]6[label=\"\\\"6:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]7[label=\"\\\"7:Constant(Integer(5))\\\"\"]8[label=\"\\\"8:BlockItem\\\"\"]9[label=\"\\\"9:Assignment\\\"\"]10[label=\"\\\"10:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]11[label=\"\\\"11:Operator(Plus)\\\"\"]12[label=\"\\\"12:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]13[label=\"\\\"13:Constant(Integer(1))\\\"\"]14[label=\"\\\"14:Return\\\"\"]15[label=\"\\\"15:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]1->2[]1->3[]0->1[]5->6[]5->7[]4->5[]9->10[]11->12[]11->13[]9->11[]8->9[]4->8[]0->4[]14->15[]0->14[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_while_loop_with_break() {
        // int main() 
        // { 
        //      int i = 0; 
        //      while (i < 5) 
        //      { 
        //          i = i + 1; 
        //          if (i == 3) 
        //          { 
        //              break; 
        //          } 
        //      } 
        //      return i; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,
            Token::While,
            Token::LeftParen,
            Token::Identifier("i".to_string()),
            Token::Operator(Operator::LessThan),
            Token::Constant(Constant::Integer(10)),
            Token::RightParen,
            Token::LeftBrace,
            Token::If,
            Token::LeftParen,
            Token::Identifier("i".to_string()),
            Token::Operator(Operator::Equal),
            Token::Constant(Constant::Integer(5)),
            Token::RightParen,
            Token::LeftBrace,
            Token::Break,
            Token::Semicolon,
            Token::RightBrace,
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::Identifier("i".to_string()),
            Token::Operator(Operator::Plus),
            Token::Constant(Constant::Integer(1)),
            Token::Semicolon,
            Token::RightBrace,
            Token::Return,
            Token::Identifier("i".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]3[label=\"\\\"3:Constant(Integer(0))\\\"\"]4[label=\"\\\"4:WhileStatement\\\"\"]5[label=\"\\\"5:Operator(LessThan)\\\"\"]6[label=\"\\\"6:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]7[label=\"\\\"7:Constant(Integer(10))\\\"\"]8[label=\"\\\"8:BlockItem\\\"\"]9[label=\"\\\"9:IfStatement\\\"\"]10[label=\"\\\"10:BlockItem\\\"\"]11[label=\"\\\"11:Break\\\"\"]12[label=\"\\\"12:Assignment\\\"\"]13[label=\"\\\"13:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]14[label=\"\\\"14:Operator(Plus)\\\"\"]15[label=\"\\\"15:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]16[label=\"\\\"16:Constant(Integer(1))\\\"\"]17[label=\"\\\"17:Return\\\"\"]18[label=\"\\\"18:Identifier(\\\\\\\"i\\\\\\\")\\\"\"]1->2[]1->3[]0->1[]5->6[]5->7[]4->5[]10->11[]9->10[]8->9[]12->13[]14->15[]14->16[]12->14[]8->12[]4->8[]0->4[]17->18[]0->17[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_array_global_definition() {
        // int arr[10]; 
        // int main() 
        // { 
        //      arr[0] = 123;
        //      return arr[0]; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("arr".to_string()),
            Token::LeftBracket,
            Token::Constant(Constant::Integer(10)),
            Token::RightBracket,
            Token::Semicolon,
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("arr".to_string()),
            Token::LeftBracket,
            Token::Constant(Constant::Integer(0)),
            Token::RightBracket,
            Token::Assign,
            Token::Constant(Constant::Integer(123)),
            Token::Semicolon,
            Token::Return,
            Token::Identifier("arr".to_string()),
            Token::LeftBracket,
            Token::Constant(Constant::Integer(0)),
            Token::RightBracket,
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:Declaration(Int)\\\"\"]1[label=\"\\\"1:Identifier(\\\\\\\"arr\\\\\\\")\\\"\"]2[label=\"\\\"2:Array(10)\\\"\"]3[label=\"\\\"3:FunctionDefinition[main]\\\"\"]4[label=\"\\\"4:ArrayAssignment\\\"\"]5[label=\"\\\"5:Identifier(\\\\\\\"arr\\\\\\\")\\\"\"]6[label=\"\\\"6:Constant(Integer(123))\\\"\"]7[label=\"\\\"7:Return\\\"\"]8[label=\"\\\"8:ArrayAccess\\\"\"]9[label=\"\\\"9:Identifier(\\\\\\\"arr\\\\\\\")\\\"\"]10[label=\"\\\"10:Constant(Integer(0))\\\"\"]0->1[]0->2[]4->5[]4->6[]3->4[]8->9[]8->10[]7->8[]3->7[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_array_local_definition() {
        // int main() 
        // { 
        //      int arr[3]; 
        //      arr[2] = 5; 
        //      return arr[2]; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("arr".to_string()),
            Token::LeftBracket,
            Token::Constant(Constant::Integer(3)),
            Token::RightBracket,
            Token::Semicolon,
            Token::Identifier("arr".to_string()),
            Token::LeftBracket,
            Token::Constant(Constant::Integer(2)),
            Token::RightBracket,
            Token::Assign,
            Token::Constant(Constant::Integer(5)),
            Token::Semicolon,
            Token::Return,
            Token::Identifier("arr".to_string()),
            Token::LeftBracket,
            Token::Constant(Constant::Integer(2)),
            Token::RightBracket,
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"arr\\\\\\\")\\\"\"]3[label=\"\\\"3:Array(3)\\\"\"]4[label=\"\\\"4:ArrayAssignment\\\"\"]5[label=\"\\\"5:Identifier(\\\\\\\"arr\\\\\\\")\\\"\"]6[label=\"\\\"6:Constant(Integer(5))\\\"\"]7[label=\"\\\"7:Return\\\"\"]8[label=\"\\\"8:ArrayAccess\\\"\"]9[label=\"\\\"9:Identifier(\\\\\\\"arr\\\\\\\")\\\"\"]10[label=\"\\\"10:Constant(Integer(2))\\\"\"]1->2[]1->3[]0->1[]4->5[]4->6[]0->4[]8->9[]8->10[]7->8[]0->7[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_function_call_multiple_args() {
        // int foo(int x, int y) 
        // { 
        //      return x + y; 
        // } 
        // int main() 
        // { 
        //      return foo(10, 20); 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("foo".to_string()),
            Token::LeftParen,
            Token::Type(ValueType::Int),
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Type(ValueType::Int),
            Token::Identifier("y".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Plus),
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Identifier("foo".to_string()),
            Token::LeftParen,
            Token::Constant(Constant::Integer(10)),
            Token::Comma,
            Token::Constant(Constant::Integer(20)),
            Token::RightParen,
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[foo]\\\"\"]1[label=\"\\\"1:Return\\\"\"]2[label=\"\\\"2:Operator(Plus)\\\"\"]3[label=\"\\\"3:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]4[label=\"\\\"4:Identifier(\\\\\\\"y\\\\\\\")\\\"\"]5[label=\"\\\"5:FunctionDefinition[main]\\\"\"]6[label=\"\\\"6:Return\\\"\"]7[label=\"\\\"7:FunctionCall[foo]\\\"\"]8[label=\"\\\"8:Constant(Integer(10))\\\"\"]9[label=\"\\\"9:Constant(Integer(20))\\\"\"]2->3[]2->4[]1->2[]0->1[]7->8[]7->9[]6->7[]5->6[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_unary_operator_minus() {
        // int main() 
        // { 
        //      int x = 5; 
        //      x = -x; 
        //      return x; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(5)),
            Token::Semicolon,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::UnaryOperator(UnaryOperator::Minus),
            Token::Identifier("x".to_string()),
            Token::Semicolon,
            Token::Return,
            Token::Identifier("x".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]3[label=\"\\\"3:Constant(Integer(5))\\\"\"]4[label=\"\\\"4:Assignment\\\"\"]5[label=\"\\\"5:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]6[label=\"\\\"6:UnaryExpression(Minus)\\\"\"]7[label=\"\\\"7:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]8[label=\"\\\"8:Return\\\"\"]9[label=\"\\\"9:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]1->2[]1->3[]0->1[]4->5[]6->7[]4->6[]0->4[]8->9[]0->8[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_arithmetic_operations() {
        // int main() 
        // { 
        //      int x = 5; 
        //      x = x + 1; 
        //      x = x - 1; 
        //      x = x * 2; 
        //      x = x / 2; 
        //      return x; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(5)),
            Token::Semicolon,
            Token::Type(ValueType::Int),
            Token::Identifier("y".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(2)),
            Token::Semicolon,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Plus),
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Minus),
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Multiply),
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Divide),
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::Return,
            Token::Identifier("x".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]3[label=\"\\\"3:Constant(Integer(5))\\\"\"]4[label=\"\\\"4:Declaration(Int)\\\"\"]5[label=\"\\\"5:Identifier(\\\\\\\"y\\\\\\\")\\\"\"]6[label=\"\\\"6:Constant(Integer(2))\\\"\"]7[label=\"\\\"7:Assignment\\\"\"]8[label=\"\\\"8:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]9[label=\"\\\"9:Operator(Plus)\\\"\"]10[label=\"\\\"10:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]11[label=\"\\\"11:Identifier(\\\\\\\"y\\\\\\\")\\\"\"]12[label=\"\\\"12:Assignment\\\"\"]13[label=\"\\\"13:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]14[label=\"\\\"14:Operator(Minus)\\\"\"]15[label=\"\\\"15:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]16[label=\"\\\"16:Identifier(\\\\\\\"y\\\\\\\")\\\"\"]17[label=\"\\\"17:Assignment\\\"\"]18[label=\"\\\"18:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]19[label=\"\\\"19:Operator(Multiply)\\\"\"]20[label=\"\\\"20:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]21[label=\"\\\"21:Identifier(\\\\\\\"y\\\\\\\")\\\"\"]22[label=\"\\\"22:Assignment\\\"\"]23[label=\"\\\"23:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]24[label=\"\\\"24:Operator(Divide)\\\"\"]25[label=\"\\\"25:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]26[label=\"\\\"26:Identifier(\\\\\\\"y\\\\\\\")\\\"\"]27[label=\"\\\"27:Return\\\"\"]28[label=\"\\\"28:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]1->2[]1->3[]0->1[]4->5[]4->6[]0->4[]7->8[]9->10[]9->11[]7->9[]0->7[]12->13[]14->15[]14->16[]12->14[]0->12[]17->18[]19->20[]19->21[]17->19[]0->17[]22->23[]24->25[]24->26[]22->24[]0->22[]27->28[]0->27[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_logical_and_comparison_operators() {
        // int main() 
        // { 
        //      int x = 5; 
        //      int y = 2; 
        //      if (x == 5 && y == 2) 
        //      { 
        //          return 1; 
        //      }
        //      else 
        //      { 
        //          return 0; 
        //      } 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(5)),
            Token::Semicolon,
            Token::Type(ValueType::Int),
            Token::Identifier("y".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(2)),
            Token::Semicolon,
            Token::If,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Equal),
            Token::Constant(Constant::Integer(5)),
            Token::Operator(Operator::LogicalAnd),
            Token::Identifier("y".to_string()),
            Token::Operator(Operator::LessThan),
            Token::Constant(Constant::Integer(3)),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Plus),
            Token::Constant(Constant::Integer(1)),
            Token::Semicolon,
            Token::RightBrace,
            Token::If,
            Token::LeftParen,
            Token::Identifier("y".to_string()),
            Token::Operator(Operator::NotEqual),
            Token::Constant(Constant::Integer(2)),
            Token::Operator(Operator::LogicalOr),
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::GreaterThan),
            Token::Constant(Constant::Integer(5)),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Plus),
            Token::Constant(Constant::Integer(2)),
            Token::Semicolon,
            Token::RightBrace,
            Token::Return,
            Token::Identifier("x".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]3[label=\"\\\"3:Constant(Integer(5))\\\"\"]4[label=\"\\\"4:Declaration(Int)\\\"\"]5[label=\"\\\"5:Identifier(\\\\\\\"y\\\\\\\")\\\"\"]6[label=\"\\\"6:Constant(Integer(2))\\\"\"]7[label=\"\\\"7:IfStatement\\\"\"]8[label=\"\\\"8:BlockItem\\\"\"]9[label=\"\\\"9:Assignment\\\"\"]10[label=\"\\\"10:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]11[label=\"\\\"11:Operator(Plus)\\\"\"]12[label=\"\\\"12:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]13[label=\"\\\"13:Constant(Integer(1))\\\"\"]14[label=\"\\\"14:IfStatement\\\"\"]15[label=\"\\\"15:BlockItem\\\"\"]16[label=\"\\\"16:Assignment\\\"\"]17[label=\"\\\"17:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]18[label=\"\\\"18:Operator(Plus)\\\"\"]19[label=\"\\\"19:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]20[label=\"\\\"20:Constant(Integer(2))\\\"\"]21[label=\"\\\"21:Return\\\"\"]22[label=\"\\\"22:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]1->2[]1->3[]0->1[]4->5[]4->6[]0->4[]9->10[]11->12[]11->13[]9->11[]8->9[]7->8[]0->7[]16->17[]18->19[]18->20[]16->18[]15->16[]14->15[]0->14[]21->22[]0->21[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_parenthesized_expression() {
        // int main() 
        // { 
        //      int x = 5; 
        //      int y = 2; 
        //      if ((x == 5) && (y == 2))
        //      { 
        //          return 1; 
        //      } 
        //      else 
        //      { 
        //          return 0; 
        //      } 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::LeftParen,
            Token::LeftParen,
            Token::Constant(Constant::Integer(1)),
            Token::Operator(Operator::Plus),
            Token::Constant(Constant::Integer(2)),
            Token::RightParen,
            Token::Operator(Operator::Multiply),
            Token::Constant(Constant::Integer(3)),
            Token::RightParen,
            Token::Semicolon,
            Token::Return,
            Token::Identifier("x".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]3[label=\"\\\"3:ParenthesizedExpression\\\"\"]4[label=\"\\\"4:Operator(Multiply)\\\"\"]5[label=\"\\\"5:ParenthesizedExpression\\\"\"]6[label=\"\\\"6:Operator(Plus)\\\"\"]7[label=\"\\\"7:Constant(Integer(1))\\\"\"]8[label=\"\\\"8:Constant(Integer(2))\\\"\"]9[label=\"\\\"9:Constant(Integer(3))\\\"\"]10[label=\"\\\"10:Return\\\"\"]11[label=\"\\\"11:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]1->2[]6->7[]6->8[]5->6[]4->5[]4->9[]3->4[]1->3[]0->1[]10->11[]0->10[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_multiple_global_and_local_vars() {
        // int globalA;
        // int globalB = 10; 
        // int main() 
        // { 
        //      int localA; 
        //      int localB = 20;
        //      return globalB + localB; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("globalA".to_string()),
            Token::Semicolon,
            Token::Type(ValueType::Int),
            Token::Identifier("globalB".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(10)),
            Token::Semicolon,
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("localA".to_string()),
            Token::Semicolon,
            Token::Type(ValueType::Int),
            Token::Identifier("localB".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(20)),
            Token::Semicolon,
            Token::Return,
            Token::Identifier("globalB".to_string()),
            Token::Operator(Operator::Plus),
            Token::Identifier("localB".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:Declaration(Int)\\\"\"]1[label=\"\\\"1:Identifier(\\\\\\\"globalA\\\\\\\")\\\"\"]2[label=\"\\\"2:Declaration(Int)\\\"\"]3[label=\"\\\"3:Identifier(\\\\\\\"globalB\\\\\\\")\\\"\"]4[label=\"\\\"4:Constant(Integer(10))\\\"\"]5[label=\"\\\"5:FunctionDefinition[main]\\\"\"]6[label=\"\\\"6:Declaration(Int)\\\"\"]7[label=\"\\\"7:Identifier(\\\\\\\"localA\\\\\\\")\\\"\"]8[label=\"\\\"8:Declaration(Int)\\\"\"]9[label=\"\\\"9:Identifier(\\\\\\\"localB\\\\\\\")\\\"\"]10[label=\"\\\"10:Constant(Integer(20))\\\"\"]11[label=\"\\\"11:Return\\\"\"]12[label=\"\\\"12:Operator(Plus)\\\"\"]13[label=\"\\\"13:Identifier(\\\\\\\"globalB\\\\\\\")\\\"\"]14[label=\"\\\"14:Identifier(\\\\\\\"localB\\\\\\\")\\\"\"]0->1[]2->3[]2->4[]6->7[]5->6[]8->9[]8->10[]5->8[]12->13[]12->14[]11->12[]5->11[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test] 
    fn test_return_constant() {
        // int main() { return 0; }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Return\\\"\"]2[label=\"\\\"2:Constant(Integer(0))\\\"\"]1->2[]0->1[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_array_assignment_expression() {
        // int main() 
        // { 
        //      int x = 0;
        //      int arr[3]; 
        //      arr[x + 1] = 100; 
        //      return arr[1]; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,
            Token::Type(ValueType::Int),
            Token::Identifier("arr".to_string()),
            Token::LeftBracket,
            Token::Constant(Constant::Integer(3)),
            Token::RightBracket,
            Token::Semicolon,
            Token::Identifier("arr".to_string()),
            Token::LeftBracket,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Plus),
            Token::Constant(Constant::Integer(1)),
            Token::RightBracket,
            Token::Assign,
            Token::Constant(Constant::Integer(100)),
            Token::Semicolon,
            Token::Return,
            Token::Identifier("arr".to_string()),
            Token::LeftBracket,
            Token::Constant(Constant::Integer(1)),
            Token::RightBracket,
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"x\\\\\\\")\\\"\"]3[label=\"\\\"3:Constant(Integer(0))\\\"\"]4[label=\"\\\"4:Declaration(Int)\\\"\"]5[label=\"\\\"5:Identifier(\\\\\\\"arr\\\\\\\")\\\"\"]6[label=\"\\\"6:Array(3)\\\"\"]7[label=\"\\\"7:ArrayAssignment\\\"\"]8[label=\"\\\"8:Identifier(\\\\\\\"arr\\\\\\\")\\\"\"]9[label=\"\\\"9:Constant(Integer(100))\\\"\"]10[label=\"\\\"10:Return\\\"\"]11[label=\"\\\"11:ArrayAccess\\\"\"]12[label=\"\\\"12:Identifier(\\\\\\\"arr\\\\\\\")\\\"\"]13[label=\"\\\"13:Constant(Integer(1))\\\"\"]1->2[]1->3[]0->1[]4->5[]4->6[]0->4[]7->8[]7->9[]0->7[]11->12[]11->13[]10->11[]0->10[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }

    #[test]
    fn test_combined_statements_in_function() {
        // int main() 
        // { 
        //      int a = 1; 
        //      int b; b = 3; 
        //      if (a < b) 
        //      { 
        //          a = a + b; 
        //      } 
        //      for (a = 0; a < 5; a = a + 1)
        //      { 
        //          b = b + a; 
        //      } 
        //      return a + b; 
        // }
        let tokens = vec![
            Token::Type(ValueType::Int),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Type(ValueType::Int),
            Token::Identifier("a".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(1)),
            Token::Semicolon,
            Token::Type(ValueType::Int),
            Token::Identifier("b".to_string()),
            Token::Semicolon,
            Token::Identifier("b".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(3)),
            Token::Semicolon,
            Token::If,
            Token::LeftParen,
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::LessThan),
            Token::Identifier("b".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("a".to_string()),
            Token::Assign,
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::Plus),
            Token::Identifier("b".to_string()),
            Token::Semicolon,
            Token::RightBrace,
            Token::For,
            Token::LeftParen,
            Token::Identifier("a".to_string()),
            Token::Assign,
            Token::Constant(Constant::Integer(0)),
            Token::Semicolon,
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::LessThan),
            Token::Constant(Constant::Integer(5)),
            Token::Semicolon,
            Token::Identifier("a".to_string()),
            Token::Assign,
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::Plus),
            Token::Constant(Constant::Integer(1)),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("b".to_string()),
            Token::Assign,
            Token::Identifier("b".to_string()),
            Token::Operator(Operator::Plus),
            Token::Identifier("a".to_string()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Return,
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::Plus),
            Token::Identifier("b".to_string()),
            Token::Semicolon,
            Token::RightBrace,
        ];

        let answer = String::from("digraph{0[label=\"\\\"0:FunctionDefinition[main]\\\"\"]1[label=\"\\\"1:Declaration(Int)\\\"\"]2[label=\"\\\"2:Identifier(\\\\\\\"a\\\\\\\")\\\"\"]3[label=\"\\\"3:Constant(Integer(1))\\\"\"]4[label=\"\\\"4:Declaration(Int)\\\"\"]5[label=\"\\\"5:Identifier(\\\\\\\"b\\\\\\\")\\\"\"]6[label=\"\\\"6:Assignment\\\"\"]7[label=\"\\\"7:Identifier(\\\\\\\"b\\\\\\\")\\\"\"]8[label=\"\\\"8:Constant(Integer(3))\\\"\"]9[label=\"\\\"9:IfStatement\\\"\"]10[label=\"\\\"10:BlockItem\\\"\"]11[label=\"\\\"11:Assignment\\\"\"]12[label=\"\\\"12:Identifier(\\\\\\\"a\\\\\\\")\\\"\"]13[label=\"\\\"13:Operator(Plus)\\\"\"]14[label=\"\\\"14:Identifier(\\\\\\\"a\\\\\\\")\\\"\"]15[label=\"\\\"15:Identifier(\\\\\\\"b\\\\\\\")\\\"\"]16[label=\"\\\"16:ForStatement\\\"\"]17[label=\"\\\"17:Assignment\\\"\"]18[label=\"\\\"18:Identifier(\\\\\\\"a\\\\\\\")\\\"\"]19[label=\"\\\"19:Constant(Integer(0))\\\"\"]20[label=\"\\\"20:Operator(LessThan)\\\"\"]21[label=\"\\\"21:Identifier(\\\\\\\"a\\\\\\\")\\\"\"]22[label=\"\\\"22:Constant(Integer(5))\\\"\"]23[label=\"\\\"23:Assignment\\\"\"]24[label=\"\\\"24:Identifier(\\\\\\\"a\\\\\\\")\\\"\"]25[label=\"\\\"25:Operator(Plus)\\\"\"]26[label=\"\\\"26:Identifier(\\\\\\\"a\\\\\\\")\\\"\"]27[label=\"\\\"27:Constant(Integer(1))\\\"\"]28[label=\"\\\"28:BlockItem\\\"\"]29[label=\"\\\"29:Assignment\\\"\"]30[label=\"\\\"30:Identifier(\\\\\\\"b\\\\\\\")\\\"\"]31[label=\"\\\"31:Operator(Plus)\\\"\"]32[label=\"\\\"32:Identifier(\\\\\\\"b\\\\\\\")\\\"\"]33[label=\"\\\"33:Identifier(\\\\\\\"a\\\\\\\")\\\"\"]34[label=\"\\\"34:Return\\\"\"]35[label=\"\\\"35:Operator(Plus)\\\"\"]36[label=\"\\\"36:Identifier(\\\\\\\"a\\\\\\\")\\\"\"]37[label=\"\\\"37:Identifier(\\\\\\\"b\\\\\\\")\\\"\"]1->2[]1->3[]0->1[]4->5[]0->4[]6->7[]6->8[]0->6[]11->12[]13->14[]13->15[]11->13[]10->11[]9->10[]0->9[]17->18[]17->19[]16->17[]20->21[]20->22[]16->20[]23->24[]25->26[]25->27[]23->25[]16->23[]29->30[]31->32[]31->33[]29->31[]28->29[]16->28[]0->16[]35->36[]35->37[]34->35[]0->34[]}");
        // 改行と空白を削除
        let answer = answer.replace(" ", "").replace("\n", "");

        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut tree_viewer = TreeViewer::new();
        for root in &parser.roots {
            tree_viewer.make_tree(root);
        }

        let result = tree_viewer.get_dot().replace(" ", "").replace("\n", "");

        assert_eq!(result, answer);
    }
}
