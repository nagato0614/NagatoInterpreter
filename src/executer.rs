use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;
use anyhow::{bail, ensure, Context, Result};
use std::io::{BufRead, BufReader, stdin};
use std::fs::File;
use std::ptr::{null, swap};
use std::rc::Rc;
use crate::tree::{create_node, Operand, Token, TreeNode};

#[derive(Debug, Default)]
pub struct NagatoLang
{
    variable: HashMap<String, i32>,
}


impl NagatoLang
{
    pub fn new() -> NagatoLang
    {
        NagatoLang { variable: Default::default() }
    }

    pub fn add_variable(&mut self, name: String, value: i32)
    {
        self.variable.insert(name, value);
    }

    pub fn get_variable_value(&self, name: String) -> Result<i32>
    {
        let ret = self.variable.get(name.as_str());

        Ok(ret.unwrap().clone())
    }
}

pub fn lexical_analysis()
{}

pub fn semantic_analysis()
{}

pub fn syntax_analysis(line: String) -> TreeNode
{
    let mut root ;
    let mut split_line = line.split_whitespace();

    if let Some(token) = split_line.next()
    {
        root = create_node(token.to_string());
    }
    else
    {
        return TreeNode::empty_node();
    }

    loop
    {
        // todo: アーリーリターンがしたい
        if let Some(token) = split_line.next()
        {
            if let Token::Op(operand) = Token::new(token.to_string())
            {
                root.borrow_mut().add_value(Token::Op(operand));
            }
            else if let Token::Val(value) = Token::new(token.to_string())
            {
                root.borrow_mut().get_mut().val = Some(Token::Val(value))
            }
            else
            {
                eprintln!("this is not operand or value : {}", token.to_string());
                panic!();
            }
        }
        else
        {
            return root.take();
        }


    }
    TreeNode::empty_node()
}

pub fn get_number(token: String) -> Result<i32, String>
{
    if let Ok(number) = token.parse::<i32>()
    {
        Ok(number)
    } else {
        Err("it is not number".to_string())
    }
}

pub fn run(reader: BufReader<File>) -> Result<()>
{
    Ok(())
}



