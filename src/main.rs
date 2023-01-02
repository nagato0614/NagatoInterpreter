use MylangExecuter::executer::NagatoLang;
use MylangExecuter::tree::TreeNode;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::path::PathBuf;
use anyhow::{bail, ensure, Context, Result};

#[derive(Parser, Debug, Default)]
#[clap(
name = "My Lang Execute",
version = "0.0.1",
author = "nagai toru",
about = "my lang parser"
)]
pub struct InputParser
{
    #[clap(short, long)]
    pub debug: bool,

    #[clap(name = "program.txt")]
    pub formula_file: Option<PathBuf>,
}


fn main() {
    let parser = InputParser::parse();
    let mut exec = NagatoLang::new();
    if let Some(path) = parser.formula_file
    {
        println!("file path : {:?}", path);
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        match exec.run(reader)
        {
            Ok(_) => println!("Finish Program"),
            Err(e) => println!("Error Program : {:?}", e),
        }

    }
    else
    {
        println!("can't find file");
    }
}


#[cfg(test)]
mod tests
{
    use std::borrow::Borrow;
    use std::rc::Rc;
    use MylangExecuter::tree::{create_node, show_tree};
    use super::*;

    // 変数の格納取り出し用テスト
    #[test]
    fn variable_test()
    {
        let mut exec = NagatoLang::new();
        exec.add_variable("a".to_string(), 1);
        exec.add_variable("b".to_string(), 2);
        assert_eq!(exec.get_variable_value("a".to_string()).unwrap(), 1);
        assert_eq!(exec.get_variable_value("b".to_string()).unwrap(), 2);

        exec.add_variable("a".to_string(), 5);
        assert_eq!(exec.get_variable_value("a".to_string()).unwrap(), 5);
    }

    #[test]
    fn parse_test()
    {

    }

    #[test]
    fn tree_test()
    {
        let mut root = create_node("1".to_string());
        root.borrow_mut().add_right_node(TreeNode::new("2".to_string()));
        root.borrow_mut().add_left_node(TreeNode::new("3".to_string()));
        println!("show");
    }
}