use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::lexical::Token;
use crate::parser::{Leaf, Node};

#[derive(Debug, Clone)]
pub struct Value<T>
{
    name: String,
    value: T,
}

#[derive(Debug, Clone)]
pub struct Array<T>
{
    name: String,
    values: Vec<T>,
}


pub enum Variable {
    Value(Rc<RefCell<dyn Any>>),
    Array(Rc<RefCell<dyn Any>>),
}


pub struct Interpreter
{
    root: Rc<RefCell<Node>>,
    variables: Vec<Variable>,
}

impl Interpreter
{
    pub fn new(root: Rc<RefCell<Node>>) -> Self
    {
        Interpreter
        {
            root,
            variables: Vec::new(),
        }
    }

    pub fn run(&mut self)
    {}

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
                            let identifier = self.identifier(lhs);
                            
                            // 
                        }
                    }
                _ => {
                    panic!("未対応のノードです : {:?}", val);
                }
            }
        }
    }
    
    fn variable(&mut self, node: &Rc<RefCell<Node>>) -> Option<Variable>
    {
        
     
        
        None
    }

    fn identifier(&self, node: &Rc<RefCell<Node>>) -> String
    {
        let mut identifier = String::new();
        if let Some(val) = node.borrow().val()
        {
            match val
            {

                _ => {
                    panic!("識別子ではありません : {:?}", val);
                }
            }
        }
        identifier
    }
}