use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::{Display, Error, write};
use anyhow::{bail, ensure, Context, Result};
use std::io::{BufRead, BufReader, stdin};
use std::fs::File;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Operand
{
    Plus,
    Minus,
    Divide,
    Multi,
}

impl fmt::Display for Operand
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self
        {
            Operand::Plus => write!(f, "+"),
            Operand::Minus => write!(f, "-"),
            Operand::Divide => write!(f, "/"),
            Operand::Multi => write!(f, "*"),
        }
    }
}

impl Operand
{
    pub fn from_string(op: String) -> Operand
    {
        let str = op.as_str();
        match str
        {
            "+" => Operand::Plus,
            "-" => Operand::Minus,
            "/" => Operand::Divide,
            "*" => Operand::Multi,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Token
{
    Op(Operand),
    Val(i32),
    Other,
}

impl fmt::Display for Token
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match &self {
            Token::Op(operand) => write!(f, "{}", operand),
            Token::Val(value) => write!(f, "{}", value),
            Token::Other => write!(f, "this is not token"),
        }
    }
}

impl Token
{
    pub fn new(str: String) -> Token
    {
        if let Ok(value) = str.parse::<i32>()
        {
            Token::Val(value)
        } else {
            Token::Op(Operand::from_string(str))
        }
    }

}

// TODO: 値を可変にできるようにする.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TreeNode
{
    pub val: Option<Token>,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode
{
    pub fn new(value: Token) -> TreeNode
    {
        TreeNode { val: Some(value.clone()), left: None, right: None }
    }

    pub fn new_val(value: i32) -> TreeNode
    {
        TreeNode { val: Some(Token::Val(value)), left: None, right: None }
    }

    pub fn new_op(op: String) -> TreeNode
    {
        TreeNode { val: Some(Token::Op(Operand::from_string(op))), left: None, right: None }
    }

    pub fn new_string(str: String) -> TreeNode
    {
        TreeNode { val: Some(Token::new(str)), left: None, right: None }
    }

    pub fn empty_node() -> TreeNode
    {
        TreeNode { val: None, left: None, right: None }
    }

    pub fn add_value(&mut self, val: Token)
    {
        self.val = Some(val.clone());
    }

    pub fn add_left_node(&mut self, left: TreeNode) -> &mut TreeNode
    {
        self.left = Some(Rc::new(RefCell::new(left)));
        self
    }

    pub fn add_right_node(&mut self, right: TreeNode) -> &mut TreeNode
    {
        self.right = Some(Rc::new(RefCell::new(right)));
        self
    }
}

pub fn create_node(val: String) -> Rc<RefCell<TreeNode>>
{
    Rc::new(RefCell::new(TreeNode::new_string(val.clone())))
}

// 幅優先探索で構造を表示
#[allow(unreachable_code)]
pub fn show_tree(root: &TreeNode) -> String
{
    let mut v: VecDeque<TreeNode> = VecDeque::new();
    v.push_back(root.clone());
    let mut result_str = String::new();

    if TreeNode::empty_node() == root.clone()
    {
        return "".to_string();
    }
    // clone したらコピーに時間がかかると思われる
    loop
    {
        let Some(node) = v.pop_front() else { break; };

        if let Some(token) = node.val.borrow()
        {
            result_str += format!("{}, ", token).borrow();
        } else {
            result_str += "";
        }

        if let Some(node) = node.left.clone()
        {
            v.push_back(node.take());
        }

        if let Some(node) = node.right.clone()
        {
            v.push_back(node.take());
        }
    }
    result_str
}