use MylangExecuter::executer::NagatoLang;
use MylangExecuter::tree::TreeNode;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::path::PathBuf;
use anyhow::{bail, ensure, Context, Result};
use MylangExecuter::executer;

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
        match executer::run(reader)
        {
            Ok(_) => println!("Finish Program"),
            Err(e) => println!("Error Program : {:?}", e),
        }
    } else {
        println!("can't find file");
    }
}


#[cfg(test)]
mod tests
{
    use std::borrow::Borrow;
    use std::rc::Rc;
    use clap::builder::Str;
    use clap::error::ContextValue::String;
    use MylangExecuter::executer::{NagatoLang, syntax_analysis};
    use MylangExecuter::tree::{show_tree, Token, Tree};
    use super::*;

    // 変数の格納取り出し用テスト
    #[test]
    fn variable_test()
    {
        let mut exec: NagatoLang = NagatoLang::new();
        exec.add_variable("a".to_string(), 1);
        exec.add_variable("b".to_string(), 2);
        assert_eq!(exec.get_variable_value("a".to_string()).unwrap(), 1);
        assert_eq!(exec.get_variable_value("b".to_string()).unwrap(), 2);

        exec.add_variable("a".to_string(), 5);
        assert_eq!(exec.get_variable_value("a".to_string()).unwrap(), 5);
    }

    #[test]
    fn parse_test_ok()
    {
        let test_line = vec![
            "".to_string(),
            "1".to_string(),
            "1 + 1".to_string(),
            "a = 1 + 1".to_string(),
            "a = 1 + 1 + 1".to_string(),
        ];

        let ans_line = vec![
            "".to_string(),
            "1, ".to_string(),
            "+, 1, 1, ".to_string(),
            "=, a, +, 1, 1, ".to_string(),
            "=, a, +, 1, +, 1, 1".to_string(),
        ];

        for (count, (line, ans))
        in (0_i32..).zip(test_line.iter().zip(ans_line.iter()))
        {
            println!("-------------------------");
            println!("test{}", count);
            let root = syntax_analysis(line.clone());
            let str = show_tree(&root);
            assert_eq!(str.clone(), ans.clone());
            println!("OK!");
        }
    }

    #[test]
    #[should_panic]
    fn parse_test_ng()
    {
        let test_line = vec![
            "#".to_string(),
            "1 1".to_string(),
        ];

        for (count, (line)) in (0_i32..).zip(test_line.iter())
        {
            println!("-------------------------");
            println!("test{}", count);
            panic!();
        }
    }

    #[test]
    fn tree_test()
    {
        let token = Token::new("0".to_string());
        let mut root = Tree::new(token);
        let str = show_tree(&root);
        println!("{}", str);
        assert_eq!(str, "0, ".to_string());


        // ２分木の生成
        //         0
        //     1       2
        //   3   4   5   6
        //  + - * /
        root.add_str("1".to_string());
        root.add_str("2".to_string());
        root.add_str("3".to_string());
        root.add_str("4".to_string());
        root.add_str("5".to_string());
        root.add_str("6".to_string());
        root.add_str("+".to_string());
        root.add_str("-".to_string());
        root.add_str("*".to_string());
        root.add_str("/".to_string());


        // 深さ優先探索
        let str = show_tree(&root);
        println!("{}", str);
        assert_eq!(str, "0, 1, 2, 3, 4, 5, 6, +, -, *, /, ");
    }
}