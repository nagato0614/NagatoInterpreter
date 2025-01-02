use std::cell::RefCell;
use std::collections::HashMap;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use inkwell::OptimizationLevel;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::types::{AnyType, AnyTypeEnum, BasicType, BasicTypeEnum};
use crate::parser::{Leaf, Node};
use inkwell::values::GlobalValue;
use crate::lexical::{Constant, ValueType};


#[derive(Debug, Clone)]
pub enum VariableValue
{
    Int(i32),
    Float(f32),
}

#[derive(Debug, Clone)]
struct GlobalVariable<'ctx>
{
    name: String,
    value: VariableValue,
    global_variable: GlobalValue<'ctx>,
}

impl<'ctx> GlobalVariable<'ctx>
{
    pub fn new(name: String, value: VariableValue, global_variable: GlobalValue<'ctx>) -> Self {
        GlobalVariable {
            name,
            value,
            global_variable,
        }
    }
}

#[derive(Debug)]
struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,

    // グローバル変数
    global_vars: HashMap<String, GlobalVariable<'ctx>>,
}

impl<'ctx> CodeGen<'ctx>
{
    pub fn new(
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
        execution_engine: ExecutionEngine<'ctx>,
    ) -> Self {
        CodeGen {
            context,
            module,
            builder,
            execution_engine,
            global_vars: HashMap::new(),
        }
    }

    // root を読み込んで、LLVM IR を生成する
    pub fn generate(&mut self, root: &Rc<RefCell<Node>>) -> Result<(), Box<dyn Error>> {
        if let Some(val) = root.borrow().val() {
            match val
            {
                Leaf::Declaration(_) =>
                    {
                        self.declare_variable(root);
                    }
                _ => {
                    return Err("対応していないノードです".into());
                }
            }

            Ok(())
        } else {
            Err("ノードが見つかりません".into())
        }
    }

    /// ValueType に応じた LLVM 型を返す
    fn get_basic_type(&self, value_type: &ValueType) -> AnyTypeEnum<'ctx> {
        match value_type {
            ValueType::Void => self.context.void_type().as_any_type_enum(),
            ValueType::Int => self.context.i32_type().as_any_type_enum(),
            ValueType::Float => self.context.f32_type().as_any_type_enum(),
        }
    }

    fn declare_variable(&mut self, node: &Rc<RefCell<Node>>) {
        let value_type = self.get_variable_type(node);

        // 左辺値から識別子を取得
        let mut identifier = String::new();
        if let Some(lhs) = node.borrow().lhs() {
            identifier = self.get_identifier(lhs);
        }

        // 右辺値がある場合は、その値を取得
        // TODO : 初期値設定時に演算を行えるようにする
        let value = if let Some(rhs) = node.borrow().rhs() {
            self.get_constant_value(rhs).unwrap()
        } else {
            // 見つからない場合は、初期値を設定
            match value_type {
                ValueType::Int => VariableValue::Int(0),
                ValueType::Float => VariableValue::Float(0.0),
                _ => panic!("未対応の型です"),
            }
        };

        // LLVM IR でグローバル変数を定義
        match value_type {
            ValueType::Int =>
                {
                    match value
                    {
                        VariableValue::Int(value) => self.add_global_int(&identifier, value),
                        VariableValue::Float(value) => self.add_global_int(&identifier, value as i32),
                    }
                }
            ValueType::Float =>
                {
                    match value
                    {
                        VariableValue::Int(value) => self.add_global_float(&identifier, value as f32),
                        VariableValue::Float(value) => self.add_global_float(&identifier, value),
                    }
                }
            _ => panic!("未対応の型です"),
        }
    }

    fn get_identifier(&self, node: &Rc<RefCell<Node>>) -> String {
        if let Some(val) = node.borrow().val() {
            if let Leaf::Identifier(identifier) = val {
                return identifier.clone();
            }
        }

        panic!("識別子が取得できませんでした");
    }

    fn add_global_int(&mut self, name: &str, value: i32) {
        let int_type = self.context.i32_type();
        let global = self.module.add_global(int_type, None, name);
        let const_value = int_type.const_int(value as u64, false);
        global.set_initializer(&const_value);

        let global_var =
            GlobalVariable::new(name.to_string(), VariableValue::Int(value), global);
        self.global_vars.insert(name.to_string(), global_var);
    }

    fn add_global_float(&mut self, name: &str, value: f32) {
        let float_type = self.context.f32_type();
        let global = self.module.add_global(float_type, None, name);
        let const_value = float_type.const_float(value as f64);
        global.set_initializer(&const_value);
        
        let global_var =
            GlobalVariable::new(name.to_string(), VariableValue::Float(value), global);
        self.global_vars.insert(name.to_string(), global_var);
    }

    fn get_constant_value(&self, node: &Rc<RefCell<Node>>) -> Option<VariableValue>
    {
        if let Some(val) = node.borrow().val() {
            if let Leaf::Constant(constant) = val {
                match constant {
                    Constant::Integer(value) => return Some(VariableValue::Int(*value)),
                    Constant::Float(value) => return Some(VariableValue::Float(*value)),
                }
            }
        }

        None
    }

    fn get_variable_type(&self, node: &Rc<RefCell<Node>>) -> ValueType {
        if let Some(val) = node.borrow().val() {
            if let Leaf::Declaration(decl_type) = val {
                return decl_type.clone();
            }
        }

        panic!("変数の型が取得できませんでした");
    }

    // ビットコードをファイルに書き出す
    pub fn write_bitcode(&self, path: &str) {
        self.module.write_bitcode_to_path(Path::new(path));
    }
}


pub fn compile(roots: &Vec<Rc<RefCell<Node>>>) -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

    let mut codegen = CodeGen::new(&context, module, builder, execution_engine);

    for root in roots {
        codegen.generate(root)?;
    }

    // 中間コードを標準出力に出力
    println!("----------------------");
    codegen.module.print_to_stderr();
    println!("----------------------");

    codegen.write_bitcode("output.bc");

    Ok(())
}