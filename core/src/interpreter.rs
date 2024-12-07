use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::interpreter::VariableType::Int;
use crate::lexical::{Constant, Operator, Token, UnaryOperator, ValueType};
use crate::parser::{Leaf, Node};

#[derive(Debug, Clone)]
pub struct Value
{
    name: String,
    value: VariableType,
}

impl Value
{
    pub fn new(name: &str, value: VariableType) -> Self
    {
        Value
        {
            name: name.to_string(),
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Array
{
    name: String,
    values: Vec<VariableType>,
}

#[derive(Debug, Clone)]
pub enum VariableType
{
    Int(i32),
    Float(f64),
}

pub enum Variable {
    Value(Value),
    Array(Array),
}


pub struct Interpreter
{
    roots: Vec<Rc<RefCell<Node>>>,
    variables: Vec<Variable>,
}

impl Interpreter
{
    pub fn new(roots: &Vec<Rc<RefCell<Node>>>) -> Self
    {
        Interpreter
        {
            roots: roots.clone(),
            variables: Vec::new(),
        }
    }

    pub fn run(&mut self)
    {
        let mut roots = self.roots.clone();
        for root in roots.iter()
        {
            self.interpret_node(root);
        }
    }

    pub fn show_variables(&self)
    {
        for variable in &self.variables
        {
            match variable
            {
                Variable::Value(value) =>
                    {
                        println!("{} = {:?}", value.name, value.value);
                    }
                Variable::Array(array) =>
                    {
                        println!("{} = {:?}", array.name, array.values);
                    }
            }
        }
    }

    fn interpret_node(&mut self, node: &Rc<RefCell<Node>>)
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
                                self.assign(variable_type, identifier, value);
                            }
                        }
                    }
                _ => {
                    panic!("未対応のノードです : {:?}", val);
                }
            }
        }
    }

    fn assign(&mut self, value_type: &ValueType, identifier: String, value: VariableType)
    {
        match value_type
        {
            ValueType::Int =>
                {
                    self.variables.push(Variable::Value(Value { name: identifier, value }));
                }
            ValueType::Float =>
                {
                    self.variables.push(Variable::Value(Value { name: identifier, value }));
                }
            _ => {
                panic!("未対応の型です : {:?}", value_type);
            }
        }
    }

    fn statement(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
    {
        if let Some(val) = node.borrow().val()
        {
            match val
            {
                // 代入
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
                _ => {
                    panic!("未対応のノードです : {:?}", val);
                }
            }
        }

        panic!("未対応のノードです");
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
                                Int(-val as i32)
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
                    }
                }
            _ => {
                panic!("未対応の演算子です : {:?}", op);
            }
        }
    }

    fn identifier(&mut self, identifier: &String) -> VariableType
    {
        for variable in &self.variables
        {
            match variable
            {
                Variable::Value(value) =>
                    {
                        if value.name == *identifier
                        {
                            return value.value.clone();
                        }
                    }
                _ => {}
            }
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
                    Int(lhs + rhs as i32)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    Int((lhs + rhs as f64) as i32)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    Int((lhs + rhs) as i32)
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
                    Int(lhs - rhs as i32)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    Int((lhs - rhs as f64) as i32)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    Int((lhs - rhs) as i32)
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
                    Int(lhs * rhs as i32)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    Int((lhs * rhs as f64) as i32)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    Int((lhs * rhs) as i32)
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
                    Int(lhs / rhs as i32)
                }
            (VariableType::Float(lhs), VariableType::Int(rhs)) =>
                {
                    Int((lhs / rhs as f64) as i32)
                }
            (VariableType::Float(lhs), VariableType::Float(rhs)) =>
                {
                    Int((lhs / rhs) as i32)
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