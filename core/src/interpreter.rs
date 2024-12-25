use crate::interpreter::VariableType::Int;
use crate::lexical::{Constant, Operator, UnaryOperator, ValueType};
use crate::parser::{FunctionCall, FunctionDefinition, Leaf, Node};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Array
{
    name: String,
    values: Vec<VariableType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType
{
    Int(i32),
    Float(f64),
    Void,
}

// VariableType の format
impl std::fmt::Display for VariableType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            VariableType::Int(val) => write!(f, "{}", val),
            VariableType::Float(val) => write!(f, "{}", val),
            VariableType::Void => write!(f, "void"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    Value(VariableType),
    Array(Array),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scope
{
    Global,
    Local,
}

pub struct Interpreter
{
    roots: Vec<Rc<RefCell<Node>>>,

    // すべての領域からアクセス可能な変数
    global_variables: HashMap<String, Variable>,

    // 関数の中でのみアクセス可能な変数
    local_variables: Vec<Vec<HashMap<String, Variable>>>,

    function_definition: HashMap<String, FunctionDefinition>,

    scope: Scope,
}

impl Interpreter
{
    pub fn new(roots: &Vec<Rc<RefCell<Node>>>) -> Self
    {
        Interpreter
        {
            roots: roots.clone(),
            global_variables: HashMap::new(),
            local_variables: Vec::new(),
            function_definition: HashMap::new(),
            scope: Scope::Global,
        }
    }

    pub fn global_variables(&self) -> &HashMap<String, Variable>
    {
        &self.global_variables
    }

    pub fn run(&mut self) -> VariableType
    {
        let mut roots = self.roots.clone();
        for root in roots.iter()
        {
            self.interpret_node(root);
        }

        // main 関数を呼び出し実行する
        if let Some(main) = self.function_definition.get("main")
        {
            // main の function_call を作成
            let function_call = FunctionCall::new("main".to_string());

            // main 関数を呼び出し
            self.scope = Scope::Local;
            let val = self.function_call(&function_call);
            self.scope = Scope::Global;

            return val;
        } else {
            panic!("main 関数が見つかりません");
        }
    }

    pub fn show_variables(&self)
    {
        for (name, variable) in &self.global_variables
        {
            match variable
            {
                Variable::Value(value) =>
                    {
                        //println!("{} = {:?}", name, value);
                    }
                Variable::Array(array) =>
                    {
                        unimplemented!("配列の表示は未実装です");
                    }
            }
        }
    }

    fn interpret_node(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
    {
        if let Some(val) = node.clone().borrow().val()
        {
            println!("interpret_node val: {}", val);
            match val
            {
                Leaf::Declaration(variable_type) =>
                    {
                        // node の左側から変数名を取得
                        if let Some(lhs) = node.borrow().lhs()
                        {
                            let identifier = self.identifier_name(lhs);

                            // node の右側から値を取得
                            if let Some(rhs) = node.borrow().rhs()
                            {
                                let value = self.statement(rhs);
                                self.variable_definition(variable_type, identifier, value);
                            }
                        }
                    }
                Leaf::FunctionDefinition(function_definition) =>
                    {
                        let name = function_definition.name();
                        self.function_definition.insert(name.clone(), function_definition.clone());
                    }

                // 関数呼び出し
                Leaf::FunctionCall(function_call) =>
                    {
                        self.function_call(function_call);
                    }

                // return 文
                Leaf::Return =>
                    {
                        if let Some(lhs) = node.borrow().lhs()
                        {
                            let value = self.statement(lhs);
                            //println!("return {:?}", value);

                            return value;
                        }
                    }
                Leaf::Assignment =>
                    {
                        return self.variable_assignment(node);
                    }
                Leaf::IfStatement(_) =>
                    {
                        return self.selection_statement(node);
                    }
                Leaf::WhileStatement =>
                    {
                        return self.iteration_statement(node);
                    }
                _ => {
                    panic!("未対応のノードです : {:?}", val);
                }
            }
        }
        VariableType::Void
    }


    fn iteration_statement(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
    {
        // while 文の条件式を取得
        if let Some(condition_root) = node.borrow().lhs()
        {
            let mut condition = true;
            condition = self.condition(condition_root);
            
            // condition != 0 の場合は while 文の中身を実行
            while condition {
                if let Some(rhs) = node.borrow().rhs()
                {
                    if let Some(Leaf::BlockItem(nodes)) = rhs.borrow().val()
                    {
                        self.compound_statement(nodes, true);
                    }
                }
                condition = self.condition(condition_root);
            }
        }

        VariableType::Void
    }

    fn condition(&mut self, node: &Rc<RefCell<Node>>) -> bool
    {
        let condition = self.statement(node);

        match condition
        {
            VariableType::Int(val) => {
                val != 0
            }
            VariableType::Float(val) => {
                val != 0.0
            }
            _ => {
                panic!("while 文の条件式が対応していません");
            }
        }
    }

    fn selection_statement(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
    {
        // if 文の条件式を取得
        if let Some(Leaf::IfStatement(expression)) = node.borrow().val()
        {
            let condition = self.statement(expression);

            // condition != 0 の場合は if 文の中身を実行
            let mut condition_value: i32 = 0;
            match condition
            {
                VariableType::Int(val) => condition_value = val,
                VariableType::Float(val) => condition_value = val as i32,
                _ => {
                    panic!("if 文の条件式が対応していません");
                }
            }

            if condition_value != 0
            {
                if let Some(lhs) = node.borrow().lhs()
                {
                    if let Some(Leaf::BlockItem(nodes)) = lhs.borrow().val()
                    {
                        return self.compound_statement(nodes, true);
                    }
                }
            } else {
                if let Some(rhs) = node.borrow().rhs()
                {
                    // else のときと, else if のとき
                    if let Some(Leaf::IfStatement(_)) = rhs.borrow().val()
                    {
                        return self.selection_statement(rhs);
                    } else {
                        if let Some(Leaf::BlockItem(nodes)) = rhs.borrow().val()
                        {
                            return self.compound_statement(nodes, true);
                        }
                    }
                }
            }
        } else {
            panic!("if 文の条件式が取得できません");
        }

        VariableType::Void
    }

    fn variable_assignment(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
    {
        // 左辺に識別子があり, 変数として登録されていることを確認する
        if let Some(lhs) = node.borrow().lhs()
        {
            let identifier = self.identifier_name(lhs);

            if let Some(rhs) = node.borrow().rhs()
            {
                let value = self.statement(rhs);
                // ローカル変数から検索
                if let Some(local_variables) = self.local_variables.last_mut()
                {
                    println!("local_variables : {:?} = {:?}", local_variables, value);
                    // 最後のスコープから検索
                    for local_variable in local_variables.iter_mut().rev()
                    {
                        if let Some(variable) = local_variable.get_mut(&identifier)
                        {
                            if let Variable::Value(variable) = variable
                            {
                                *variable = value;
                                return VariableType::Void;
                            }
                        }
                    }
                    println!("global_variables : {:?}", self.global_variables);
                    // グローバル変数から検索
                    if let Some(variable) = self.global_variables.get_mut(&identifier)
                    {
                        if let Variable::Value(variable) = variable
                        {
                            *variable = value;
                        } else {
                            panic!("Global 変数が見つかりません : {}", identifier);
                        }
                    } else {
                        panic!("Global 変数が見つかりません : {}", identifier);
                    }
                }
            } else {
                panic!("右辺に識別子がありません");
            }
        } else {
            panic!("左辺に識別子がありません");
        }

        VariableType::Void
    }

    fn variable_definition(&mut self, value_type: &ValueType, identifier: String, value: VariableType)
    {
        match value_type
        {
            ValueType::Int =>
                {
                    if let VariableType::Float(val) = value
                    {
                        self.insert_variable(identifier, VariableType::Int(val as i32));
                    } else {
                        self.insert_variable(identifier, value);
                    }
                }
            ValueType::Float =>
                {
                    if let VariableType::Int(val) = value
                    {
                        self.insert_variable(identifier, VariableType::Float(val as f64));
                    } else {
                        self.insert_variable(identifier, value);
                    }
                }
            _ => {
                panic!("未対応の型です : {:?}", value_type);
            }
        }
    }

    fn insert_variable(&mut self, identifier: String, value: VariableType)
    {
        match self.scope
        {
            Scope::Global =>
                {
                    self.global_variables.insert(identifier, Variable::Value(value));
                }
            Scope::Local =>
                {
                    if let Some(local_variables) = self.local_variables.last_mut()
                    {
                        local_variables.last_mut().unwrap().insert(identifier, Variable::Value(value));
                    }
                }
        }
    }

    fn statement(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
    {
        if let Some(val) = node.borrow().val()
        {
            match val
            {
                // 演算子
                Leaf::Operator(op) =>
                    {
                        if let Some((lhs, rhs))
                            = node.borrow().get_lhs_and_rhs()
                        {
                            return self.operator(op, lhs, rhs);
                        }
                    }

                // 定数
                Leaf::Constant(value) =>
                    {
                        return self.constant(value);
                    }

                // 識別子
                Leaf::Identifier(identifier) =>
                    {
                        return self.identifier(identifier);
                    }

                // 単項演算子
                Leaf::UnaryExpression(op) =>
                    {
                        if let Some(lhs) = node.borrow().lhs()
                        {
                            return self.unary_expression(op, lhs);
                        }
                    }

                // 括弧で囲まれた式
                Leaf::ParenthesizedExpression =>
                    {
                        if let Some(lhs) = node.borrow().lhs()
                        {
                            return self.statement(lhs);
                        }
                    }
                Leaf::FunctionCall(function_call) =>
                    {
                        let value = self.function_call(function_call);

                        // statement で void の場合はエラー
                        if let VariableType::Void = value
                        {
                            panic!("void は代入できません");
                        }

                        return value;
                    }
                _ => {
                    panic!("未対応のノードです : {:?}", val);
                }
            }
        }

        panic!("未対応のノードです");
    }

    fn function_call(&mut self, function_call: &FunctionCall) -> VariableType
    {
        let name = function_call.name();
        let function_definitions = self.function_definition.clone();
        //println!("function_call : {}", name);

        if let Some(function_definition) = function_definitions.get(name)
        {
            let mut new_variables: HashMap<String, Variable> = HashMap::new();

            // 引数がある場合計算する
            let function_arguments = function_call.arguments();

            // 引数がある場合は引数を計算してローカル変数に追加
            if !function_arguments.is_empty()
            {
                //println!("function_call 引数の数 : {}", function_arguments.len());
                // 引数の数と function-definition の引数リストの数が一致することを確認する
                if function_arguments.len() != function_definition.arguments().len()
                {
                    panic!("引数の数が一致しません");
                }

                for (i, argument) in function_arguments.iter().enumerate()
                {
                    let argument_value = self.statement(argument);
                    let argument_name = function_definition.arguments()[i].identify().clone();
                    //println!("argument_name : {}", argument_name);
                    // 引数をローカル変数に追加
                    if let Some(local_variables) = self.local_variables.last_mut()
                    {
                        new_variables.insert(argument_name, Variable::Value(argument_value));
                    }
                }
            }

            // 新しくローカル変数を追加
            self.local_variables.push(Vec::new());
            self.local_variables.last_mut().unwrap().push(new_variables);

            let return_value = self.compound_statement(function_definition.body(),
                                                       false);

            // ローカル変数を削除
            self.local_variables.pop();

            return_value
        } else {
            panic!("関数が見つかりません : {}", name);
        }
    }

    fn compound_statement(&mut self, nodes: &Vec<Rc<RefCell<Node>>>,
                          is_generate_local_variables: bool) -> VariableType
    {
        if is_generate_local_variables
        {
            if let Some(local_variables) = self.local_variables.last_mut()
            {
                local_variables.push(HashMap::new());
            }
        }

        let mut return_value = VariableType::Void;
        for statement in nodes.iter()
        {
            return_value = self.interpret_node(statement);
        }

        if is_generate_local_variables
        {
            if let Some(local_variables) = self.local_variables.last_mut()
            {
                local_variables.pop();
            }
        }

        return_value
    }

    fn unary_expression(&mut self, op: &UnaryOperator, lhs: &Rc<RefCell<Node>>) -> VariableType
    {
        let lhs = self.statement(lhs);
        match op
        {
            UnaryOperator::Minus =>
                {
                    match lhs
                    {
                        VariableType::Int(val) =>
                            {
                                Int(-val)
                            }
                        VariableType::Float(val) =>
                            {
                                VariableType::Float(-val)
                            }
                        _ => {
                            panic!("未対応の型です");
                        }
                    }
                }
            UnaryOperator::LogicalNot =>
                {
                    match lhs
                    {
                        VariableType::Int(val) =>
                            {
                                Int(if val == 0 { 1 } else { 0 })
                            }
                        VariableType::Float(val) =>
                            {
                                Int(if val == 0.0 { 1 } else { 0 })
                            }
                        _ => {
                            panic!("未対応の型です");
                        }
                    }
                }
            _ => {
                panic!("未対応の演算子です : {:?}", op);
            }
        }
    }

    fn identifier(&mut self, identifier: &String) -> VariableType
    {
        // ローカル変数から検索.最後のスコープから検索する
        if let Some(local_variables) = self.local_variables.last_mut()
        {
            for variable in local_variables.iter().rev()
            {
                if let Some(variable) = variable.get(identifier)
                {
                    match variable
                    {
                        Variable::Value(value) => return value.clone(),
                        _ => {
                            panic!("未対応の変数です : {:?}", variable);
                        }
                    }
                }
            }
        }

        // グローバル変数から検索
        if let Some(Variable::Value(value)) = self.global_variables.get(identifier) {
            return value.clone();
        }

        panic!("未定義の変数です : {}", identifier);
    }

    fn constant(&mut self, value: &Constant) -> VariableType
    {
        match value
        {
            Constant::Integer(val) =>
                {
                    VariableType::Int(*val)
                }
            Constant::Float(val) =>
                {
                    VariableType::Float(*val)
                }
            _ => {
                panic!("未対応の定数です : {:?}", value);
            }
        }
    }


    fn operator(&mut self, op: &Operator, lhs: &Rc<RefCell<Node>>, rhs: &Rc<RefCell<Node>>) -> VariableType
    {
        let lhs = self.statement(lhs);
        let rhs = self.statement(rhs);
        let mut result: VariableType = Int(0);
        match op
        {
            Operator::LogicalOr =>
                {
                    result = self.logical_or(lhs, rhs);
                }
            Operator::LogicalAnd =>
                {
                    result = self.logical_and(lhs, rhs);
                }
            Operator::Equal =>
                {
                    result = self.equal(lhs, rhs);
                }
            Operator::NotEqual =>
                {
                    result = self.not_equal(lhs, rhs);
                }
            Operator::LessThan =>
                {
                    result = self.less_than(lhs, rhs);
                }
            Operator::GreaterThan =>
                {
                    result = self.greater_than(lhs, rhs);
                }
            Operator::LessThanOrEqual =>
                {
                    result = self.less_than_or_equal(lhs, rhs);
                }
            Operator::GreaterThanOrEqual =>
                {
                    result = self.greater_than_or_equal(lhs, rhs);
                }
            Operator::Plus =>
                {
                    result = self.add(lhs, rhs);
                }
            Operator::Minus =>
                {
                    result = self.sub(lhs, rhs);
                }
            Operator::Multiply =>
                {
                    result = self.mul(lhs, rhs);
                }
            Operator::Divide =>
                {
                    result = self.div(lhs, rhs);
                }
            Operator::Modulo =>
                {
                    result = self.remainder(lhs, rhs);
                }
            _ => {
                panic!("未対応の演算子です : {:?}", op);
            }
        }

        result
    }

    // 加算演算子　'+'
    fn add(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    Int(lhs + rhs)
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    VariableType::Float(lhs as f64 + rhs)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    VariableType::Float(lhs + rhs as f64)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    VariableType::Float(lhs + rhs)
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 減算演算子　'-'
    fn sub(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    Int(lhs - rhs)
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    VariableType::Float(lhs as f64 - rhs)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    VariableType::Float(lhs - rhs as f64)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    VariableType::Float(lhs - rhs)
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 乗算演算子　'*'
    fn mul(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    Int(lhs * rhs)
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    VariableType::Float(lhs as f64 * rhs)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    VariableType::Float(lhs * rhs as f64)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    VariableType::Float(lhs * rhs)
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 除算演算子　'/'
    fn div(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        // 右辺値が0の場合はエラー
        match rhs
        {
            VariableType::Int(val) if val == 0 =>
                {
                    panic!("0で割ることはできません");
                }
            VariableType::Float(val) if val == 0.0 =>
                {
                    panic!("0で割ることはできません");
                }
            _ => {}
        }

        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    Int(lhs / rhs)
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    VariableType::Float(lhs as f64 / rhs)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    VariableType::Float(lhs / rhs as f64)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    VariableType::Float(lhs / rhs)
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 余り演算子　'%'
    fn remainder(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        // 右辺値が0の場合はエラー
        match rhs
        {
            VariableType::Int(val) if val == 0 =>
                {
                    panic!("0で割ることはできません");
                }
            VariableType::Float(val) if val == 0.0 =>
                {
                    panic!("0で割ることはできません");
                }
            _ => {}
        }

        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    Int(lhs % rhs)
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    Int(lhs % rhs as i32)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    Int((lhs % rhs as f64) as i32)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    Int((lhs % rhs) as i32)
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 同値演算子　'=='
    fn equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs == rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs == rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs == rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs == rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 否定演算子　'!='
    fn not_equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 小なり演算子　'<'
    fn less_than(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs < rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs < rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs < rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs < rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 大なり演算子　'>'
    fn greater_than(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs > rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs > rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs > rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs > rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 小なりイコール演算子　'<='
    fn less_than_or_equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs <= rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs <= rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs <= rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs <= rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 大なりイコール演算子　'>='
    fn greater_than_or_equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs >= rhs;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs >= rhs as i32;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs >= rhs as f64;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs >= rhs;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 論理和　'||'
    fn logical_or(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != 0 || rhs != 0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != 0 || rhs != 0.0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != 0.0 || rhs != 0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != 0.0 || rhs != 0.0;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    // 論理積　'&&'
    fn logical_and(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
    {
        match (lhs, rhs)
        {
            (VariableType::Int(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != 0 && rhs != 0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Int(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != 0 && rhs != 0.0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    let result = lhs != 0.0 && rhs != 0;
                    Int(if result { 1 } else { 0 })
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    let result = lhs != 0.0 && rhs != 0.0;
                    Int(if result { 1 } else { 0 })
                }
            _ => {
                panic!("未対応の型です");
            }
        }
    }

    fn identifier_name(&self, node: &Rc<RefCell<Node>>) -> String
    {
        let mut identifier = String::new();
        if let Some(val) = node.borrow().val()
        {
            match val
            {
                Leaf::Identifier(name) => {
                    identifier = name.clone();
                }
                _ => {
                    panic!("識別子ではありません : {:?}", val);
                }
            }
        }
        identifier
    }
}

#[cfg(test)]
mod tests
{
    use crate::interpreter::VariableType::{Float, Int};
    use crate::interpreter::{Interpreter, Variable, VariableType};
    use crate::parser::Parser;
    use std::collections::HashMap;
    use crate::lexical::Lexer;

    #[test]
    fn test_static_variable()
    {
        let program = String::from("
        int x = (10 + 20) * 3 - 4 / 2;
        int fib = 0;
        int add(int a, int b) { return a + b; }
        int sub(int a, int b) { return a - b; }
        int fibo(int n) {
            if (n == 0) {
                return 0;
            } else if (n == 1) {
                return 1;
            } else {
                return fibo(n - 1) + fibo(n - 2);
            }
        }
    
        int main(void) {
            int a = 10;
            int b = 20;
            a = add(a * 2, (b + 10) / 2);
            int c = sub(a, b);
            int d = c + x;
            fib = fibo(10);
            return d;
        }
        ");

        let mut lexer = Lexer::new(program);
        lexer.tokenize();

        let tokens = lexer.tokens().clone();
        let mut parser = Parser::new(tokens);
        parser.parse();

        let mut interpreter = Interpreter::new(parser.roots());
        let val = interpreter.run();

        assert_eq!(val, Int(103));

        // global 変数の値を確認する
        let global_variables = interpreter.global_variables();
        assert_eq!(global_variables.len(), 2);

        let mut variables = HashMap::new();
        variables.insert("x".to_string(), Variable::Value(Int(88)));
        variables.insert("fib".to_string(), Variable::Value(Int(55)));

        for (name, variable) in global_variables
        {
            println!("{} = {:?}", name, variable);
            match variable
            {
                Variable::Value(value) =>
                    {
                        assert_eq!(variables.get(name).unwrap(), &Variable::Value(value.clone()));
                    }
                _ => {
                    panic!("未対応の変数です");
                }
            }
        }
    }
}