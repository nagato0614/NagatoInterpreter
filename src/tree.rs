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

/// トークンを生成する
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

// ２分木を配列で表現する
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Tree
{
    pub nodes: Vec<TreeNode>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TreeNode
{
    pub val: Option<Token>,
    pub left: Option<u32>,
    pub right: Option<u32>,
}

impl Tree
{
    pub fn new(token: Token) -> Tree
    {
        let mut tree = Tree::default();
        let mut root = TreeNode::new(token);
        tree.nodes.push(root);
        return tree;
    }

    pub fn empty_node() -> Tree
    {
        Tree::default()
    }

    /// 二分木にノードを追加する
    pub fn add_value(&mut self, token: Token)
    {
        // ノード生成
        let new_node = TreeNode::new(token);

        // 追加するノードを探す
        let mut node = self.find_empty_node();
    }

    /// 子ノードを持っていないノードを見つける
    /// 幅優先探索
    pub fn find_empty_node(&self) -> Option<&TreeNode>
    {
        // ツリーがからの場合はNoneを返す
        if self.nodes.len() == 0
        {
            return None;
        }

        // ルートノードから探索を開始する
        let mut queue = VecDeque::new();

        // ルートノードを追加
        queue.push_back(&self.nodes[0]);

        // ノードを探索
        while let Some(node) = queue.pop_front()
        {
            // 子ノードを持っていないノードを返す
            if node.left.is_none() || node.right.is_none()
            {
                return Some(node);
            }

            // 子ノードを追加
            if let Some(left) = node.left
            {
                queue.push_back(&self.nodes[left as usize]);
            }

            // 子ノードを追加
            if let Some(right) = node.right
            {
                queue.push_back(&self.nodes[right as usize]);
            }
        }

        // ノードが見つからない場合はNoneを返す
        None
    }

    pub fn add_str(&mut self, str: String)
    {
        let token = Token::new(str);
        self.add_node(token);
    }

    /// 左か右にノードを追加する
    /// 左側が優先,
    /// 両方にノードがある場合は追加しない
    pub fn add_node(&mut self, token: Token)
    {
        // 追加するノードを探す
        let mut node = self.find_empty_node();

        if let Some(mut node) = node {
            if node.left.is_none()
            {
                let mut new_node = TreeNode::new(token);
                new_node.left = Some((self.nodes.len() + 1) as u32);
                self.nodes.push(new_node);
                return;
            }
            else if node.right.is_none()
            {
                let mut new_node = TreeNode::new(token);
                new_node.right = Some((self.nodes.len() + 1) as u32);
                self.nodes.push(new_node);
                return;
            }
        }
    }
}


impl TreeNode
{
    /// ノードを生成する
    pub fn new(value: Token) -> TreeNode
    {
        TreeNode { val: Some(value.clone()), left: None, right: None }
    }

    /// 数値のノードを生成する
    pub fn new_val(value: i32) -> TreeNode
    {
        TreeNode { val: Some(Token::Val(value)), left: None, right: None }
    }

    /// 演算子のノードを生成する
    pub fn new_op(op: String) -> TreeNode
    {
        TreeNode { val: Some(Token::Op(Operand::from_string(op))), left: None, right: None }
    }

    /// 文字列のノードを生成する
    pub fn new_string(str: String) -> TreeNode
    {
        TreeNode { val: Some(Token::new(str)), left: None, right: None }
    }


    /// 空のノードを生成する
    pub fn empty_node() -> TreeNode
    {
        TreeNode { val: None, left: None, right: None }
    }

    /// ノードに値を追加する
    pub fn add_value(&mut self, val: Token)
    {
        self.val = Some(val.clone());
    }
}


// 幅優先探索で構造を表示
#[allow(unreachable_code)]
pub fn show_tree(root: &Tree) -> String
{
    let mut queue = VecDeque::new();
    let mut result = String::from("");

    // ルートノードを追加
    queue.push_back(&root.nodes[0]);

    // ノードを探索
    while let Some(node) = queue.pop_front()
    {
        // ノードの値を表示
        if let Some(val) = &node.val
        {
            result.push_str(&format!("{}, ", val));
        }

        // 子ノードを追加
        if let Some(left) = node.left
        {
            queue.push_back(&root.nodes[left as usize]);
        }

        // 子ノードを追加
        if let Some(right) = node.right
        {
            queue.push_back(&root.nodes[right as usize]);
        }
    }

    return result;
}