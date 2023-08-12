use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell};
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

impl Display for Token
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
    pub nodes: RefCell<Vec<RefCell<TreeNode>>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TreeNode
{
    pub val: Option<Token>,
    pub left: Cell<Option<usize>>,
    pub right: Cell<Option<usize>>,
}

impl Tree
{
    pub fn new(token: Token) -> Tree
    {
        let mut tree = Tree::default();
        let root = TreeNode::new(token);
        tree.nodes.borrow_mut().push(RefCell::new(root));
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
    /// 戻り地はノードのインデックス
    pub fn find_empty_node(&self) -> Option<usize>
    {
        // ツリーがからの場合はNoneを返す
        if self.nodes.borrow().len() == 0
        {
            return None;
        }

        // ルートノードから探索を開始する
        let mut queue = VecDeque::new();

        // ルートノードを追加
        queue.push_back(0usize);

        // キューが空になるまで探索を続ける
        while let Some(node_index) = queue.pop_front()
        {
            // ノードを取得
            let node = &self.nodes.borrow()[node_index as usize];

            // 左右のノードを取得
            let left = node.borrow().left.get();
            let right = node.borrow().right.get();

            // 左右のノードがない場合はそのノードを返す
            if left.is_none() || right.is_none()
            {
                return Some(node_index);
            }

            // 左右のノードがある場合はキューに追加
            queue.push_back(left.unwrap());
            queue.push_back(right.unwrap());
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
    pub fn add_node(&mut self, token: Token) {
        // 追加するノードを探す
        if let Some(node_index) = self.find_empty_node() {
            let mut nodes = &mut self.nodes.borrow_mut();
            let mut node = &mut nodes[node_index].borrow_mut();

            if node.left.get().is_none() {
                // 左側にノードがない場合は左側に追加
                let new_node_index = nodes.len();
                node.left.set(Some(new_node_index));
                nodes.push(RefCell::new(TreeNode::new(token)));
            } else if node.right.get().is_none() {

                // 右側にノードがない場合は右側に追加
                let new_node_index = nodes.len();
                node.right.set(Some(new_node_index));
                nodes.push(RefCell::new(TreeNode::new(token)));
            }
        }
    }
}


impl TreeNode
{
    /// ノードを生成する
    pub fn new(value: Token) -> TreeNode
    {
        TreeNode {
            val: Some(value.clone()),
            left: Cell::new(None),
            right:
            Cell::new(None)
        }
    }

    /// 数値のノードを生成する
    pub fn new_val(value: i32) -> TreeNode
    {
        TreeNode { val: Some(Token::Val(value)), left: Cell::new(None), right: Cell::new(None) }
    }

    /// 演算子のノードを生成する
    pub fn new_op(op: String) -> TreeNode
    {
        TreeNode { val: Some(Token::Op(Operand::from_string(op))), left: Cell::new(None), right: Cell::new(None) }
    }

    /// 文字列のノードを生成する
    pub fn new_string(str: String) -> TreeNode
    {
        TreeNode { val: Some(Token::new(str)), left: Cell::new(None), right: Cell::new(None) }
    }


    /// 空のノードを生成する
    pub fn empty_node() -> TreeNode
    {
        TreeNode { val: None, left: Cell::new(None), right: Cell::new(None) }
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

    let nodes = root.nodes.borrow();

    // ノードのインデックスを追加
    queue.push_back(0usize);

    // ノードを探索
    while let Some(node_index) = queue.pop_front()
    {

        // ノードを取得
        let node = nodes[node_index].borrow();

        // ノードの値を取得
        let val = node.val.clone().unwrap();

        // ノードの値を表示
        result.push_str(&format!("{}, ", val));


        // 左右のノードがある場合はキューに追加
        if let Some(left) = node.left.get()
        {
            queue.push_back(left);
        }
        if let Some(right) = node.right.get()
        {
            queue.push_back(right);
        }
    }


    return result;
}