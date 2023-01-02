use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt::Display;
use anyhow::{bail, ensure, Context, Result};
use std::io::{BufRead, BufReader, stdin};
use std::fs::File;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TreeNode
{
    pub val: Option<String>,
    pub left:  Option<Rc<RefCell<TreeNode>>>,
    pub right:  Option<Rc<RefCell<TreeNode>>>,
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
    Rc::new(RefCell::new(TreeNode::new(val.clone())))
}

// 幅優先探索で構造を表示
pub fn show_tree(root: &Rc<RefCell<TreeNode>>)
{
    let mut v = Vec::new();
    v.push(root);
    for n in v.pop()
    {
        if let Some(value) = &n.borrow_mut().val
        {
            print!("{}, ", value);
        }
        else
        {
            print!(" , ");
        }

        if let Some(node) = n.borrow().left
        {
            v.push(&node);
        }

        if let Some(node) = n.borrow().right
        {
            v.push(&node);
        }
    }
}