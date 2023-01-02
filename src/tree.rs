use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use anyhow::{bail, ensure, Context, Result};
use std::io::{BufRead, BufReader, stdin};
use std::fs::File;
use std::rc::Rc;

// TODO: 値を可変にできるようにする.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TreeNode
{
    pub val: Option<String>,
    pub left: Option<Rc<TreeNode>>,
    pub right: Option<Rc<TreeNode>>,
}

impl TreeNode
{
    pub fn new(value: String) -> TreeNode
    {
        TreeNode { val: Some(value.clone()), left: None, right: None }
    }

    pub fn add_value(&mut self, val: &String)
    {
        self.val = Some(val.clone());
    }

    pub fn add_left_node(&mut self, left: TreeNode) -> &mut TreeNode
    {
        self.left = Some(Rc::new(left));
        self
    }

    pub fn add_right_node(&mut self, right: TreeNode) -> &mut TreeNode
    {
        self.right = Some(Rc::new(right));
        self
    }
}

pub fn create_node(val: String) -> Rc<TreeNode>
{
    Rc::new(TreeNode::new(val.clone()))
}

// 幅優先探索で構造を表示
pub fn show_tree(root: &Rc<TreeNode>)
{
    let mut v: VecDeque<&Rc<TreeNode>> = VecDeque::new();
    v.push_back(root);
    while true
    {
        let Some(n) = v.pop_front() else { break; };

        if let Some(value) = &n.val
        {
            print!("{}, ", value);
        } else {
            print!(" , ");
        }

        if let Some(node) = &n.left
        {
            v.push_back(&node);
        }

        if let Some(node) = &n.right
        {
            v.push_back(&node);
        }
    }
}