use std::cell::RefCell;
use std::collections::HashMap;
use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{CallSiteValue, FunctionValue, InstructionValue, PointerValue};
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

#[derive(Debug, Clone)]
struct LocalVariable<'ctx>
{
    name: String,
    pointer: PointerValue<'ctx>,
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

    // ローカル変数
    local_vars: HashMap<String, PointerValue<'ctx>>,

    // 関数一覧
    functions: HashMap<String, FunctionValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx>
{
    pub fn new(
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
        execution_engine: ExecutionEngine<'ctx>,
    ) -> Self {
        let codegen = CodeGen {
            context,
            module,
            builder,
            execution_engine,
            global_vars: HashMap::new(),
            local_vars: HashMap::new(),
            functions: HashMap::new(),
        };

        codegen
    }

    // root を読み込んで、LLVM IR を生成する
    pub fn generate(&mut self, root: &Rc<RefCell<Node>>) -> Result<(), Box<dyn Error>> {
        println!("## generate");
        if let Some(val) = root.borrow().val() {
            match val
            {
                Leaf::Declaration(_) =>
                    {
                        self.declare_global_variable(root);
                    }
                Leaf::FunctionDefinition(_) =>
                    {
                        self.function_definition(root);
                    }
                _ =>
                    {
                        return Err("対応していないノードです".into());
                    }
            }

            Ok(())
        } else {
            Err("ノードが見つかりません".into())
        }
    }

    fn function_definition(&mut self, node: &Rc<RefCell<Node>>) {
        println!("## function_definition");
        let function_name = self.get_function_name(node);
        let function_type = self.get_function_type(node);
        let function_body = self.get_function_body(node);

        // 関数を定義
        let function = self.define_function(&function_name, &function_type);

        // 関数の本体をコンパイル
        if function_body.len() == 0 {
            // 関数の本体がない場合は、適当な値を返す
            match function_type {
                ValueType::Void => {
                    // None なので型を指定する必要がないが, 型推論を解決できないため明示的に指定
                    self.add_ret::<inkwell::values::IntValue<'ctx>>(None);
                }
                ValueType::Int => {
                    self.add_ret(Some(self.context.i32_type().const_int(0, false)));
                }
                ValueType::Float => {
                    self.add_ret(Some(self.context.f32_type().const_float(0.0)));
                }
                _ => panic!("未対応の関数型です"),
            }
        } else {
            for node in function_body {
                self.compound_statement(&node);
            }
        }
    }

    fn compound_statement(&mut self, node: &Rc<RefCell<Node>>) {
        println!("## compound_statement");
        // compound は関す内部でしか呼ばれないため、ローカル変数のみを扱う
        if let Some(val) = node.borrow().val() {
            match val {
                Leaf::Declaration(_) => {
                    self.declare_local_variable(node);
                }
                Leaf::Return => {
                    self.return_statement(node);
                }
                _ => {
                    panic!("未対応のノードです");
                }
            }
        }
    }


    fn compile_node(&mut self, node: Rc<RefCell<Node>>) {
        println!("## compile_node");

        if let Some(val) = node.borrow().val() {
            match val {
                Leaf::Return => {
                    self.return_statement(&node);
                }
                Leaf::Declaration(_) => {
                    self.declare_global_variable(&node);
                }
                _ => {
                    panic!("未対応のノードです");
                }
            }
        }
    }

    fn declare_local_variable(&mut self, node: &Rc<RefCell<Node>>)
    {
        
    }

    /// return 文を処理
    /// TODO : 戻り値を定数以外取り扱えるようにする
    fn return_statement(&self, node: &Rc<RefCell<Node>>)
    {
        if let Some(constant_value) = self.get_constant_value(node.borrow().lhs().unwrap()) {
            match constant_value {
                VariableValue::Int(value) => {
                    self.add_ret(Some(self.context.i32_type().const_int(value as u64, false)));
                }
                VariableValue::Float(value) => {
                    self.add_ret(Some(self.context.f32_type().const_float(value as f64)));
                }
            }
        } else {
            // 戻り値がない場合
            self.add_ret::<inkwell::values::IntValue<'ctx>>(None)
                .expect("return 文の追加に失敗しました");
        }
    }

    fn add_ret<T: inkwell::values::BasicValue<'ctx>>(&self, value: Option<T>)
                                                     -> Result<InstructionValue<'ctx>, BuilderError>
    {
        match value {
            Some(v) => self.builder.build_return(Some(&v)),
            None => self.builder.build_return(None),
        }
    }

    fn get_function_body(&self, node: &Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>> {
        if let Some(val) = node.borrow().val() {
            if let Leaf::FunctionDefinition(func) = val {
                return func.body().clone();
            }
        }

        panic!("関数の本体が取得できませんでした");
    }

    fn define_function(&self, function_name: &str, function_type: &ValueType) -> FunctionValue {
        match function_type {
            ValueType::Void => self.define_function_void(function_name),
            ValueType::Int => self.define_function_int(function_name),
            _ => panic!("未対応の関数型です"),
        }
    }
    fn define_function_void(&self, function_name: &str) -> FunctionValue
    {
        let void_type = self.context.void_type();
        let function_type = void_type.fn_type(&[], false);
        let function = self.module.add_function(function_name, function_type, None);


        // 関数が main の場合は、エントリーポイントを設定
        if function_name == "main" {
            let basic_block = self.context.append_basic_block(function, "entry");
            self.builder.position_at_end(basic_block);
        }

        function
    }

    fn define_function_int(&self, function_name: &str) -> FunctionValue
    {
        let int_type = self.context.i32_type();
        let function_type = int_type.fn_type(&[], false);
        let function = self.module.add_function(function_name, function_type, None);

        // 関数が main の場合は、エントリーポイントを設定
        if function_name == "main" {
            let basic_block = self.context.append_basic_block(function, "entry");
            self.builder.position_at_end(basic_block);
        }

        function
    }


    fn get_function_name(&self, node: &Rc<RefCell<Node>>) -> String
    {
        if let Some(val) = node.borrow().val() {
            if let Leaf::FunctionDefinition(func) = val {
                return func.name().clone();
            }
        }

        panic!("関数名が取得できませんでした");
    }

    fn get_function_type(&self, node: &Rc<RefCell<Node>>) -> ValueType
    {
        if let Some(val) = node.borrow().val() {
            if let Leaf::FunctionDefinition(func) = val {
                return func.type_specifier().clone();
            }
        }

        panic!("関数の型が取得できませませんでした");
    }

    /// ValueType に応じた LLVM 型を返す
    fn get_basic_type(&self, value_type: &ValueType) -> AnyTypeEnum<'ctx> {
        match value_type {
            ValueType::Void => self.context.void_type().as_any_type_enum(),
            ValueType::Int => self.context.i32_type().as_any_type_enum(),
            ValueType::Float => self.context.f32_type().as_any_type_enum(),
            ValueType::Struct(_) => {
                panic!("LLVM IR では構造体は未対応です");
            }
            ValueType::Array(_, _) => {
                panic!("LLVM IR では配列は未対応です");
            }
        }
    }

    fn assign_value(&mut self, identifier: String, value: VariableValue)
    {
        let variable = self.local_vars.get(&identifier).unwrap();

        match value {
            VariableValue::Int(value) => {
                self.builder.build_store(*variable, self.context.i32_type().const_int(value as u64, false));
            }
            VariableValue::Float(value) => {
                self.builder.build_store(*variable, self.context.f32_type().const_float(value as f64));
            }
        }
    }

    fn define_local_variable(&mut self,
                             value_type: ValueType,
                             identifier: &str,
                             value: VariableValue)
    {
        println!("## define_local_variable");

        unimplemented!("define_local_variable");
    }

    fn declare_global_variable(&mut self, node: &Rc<RefCell<Node>>) {
        println!("## declare_global_variable");
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
        self.define_global_variable(value_type, &identifier, value);
    }

    fn define_global_variable(&mut self,
                              value_type: ValueType,
                              identifier: &str,
                              value: VariableValue) {
        match value_type {
            ValueType::Int => {
                match value {
                    VariableValue::Int(value) => self.add_global_int(identifier, value),
                    VariableValue::Float(value) => self.add_global_int(identifier, value as i32),
                }
            }
            ValueType::Float => {
                match value {
                    VariableValue::Int(value) => self.add_global_float(identifier, value as f32),
                    VariableValue::Float(value) => self.add_global_float(identifier, value),
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

    // ----------------------------------------------------------------------------------------
    // デバッグ用関数群

    pub fn define_printf(&mut self)
    {
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::from(0));
        let printf_type = i8_ptr_type.fn_type(&[i8_ptr_type.into()], /* is_var_arg */ true);
        let printf = self.module.add_function("printf", printf_type, None);

        // 関数を追加
        self.functions.insert("printf".to_string(), printf);
    }

    pub fn call_printf(&self, text: &str) -> Result<CallSiteValue<'ctx>, BuilderError>
    {
        let printf = self.functions.get("printf").unwrap();
        let hello_str = self.builder.build_global_string_ptr(text, "hello_str");

        // printf 呼び出し
        self.builder.build_call(
            *printf,
            &[hello_str.expect("REASON").as_pointer_value().into()],
            "printf_call",
        )
    }
}


pub fn compile(roots: &Vec<Rc<RefCell<Node>>>) -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

    let mut codegen = CodeGen::new(&context, module, builder, execution_engine);
    codegen.define_printf();

    println!("## コンパイル開始");

    for root in roots {
        codegen.generate(root)?;
    }


    // 中間コードを標準出力に出力
    codegen.module.print_to_stderr();

    codegen.write_bitcode("output.bc");

    Ok(())
}
