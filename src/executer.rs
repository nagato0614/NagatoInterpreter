use std::borrow::Borrow;
use std::collections::HashMap;
use anyhow::Result;
use std::io::BufReader;
use std::fs::File;
use crate::tree::{Tree, Token};

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

pub fn syntax_analysis(line: String) -> Tree
{
    let mut root ;
    let mut split_line = line.split_whitespace();

    // ルートノードの作成
    if let Some(token) = split_line.next()
    {
        // トークンがある場合はトークンをルートノードに設定
        root = Tree::new(Token::new(token.to_string()));
    }
    else
    {
        // トークンがない場合は空のノードを返す
        return Tree::empty_node();
    }

    // ルートノードの子ノードの作成
    loop
    {

        // トークンがある場合はトークンをルートノードに設定
        if let Some(token) = split_line.next()
        {
            // トークンが演算子か値かを判定
            if let Token::Op(operand) = Token::new(token.to_string())
            {
                root.add_value(Token::Op(operand));
            }
            else if let Token::Val(value) = Token::new(token.to_string())
            {
                root.add_value(Token::Val(value));
            }
            else
            {
                // トークンがない場合は空のノードを返す
                eprintln!("this is not operand or value : {}", token.to_string());
                panic!();
            }
        }
        else
        {
            // トークンがない場合は空のノードを返す
            return root;
        }


    }
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



