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
    global_variables: HashMap<String, Variable>,
    local_variables: Vec<HashMap<String, Variable>>,
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

    pub fn variables(&self) -> &HashMap<String, Variable>
    {
        &self.global_variables
    }

    pub fn run(&mut self)
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
            self.function_call(&function_call);
            self.scope = Scope::Global;
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
                        println!("{} = {:?}", name, value);
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
        if let Some(val) = node.borrow().val()
        {
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
                        self.scope = Scope::Local;
                        self.function_call(function_call);
                        self.scope = Scope::Global;
                    }

                // return 文
                Leaf::Return =>
                    {
                        if let Some(lhs) = node.borrow().lhs()
                        {
                            let value = self.statement(lhs);
                            println!("return {:?}", value);

                            return value;
                        }
                    }
                Leaf::Assignment =>
                    {}
                _ => {
                    panic!("未対応のノードです : {:?}", val);
                }
            }
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
                if let Some(local_variable) = self.local_variables.last_mut()
                {
                    if let Some(variable) = local_variable.get_mut(&identifier)
                    {
                        if let Variable::Value(variable) = variable
                        {
                            *variable = value;
                        }
                    }
                } else {
                    // グローバル変数から検索
                    if let Some(variable) = self.global_variables.get_mut(&identifier)
                    {
                        if let Variable::Value(variable) = variable
                        {
                            *variable = value;
                        }
                    }
                }
            }
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
                    self.local_variables.last_mut().unwrap().insert(identifier, Variable::Value(value));
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
                        self.scope = Scope::Local;
                        let value = self.function_call(function_call);
                        self.scope = Scope::Global;

                        // statement で void の場合はエラー
                        if let VariableType::Void = value
                        {
                            panic!("void は代入できません");
                        }
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
        

        if let Some(function_definition) = function_definitions.get(name)
        {
            // 新しくローカル変数を追加
            self.local_variables.push(HashMap::new());

            // 引数がある場合計算する
            let function_arguments = function_call.arguments();

            // 引数がある場合は引数を計算してローカル変数に追加
            if !function_arguments.is_empty()
            {
                // 引数の数と function-definition の引数リストの数が一致することを確認する
                if function_arguments.len() != function_definition.arguments().len()
                {
                    panic!("引数の数が一致しません");
                }

                for (i, argument) in function_arguments.iter().enumerate()
                {
                    let argument_value = self.statement(argument);
                    let argument_name = function_definition.arguments()[i].identify().clone();

                    // 引数をローカル変数に追加
                    self.local_variables.last_mut().unwrap().insert(argument_name, Variable::Value(argument_value));
                }
            }

            let mut return_value = VariableType::Void;

            // 関数の本体を実行
            for statement in function_definition.body()
            {
                return_value = self.interpret_node(statement);
                if let VariableType::Void = return_value
                {
                    continue;
                }
            }

            // ローカル変数を削除
            self.local_variables.pop();

            if let VariableType::Void = return_value
            {
                return return_value;
            }
        } else {
            panic!("関数が見つかりません : {}", name);
        }


        VariableType::Void
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
        if let Some(value) = self.local_variables.iter().rev()
            .find_map(|local_variable| local_variable.get(identifier))
        {
            if let Variable::Value(value) = value {
                return value.clone();
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