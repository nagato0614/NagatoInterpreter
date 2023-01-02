use std::collections::HashMap;
use anyhow::{bail, ensure, Context, Result};
use std::io::{BufRead, BufReader, stdin};
use std::fs::File;

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

    fn lexical_analysis(&self)
    {

    }

    fn syntax_analysis(&self)
    {

    }

    fn semantic_analysis(&self)
    {

    }

    pub fn run(&self, reader: BufReader<File>) -> Result<()>
    {
        Ok(())
    }

}


