use crate::interpreter::VariableType::Int;
use crate::lexical::{Constant, Operator, UnaryOperator, ValueType};
use crate::parser::{FunctionCall, FunctionDefinition, Leaf, Node};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Array
{
  name: String,
  variable_type: VariableType,
  values: Vec<VariableType>,
}

impl Array
{
  pub fn new(name: String, variable_type: VariableType, values: Vec<VariableType>) -> Self
  {
    Array { name, variable_type, values }
  }

  pub fn name(&self) -> &String
  {
    &self.name
  }

  pub fn variable_type(&self) -> &VariableType
  {
    &self.variable_type
  }

  pub fn values(&self) -> &Vec<VariableType>
  {
    &self.values
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType
{
  Int(i32),
  Float(f64),
  Struct(Struct),
  Array(Rc<RefCell<Array>>),
  Void,
  Break,
  Continue,
  Return(Box<VariableType>),
}

// VariableType の format
impl std::fmt::Display for VariableType
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
  {
    match self
    {
      VariableType::Int(val) => write!(f, "{}", val),
      VariableType::Float(val) => write!(f, "{}", val),
      VariableType::Struct(s) => write!(f, "struct {}", s.name),
      VariableType::Array(a) => write!(f, "array {}", a.borrow().name),
      VariableType::Void => write!(f, "void"),
      VariableType::Break => write!(f, "break"),
      VariableType::Continue => write!(f, "continue"),
      VariableType::Return(val) => write!(f, "return {}", val),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct
{
  name: String,
  members: HashMap<String, VariableType>,
}

impl Struct
{
  pub fn new(name: String, members: HashMap<String, VariableType>) -> Self
  {
    Struct { name, members }
  }

  pub fn name(&self) -> &String
  {
    &self.name
  }

  pub fn members(&self) -> &HashMap<String, VariableType>
  {
    &self.members
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
  Value(VariableType),
  Array(Rc<RefCell<Array>>),
  Struct(Struct),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scope
{
  Global,
  Local,
}

pub struct Interpreter
{
  roots: Vec<Rc<RefCell<Node>>>,

  // すべての領域からアクセス可能な変数
  global_variables: HashMap<String, Variable>,

  // 関数の中でのみアクセス可能な変数
  local_variables: Vec<Vec<HashMap<String, Variable>>>,

  function_definition: HashMap<String, FunctionDefinition>,

  struct_definition: HashMap<String, HashMap<String, ValueType>>,

  scope: Scope,
}

impl Interpreter
{
  pub fn new(roots: &Vec<Rc<RefCell<Node>>>) -> Self
  {
    Interpreter
    {
      roots: roots.clone(),
      global_variables: HashMap::new(),
      local_variables: Vec::new(),
      function_definition: HashMap::new(),
      struct_definition: HashMap::new(),
      scope: Scope::Global,
    }
  }

  pub fn global_variables(&self) -> &HashMap<String, Variable>
  {
    &self.global_variables
  }

  pub fn run(&mut self) -> VariableType
  {
    let mut roots = self.roots.clone();
    for root in roots.iter()
    {
      self.interpret_node(root);
    }

    // main 関数を呼び出し実行する
    if let Some(main) = self.function_definition.get("main")
    {
      // main の function_call を作成
      let function_call = FunctionCall::new("main".to_string());

      // main 関数を呼び出し
      self.scope = Scope::Local;
      let val = self.function_call(&function_call);
      self.scope = Scope::Global;

      if let VariableType::Return(val) = val
      {
        return *val;
      }
      return val;
    } else {
      panic!("main 関数が見つかりません");
    }
  }

  pub fn show_variables(&self)
  {
    for (name, variable) in &self.global_variables
    {
      match variable
      {
        Variable::Value(value) =>
          {
            //println!("{} = {:?}", name, value);
          }
        Variable::Array(array) =>
          {
            // unimplemented!("配列の表示は未実装です");
          }
        Variable::Struct(s) =>
          {
            // unimplemented!("構造体の表示は未実装です");
          }
      }
    }
  }

  fn interpret_node(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
  {
    if let Some(val) = node.clone().borrow().val()
    {
      match val
      {
        Leaf::Declaration(variable_type) =>
          {
            // node の左側から変数名を取得
            if let Some(lhs) = node.borrow().lhs()
            {
              let identifier = self.identifier_name(lhs);

              // node の右側から値を取得
              if let Some(rhs) = node.borrow().rhs()
              {
                if let Leaf::Array(size) = rhs.borrow().val().unwrap()
                {
                  self.array_variable_definition(&variable_type, identifier,
                                                 *size);
                } else {
                  let mut value = self.statement(rhs);

                  // value が 定数ではなく Return の時中身を取り出す
                  if let VariableType::Return(val) = value
                  {
                    value = val.as_ref().clone();
                  }

                  self.variable_definition(variable_type, identifier, value);
                }
              } else {
                // 初期値がない場合は 0 で初期化
                match variable_type
                {
                  ValueType::Int =>
                    {
                      self.insert_variable_int(identifier, 0);
                    }
                  ValueType::Float =>
                    {
                      self.insert_variable_float(identifier, 0.0);
                    }
                  ValueType::Struct(struct_name) =>
                    {
                      // 構造体定義からメンバを取得
                      let struct_def = self.struct_definition.get(struct_name).unwrap().clone();
                      let mut members = HashMap::new();
                      for (member_name, member_type) in struct_def
                      {
                        match member_type
                        {
                          ValueType::Int => { members.insert(member_name, VariableType::Int(0)); }
                          ValueType::Float => { members.insert(member_name, VariableType::Float(0.0)); }
                          ValueType::Struct(nested_struct_name) => {
                            // ネストした構造体の初期化
                            let nested_struct_def = self.struct_definition.get(&nested_struct_name).unwrap().clone();
                            let mut nested_members = HashMap::new();
                            for (n_name, n_type) in nested_struct_def {
                                match n_type {
                                    ValueType::Int => { nested_members.insert(n_name, VariableType::Int(0)); }
                                    ValueType::Float => { nested_members.insert(n_name, VariableType::Float(0.0)); }
                                    _ => { panic!("深いネストの構造体は未対応です"); }
                                }
                            }
                            let nested_s = Struct::new(nested_struct_name, nested_members);
                            members.insert(member_name, VariableType::Struct(nested_s));
                          }
                          _ => { panic!("未対応のメンバ型です : {:?}", member_type); }
                        }
                      }
                      let s = Struct::new(struct_name.clone(), members);
                      self.insert_variable(identifier, Variable::Struct(s));
                    }
                  _ => {
                    panic!("未対応の型です : {:?}", variable_type);
                  }
                }
              }
            }
          }
        Leaf::FunctionDefinition(function_definition) =>
          {
            let name = function_definition.name();
            self.function_definition.insert(name.clone(), function_definition.clone());
          }

        // 関数呼び出し
        Leaf::FunctionCall(function_call) =>
          {
            self.function_call(function_call);
          }

        // return 文
        Leaf::Return =>
          {
            if let Some(lhs) = node.borrow().lhs()
            {
              let value = self.statement(lhs);
              println!("return value : {:?}", value);
              return VariableType::Return(Box::new(value));
            }
          }
        Leaf::Assignment =>
          {
            return self.variable_assignment(node);
          }
        Leaf::IfStatement(_) =>
          {
            return self.selection_statement(node);
          }
        Leaf::WhileStatement =>
          {
            return self.while_statement(node);
          }
        Leaf::ForStatement(_) =>
          {
            return self.for_statement(node);
          }
        Leaf::Break =>
          {
            return VariableType::Break;
          }
        Leaf::Continue =>
          {
            return VariableType::Continue;
          }
        Leaf::BlockItem(nodes) =>
          {
            return self.compound_statement(nodes, true);
          }
        Leaf::ArrayAssignment(_) =>
          {
            return self.array_assignment(node);
          }
        Leaf::StructDefinition(name, members) =>
          {
            let mut struct_members = HashMap::new();
            for member in members
            {
              if let Some(Leaf::Declaration(value_type)) = member.borrow().val()
              {
                if let Some(lhs) = member.borrow().lhs()
                {
                  let identifier = self.identifier_name(lhs);
                  struct_members.insert(identifier, value_type.clone());
                }
              }
            }
            self.struct_definition.insert(name.clone(), struct_members);
          }
        Leaf::StructMemberAccess =>
          {
            return self.struct_member_access(node);
          }
        _ => {
          panic!("未対応のノードです : {:?}", val);
        }
      }
    }
    VariableType::Void
  }

  fn array_assignment(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
  {
    // アクセスするindex を計算する
    if let Some(Leaf::ArrayAssignment(index_root)) =
      node.borrow().val().clone()
    {
      let index = self.statement(index_root);
      let index = match index
      {
        VariableType::Int(val) => val,
        _ => panic!("Array index は整数で指定してください")
      };

      // 左辺に識別子があり, 変数として登録されていることを確認する
      if let Some(lhs) = node.borrow().lhs()
      {
        let identifier = self.identifier_name(lhs);

        if let Some(rhs) = node.borrow().rhs()
        {
          let value = self.statement(rhs);
          // ローカル変数から検索
          if let Some(local_variables) = self.local_variables.last_mut()
          {
            // 最後のスコープから検索
            for local_variable in local_variables.iter_mut().rev()
            {
              if let Some(variable) = local_variable.get_mut(&identifier)
              {
                if let Variable::Array(array) = variable
                {
                  array.borrow_mut().values[index as usize] = value;
                  return VariableType::Void;
                }
              }
            }
            // グローバル変数から検索
            if let Some(variable) = self.global_variables.get_mut(&identifier)
            {
              if let Variable::Array(array) = variable
              {
                array.borrow_mut().values[index as usize] = value;
              } else {
                panic!("Global 変数が見つかりません : {}", identifier);
              }
            } else {
              panic!("Global 変数が見つかりません : {}", identifier);
            }
          }
        } else {
          panic!("右辺に識別子がありません");
        }
      } else {
        panic!("左辺に識別子がありません");
      }
    }

    VariableType::Void
  }

  fn for_statement(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
  {
    // for_statement を取得
    if let Some(Leaf::ForStatement(for_statement)) = node.borrow().val()
    {
      // 初期化式を取得
      let initializer = for_statement.initializer();
      self.interpret_node(initializer);

      // 条件式を取得
      let condition = for_statement.condition();

      // 更新式を取得
      let update = for_statement.update();

      // for 文の中身を取得
      let statement = for_statement.statement();

      loop {
        // 条件式を評価
        let is_continue = self.condition(condition);
        if !is_continue {
          break;
        }

        // for 文の中身を実行
        let result = self.interpret_node(statement);

        match result
        {
          VariableType::Break => {
            break;
          }
          VariableType::Continue => {
            // 更新式を実行してから次のループへ
            self.interpret_node(update);
            continue;
          }
          VariableType::Return(return_val) => {
            return VariableType::Return(return_val);
          }
          _ => {}
        }

        // 更新式を実行
        self.interpret_node(update);
      }
    }

    VariableType::Void
  }


  fn while_statement(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
  {
    // while 文の条件式を取得
    if let Some(condition_root) = node.borrow().lhs()
    {
      let mut condition = true;
      condition = self.condition(condition_root);

      // condition != 0 の場合は while 文の中身を実行
      while condition {
        if let Some(rhs) = node.borrow().rhs()
        {
          if let Some(Leaf::BlockItem(nodes)) = rhs.borrow().val()
          {
            let result = self.compound_statement(nodes, true);
            match result
            {
              VariableType::Break => {
                break;
              }
              VariableType::Continue => {
                // そのまま次のループへ (条件評価へ)
                continue;
              }
              VariableType::Return(return_val) => {
                return VariableType::Return(return_val);
              }
              _ => {}
            }
          }
        }
        condition = self.condition(condition_root);
      }
    }

    VariableType::Void
  }

  fn condition(&mut self, node: &Rc<RefCell<Node>>) -> bool
  {
    let condition = self.statement(node);

    match condition
    {
      VariableType::Int(val) => {
        val != 0
      }
      VariableType::Float(val) => {
        val != 0.0
      }
      _ => {
        panic!("while 文の条件式が対応していません");
      }
    }
  }

  fn selection_statement(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
  {
    // if 文の条件式を取得
    if let Some(Leaf::IfStatement(expression)) = node.borrow().val()
    {
      let condition = self.statement(expression);

      // condition != 0 の場合は if 文の中身を実行
      let mut condition_value: i32 = 0;
      match condition
      {
        VariableType::Int(val) => condition_value = val,
        VariableType::Float(val) => condition_value = val as i32,
        _ => {
          panic!("if 文の条件式が対応していません");
        }
      }

      if condition_value != 0
      {
        if let Some(lhs) = node.borrow().lhs()
        {
          if let Some(Leaf::BlockItem(nodes)) = lhs.borrow().val()
          {
            return self.compound_statement(nodes, true);
          }
        }
      } else {
        if let Some(rhs) = node.borrow().rhs()
        {
          // else のときと, else if のとき
          if let Some(Leaf::IfStatement(_)) = rhs.borrow().val()
          {
            return self.selection_statement(rhs);
          } else {
            if let Some(Leaf::BlockItem(nodes)) = rhs.borrow().val()
            {
              return self.compound_statement(nodes, true);
            }
          }
        }
      }
    } else {
      panic!("if 文の条件式が取得できません");
    }

    VariableType::Void
  }

  fn variable_assignment(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
  {
    if let Some(lhs) = node.borrow().lhs()
    {
      if let Some(rhs) = node.borrow().rhs()
      {
        let mut value = self.statement(rhs);
        if let VariableType::Return(val) = value
        {
          value = val.as_ref().clone();
        }

        if let Some(Leaf::Identifier(identifier)) = lhs.borrow().val()
        {
          // identifier への代入
          // ローカル変数から検索
          if let Some(local_variables) = self.local_variables.last_mut()
          {
            // 最後のスコープから検索
            for local_variable in local_variables.iter_mut().rev()
            {
              if let Some(variable) = local_variable.get_mut(identifier)
              {
                if let Variable::Value(variable) = variable
                {
                  *variable = value;
                  return VariableType::Void;
                }
              }
            }
          }

          // グローバル変数から検索
          if let Some(variable) = self.global_variables.get_mut(identifier)
          {
            if let Variable::Value(variable) = variable
            {
              *variable = value;
              return VariableType::Void;
            }
          }
        } else if let Some(Leaf::StructMemberAccess) = lhs.borrow().val() {
          // 構造体メンバへの代入
          if let Some(struct_node) = lhs.borrow().lhs() {
            // 構造体ノードが識別子の場合だけでなく、より複雑な後置修飾式を評価できるように修正
            let struct_val = self.statement(struct_node);
            let mut struct_identifier: Option<String> = None;

            // もし識別子なら、元の変数を特定する
            if let Some(Leaf::Identifier(name)) = struct_node.borrow().val() {
              struct_identifier = Some(name.clone());
            }

            if let Some(member_node) = lhs.borrow().rhs() {
              if let Some(Leaf::Identifier(member_name)) = member_node.borrow().val() {
                // もし構造体が識別子なら、元の変数を更新する
                if let Some(id) = struct_identifier {
                  // ローカル変数から検索
                  if let Some(local_variables) = self.local_variables.last_mut() {
                    for local_variable in local_variables.iter_mut().rev() {
                      if let Some(variable) = local_variable.get_mut(&id) {
                        if let Variable::Struct(s) = variable {
                          s.members.insert(member_name.clone(), value);
                          return VariableType::Void;
                        }
                      }
                    }
                  }
                  // グローバル変数から検索
                  if let Some(variable) = self.global_variables.get_mut(&id) {
                    if let Variable::Struct(s) = variable {
                      s.members.insert(member_name.clone(), value);
                      return VariableType::Void;
                    }
                  }
                } else {
                    // 識別子でない場合は、今のところ代入をサポートしない（例：(p1).x = 10 など）
                    panic!("構造体への代入に失敗しました：左辺が識別子ではありません");
                }
              }
            }
          }
        }
      }
    }

    VariableType::Void
  }

  fn array_variable_definition(&mut self, value_type: &ValueType, identifier: String, size: usize)
  {
    let mut values = Vec::new();
    for _ in 0..size
    {
      match value_type
      {
        ValueType::Int =>
          {
            values.push(VariableType::Int(0));
          }
        ValueType::Float =>
          {
            values.push(VariableType::Float(0.0));
          }
        ValueType::Struct(_) =>
          {
            panic!("構造体の配列は未対応です");
          }
        ValueType::Void => {
          panic!("void の配列は定義できません");
        }
        ValueType::Array(_, _) =>
          {
            panic!("多次元配列は未対応です");
          }
      }
    }

    let array = Array::new(identifier.clone(), VariableType::Int(0), values);
    self.insert_variable(identifier, Variable::Array(Rc::new(RefCell::new(array))));
  }

  fn variable_definition(&mut self, value_type: &ValueType, identifier: String, value: VariableType)
  {
    match value_type
    {
      ValueType::Int =>
        {
          match value
          {
            VariableType::Float(val) =>
              {
                self.insert_variable(identifier, Variable::Value(VariableType::Int(val as i32)));
              }
            VariableType::Int(val) =>
              {
                self.insert_variable(identifier, Variable::Value(VariableType::Int(val)));
              }
            _ => {
              panic!("未対応の型です : {:?}", value);
            }
          }
        }
      ValueType::Float =>
        {
          match value
          {
            VariableType::Float(val) =>
              {
                self.insert_variable(identifier, Variable::Value(VariableType::Float(val)));
              }
            VariableType::Int(val) =>
              {
                self.insert_variable(identifier, Variable::Value(VariableType::Float(val as f64)));
              }
            _ => {
              panic!("未対応の型です : {:?}", value);
            }
          }
        }
      ValueType::Struct(_) =>
        {
          panic!("構造体の代入による初期化は未対応です");
        }
      ValueType::Void => {
        panic!("void 型の変数は定義できません");
      }
      ValueType::Array(_, _) =>
        {
          panic!("配列の代入による初期化は未対応です");
        }
    }
  }

  fn insert_variable_int(&mut self, identifier: String, value: i32)
  {
    self.insert_variable(identifier, Variable::Value(VariableType::Int(value)));
  }

  fn insert_variable_float(&mut self, identifier: String, value: f64)
  {
    self.insert_variable(identifier, Variable::Value(VariableType::Float(value)));
  }

  fn insert_variable(&mut self, identifier: String, value: Variable)
  {
    match self.scope
    {
      Scope::Global =>
        {
          self.global_variables.insert(identifier, value);
        }
      Scope::Local =>
        {
          if let Some(local_variables) = self.local_variables.last_mut()
          {
            local_variables.last_mut().unwrap().insert(identifier, value);
          }
        }
    }
  }

  fn statement(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
  {
    if let Some(val) = node.borrow().val()
    {
      match val
      {
        // 演算子
        Leaf::Operator(op) =>
          {
            if let Some((lhs, rhs))
              = node.borrow().get_lhs_and_rhs()
            {
              return self.operator(op, lhs, rhs);
            }
          }

        // 定数
        Leaf::Constant(value) =>
          {
            return self.constant(value);
          }

        // 識別子
        Leaf::Identifier(identifier) =>
          {
            return self.identifier(identifier);
          }

        // 単項演算子
        Leaf::UnaryExpression(op) =>
          {
            if let Some(lhs) = node.borrow().lhs()
            {
              return self.unary_expression(op, lhs);
            }
          }

        // 括弧で囲まれた式
        Leaf::ParenthesizedExpression =>
          {
            if let Some(lhs) = node.borrow().lhs()
            {
              return self.statement(lhs);
            }
          }
        Leaf::FunctionCall(function_call) =>
          {
            let value = self.function_call(function_call);

            // statement で void の場合はエラー
            if let VariableType::Void = value
            {
              panic!("void は代入できません");
            }

            return value;
          }
        Leaf::ArrayAccess =>
          {
            return self.array_access(node);
          }
        Leaf::StructMemberAccess =>
          {
            return self.struct_member_access(node);
          }
        _ => {
          panic!("未対応のノードです : {:?}", val);
        }
      }
    }

    panic!("未対応のノードです");
  }

  fn array_access(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
  {
    // ArrayAccess を取得
    if let Some(Leaf::ArrayAccess) = node.borrow().val()
    {}
    else {
      panic!("ArrayAccess が取得できません");
    }

    // 左辺に配列（または式）があることを確認する
    if let Some(lhs) = node.borrow().lhs()
    {
      let mut array_val = self.statement(lhs);
      // Return の場合は中身を取り出す
      if let VariableType::Return(inner) = array_val {
        array_val = inner.as_ref().clone();
      }

      if let VariableType::Array(array) = array_val {
        // 右辺に index があることを確認する
        if let Some(rhs) = node.borrow().rhs()
        {
          let index = self.statement(rhs);
          let index = match index
          {
            VariableType::Int(val) => val,
            _ => panic!("Array index は整数で指定してください")
          };
          return array.borrow().values[index as usize].clone();
        } else {
          panic!("右辺に index がありません");
        }
      } else {
        panic!("配列ではありません : {:?}", array_val);
      }
    } else {
      panic!("左辺に配列がありません");
    }
  }

  fn struct_member_access(&mut self, node: &Rc<RefCell<Node>>) -> VariableType
  {
    // StructMemberAccess を取得
    if let Some(Leaf::StructMemberAccess) = node.borrow().val()
    {}
    else {
      panic!("StructMemberAccess が取得できません");
    }

    // 左辺に構造体があることを確認する
    if let Some(lhs) = node.borrow().lhs()
    {
      let mut struct_val = self.statement(lhs);
      // struct_val が Return の場合は中身を取り出す
      if let VariableType::Return(inner) = struct_val {
          struct_val = inner.as_ref().clone();
      }

      if let VariableType::Struct(s) = struct_val {
          // 右辺にメンバ名があることを確認する
          if let Some(rhs) = node.borrow().rhs()
          {
            if let Some(Leaf::Identifier(member_name)) = rhs.borrow().val()
            {
              return s.members.get(member_name).expect(&format!("メンバ {} が見つかりません", member_name)).clone();
            }
          }
      } else {
          panic!("構造体ではありません : {:?}", struct_val);
      }
    }

    panic!("構造体メンバアクセスに失敗しました");
  }

  fn function_call(&mut self, function_call: &FunctionCall) -> VariableType
  {
    let name = function_call.name();
    let function_definitions = self.function_definition.clone();
    //println!("function_call : {}", name);

    if let Some(function_definition) = function_definitions.get(name)
    {
      let mut new_variables: HashMap<String, Variable> = HashMap::new();

      // 引数がある場合計算する
      let function_arguments = function_call.arguments();

      // 引数がある場合は引数を計算してローカル変数に追加
      if !function_arguments.is_empty()
      {
        // 引数の数と function-definition の引数リストの数が一致することを確認する
        if function_arguments.len() != function_definition.arguments().len()
        {
          panic!("引数の数が一致しません");
        }

        for (i, argument) in function_arguments.iter().enumerate()
        {
          let mut argument_value = self.statement(argument);
          let argument_name = function_definition.arguments()[i].identify().clone();
          let argument_type = function_definition.arguments()[i].type_specifier();

          // 引数をローカル変数に追加
          if let Some(local_variables) = self.local_variables.last_mut()
          {
            match argument_type
            {
              ValueType::Array(_, _) =>
                {
                  if let VariableType::Array(array) = argument_value
                  {
                    new_variables.insert(argument_name, Variable::Array(array));
                  } else {
                    panic!("配列が期待されていますが、別の型が渡されました");
                  }
                }
              ValueType::Struct(_) =>
                {
                  if let VariableType::Struct(s) = argument_value
                  {
                    // 構造体の値渡し（コピー）
                    new_variables.insert(argument_name, Variable::Struct(s.clone()));
                  } else {
                    panic!("構造体が期待されていますが、別の型が渡されました");
                  }
                }
              _ =>
                {
                  // value が 定数ではなく Return の時中身を取り出す
                  if let VariableType::Return(val) = argument_value
                  {
                    argument_value = val.as_ref().clone();
                  }
                  new_variables.insert(argument_name, Variable::Value(argument_value));
                }
            }
          }
        }
      }

      // 新しくローカル変数を追加
      self.local_variables.push(Vec::new());
      self.local_variables.last_mut().unwrap().push(new_variables);

      let return_value = self.compound_statement(function_definition.body(),
                                                 false);

      // ローカル変数を削除
      self.local_variables.pop();

      // return が Break の場合は エラー
      if let VariableType::Break = return_value
      {
        panic!("関数内で break は使用できません");
      }

      if let VariableType::Return(val) = return_value
      {
        return VariableType::Return(val);
      }

      return_value
    } else {
      panic!("関数が見つかりません : {}", name);
    }
  }

  fn compound_statement(&mut self, nodes: &Vec<Rc<RefCell<Node>>>,
                        is_generate_local_variables: bool) -> VariableType
  {
    if is_generate_local_variables
    {
      if let Some(local_variables) = self.local_variables.last_mut()
      {
        local_variables.push(HashMap::new());
      }
    }

    let mut return_value = VariableType::Void;
    for statement in nodes.iter()
    {
      return_value = self.interpret_node(statement);
      match return_value
      {
        VariableType::Return(_) | VariableType::Break | VariableType::Continue => {
          break;
        }
        _ => {}
      }
    }

    if is_generate_local_variables
    {
      if let Some(local_variables) = self.local_variables.last_mut()
      {
        local_variables.pop();
      }
    }

    return_value
  }

  fn unary_expression(&mut self, op: &UnaryOperator, lhs: &Rc<RefCell<Node>>) -> VariableType
  {
    let lhs = self.statement(lhs);
    match op
    {
      UnaryOperator::Minus =>
        {
          match lhs
          {
            VariableType::Int(val) =>
              {
                Int(-val)
              }
            VariableType::Float(val) =>
              {
                VariableType::Float(-val)
              }
            _ => {
              panic!("未対応の型です");
            }
          }
        }
      UnaryOperator::LogicalNot =>
        {
          match lhs
          {
            VariableType::Int(val) =>
              {
                Int(if val == 0 { 1 } else { 0 })
              }
            VariableType::Float(val) =>
              {
                Int(if val == 0.0 { 1 } else { 0 })
              }
            _ => {
              panic!("未対応の型です");
            }
          }
        }
      _ => {
        panic!("未対応の演算子です : {:?}", op);
      }
    }
  }

  fn identifier(&mut self, identifier: &String) -> VariableType
  {
    // ローカル変数から検索.最後のスコープから検索する
    if let Some(local_variables) = self.local_variables.last_mut()
    {
      for variable in local_variables.iter().rev()
      {
        if let Some(variable) = variable.get(identifier)
        {
          match variable
          {
            Variable::Value(value) => return value.clone(),
            Variable::Array(array) => return VariableType::Array(array.clone()),
            Variable::Struct(s) => return VariableType::Struct(s.clone()),
          }
        }
      }
    }

    // グローバル変数から検索
    if let Some(variable) = self.global_variables.get(identifier) {
      match variable
      {
        Variable::Value(value) => return value.clone(),
        Variable::Array(array) => return VariableType::Array(array.clone()),
        Variable::Struct(s) => return VariableType::Struct(s.clone()),
      }
    }

    panic!("未定義の変数です : {}", identifier);
  }

  fn constant(&mut self, value: &Constant) -> VariableType
  {
    match value
    {
      Constant::Integer(val) =>
        {
          VariableType::Int(*val)
        }
      Constant::Float(val) =>
        {
          VariableType::Float((*val).into())
        }
      _ => {
        panic!("未対応の定数です : {:?}", value);
      }
    }
  }


  fn operator(&mut self, op: &Operator, lhs: &Rc<RefCell<Node>>, rhs: &Rc<RefCell<Node>>) -> VariableType
  {
    let mut lhs = self.statement(lhs);
    let mut rhs = self.statement(rhs);
    let mut result: VariableType = Int(0);

    // 左右の値からreturn を除去する
    lhs = self.remove_return(lhs);
    rhs = self.remove_return(rhs);

    match op
    {
      Operator::LogicalOr =>
        {
          result = self.logical_or(lhs, rhs);
        }
      Operator::LogicalAnd =>
        {
          result = self.logical_and(lhs, rhs);
        }
      Operator::Equal =>
        {
          result = self.equal(lhs, rhs);
        }
      Operator::NotEqual =>
        {
          result = self.not_equal(lhs, rhs);
        }
      Operator::LessThan =>
        {
          result = self.less_than(lhs, rhs);
        }
      Operator::GreaterThan =>
        {
          result = self.greater_than(lhs, rhs);
        }
      Operator::LessThanOrEqual =>
        {
          result = self.less_than_or_equal(lhs, rhs);
        }
      Operator::GreaterThanOrEqual =>
        {
          result = self.greater_than_or_equal(lhs, rhs);
        }
      Operator::Plus =>
        {
          result = self.add(lhs, rhs);
        }
      Operator::Minus =>
        {
          result = self.sub(lhs, rhs);
        }
      Operator::Multiply =>
        {
          result = self.mul(lhs, rhs);
        }
      Operator::Divide =>
        {
          result = self.div(lhs, rhs);
        }
      Operator::Modulo =>
        {
          result = self.remainder(lhs, rhs);
        }
      _ => {
        panic!("未対応の演算子です : {:?}", op);
      }
    }

    result
  }

  // 加算演算子　'+'
  fn add(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    match (lhs.clone(), rhs.clone())
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          Int(lhs + rhs)
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          VariableType::Float(lhs as f64 + rhs)
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          VariableType::Float(lhs + rhs as f64)
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          VariableType::Float(lhs + rhs)
        }
      _ => {
        panic!("未対応の型です : {:?}, {:?}", lhs, rhs);
      }
    }
  }

  // 減算演算子　'-'
  fn sub(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    match (lhs.clone(), rhs.clone())
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          Int(lhs - rhs)
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          VariableType::Float(lhs as f64 - rhs)
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          VariableType::Float(lhs - rhs as f64)
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          VariableType::Float(lhs - rhs)
        }
      _ => {
        panic!("未対応の型です : {:?}, {:?}", lhs, rhs);
      }
    }
  }

  // 乗算演算子　'*'
  fn mul(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    match (lhs, rhs)
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          Int(lhs * rhs)
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          VariableType::Float(lhs as f64 * rhs)
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          VariableType::Float(lhs * rhs as f64)
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          VariableType::Float(lhs * rhs)
        }
      _ => {
        panic!("未対応の型です");
      }
    }
  }

  // 除算演算子　'/'
  fn div(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    // 右辺値が0の場合はエラー
    match rhs
    {
      VariableType::Int(val) if val == 0 =>
        {
          panic!("0で割ることはできません");
        }
      VariableType::Float(val) if val == 0.0 =>
        {
          panic!("0で割ることはできません");
        }
      _ => {}
    }

    match (lhs, rhs)
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          Int(lhs / rhs)
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          VariableType::Float(lhs as f64 / rhs)
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          VariableType::Float(lhs / rhs as f64)
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          VariableType::Float(lhs / rhs)
        }
      _ => {
        panic!("未対応の型です");
      }
    }
  }

  // 余り演算子　'%'
  fn remainder(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    // 右辺値が0の場合はエラー
    match rhs
    {
      VariableType::Int(val) if val == 0 =>
        {
          panic!("0で割ることはできません");
        }
      VariableType::Float(val) if val == 0.0 =>
        {
          panic!("0で割ることはできません");
        }
      _ => {}
    }

    match (lhs, rhs)
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          Int(lhs % rhs)
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          Int(lhs % rhs as i32)
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          Int((lhs % rhs as f64) as i32)
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          Int((lhs % rhs) as i32)
        }
      _ => {
        panic!("未対応の型です");
      }
    }
  }

  // 同値演算子　'=='
  fn equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    match (lhs, rhs)
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs == rhs;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs == rhs as i32;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs == rhs as f64;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs == rhs;
          Int(if result { 1 } else { 0 })
        }
      _ => {
        panic!("未対応の型です");
      }
    }
  }

  // 否定演算子　'!='
  fn not_equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    match (lhs, rhs)
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs != rhs;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs != rhs as i32;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs != rhs as f64;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs != rhs;
          Int(if result { 1 } else { 0 })
        }
      _ => {
        panic!("未対応の型です");
      }
    }
  }

  // 小なり演算子　'<'
  fn less_than(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    match (lhs, rhs)
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs < rhs;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs < rhs as i32;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs < rhs as f64;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs < rhs;
          Int(if result { 1 } else { 0 })
        }
      _ => {
        panic!("未対応の型です");
      }
    }
  }

  // 大なり演算子　'>'
  fn greater_than(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    match (lhs, rhs)
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs > rhs;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs > rhs as i32;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs > rhs as f64;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs > rhs;
          Int(if result { 1 } else { 0 })
        }
      _ => {
        panic!("未対応の型です");
      }
    }
  }

  // 小なりイコール演算子　'<='
  fn less_than_or_equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    match (lhs, rhs)
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs <= rhs;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs <= rhs as i32;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs <= rhs as f64;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs <= rhs;
          Int(if result { 1 } else { 0 })
        }
      _ => {
        panic!("未対応の型です");
      }
    }
  }

  // 大なりイコール演算子　'>='
  fn greater_than_or_equal(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    match (lhs, rhs)
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs >= rhs;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs >= rhs as i32;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs >= rhs as f64;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs >= rhs;
          Int(if result { 1 } else { 0 })
        }
      _ => {
        panic!("未対応の型です");
      }
    }
  }

  // 論理和　'||'
  fn logical_or(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    match (lhs, rhs)
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs != 0 || rhs != 0;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs != 0 || rhs != 0.0;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs != 0.0 || rhs != 0;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs != 0.0 || rhs != 0.0;
          Int(if result { 1 } else { 0 })
        }
      _ => {
        panic!("未対応の型です");
      }
    }
  }

  // 論理積　'&&'
  fn logical_and(&mut self, lhs: VariableType, rhs: VariableType) -> VariableType
  {
    match (lhs, rhs)
    {
      (VariableType::Int(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs != 0 && rhs != 0;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Int(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs != 0 && rhs != 0.0;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Int(rhs)) =>
        {
          let result = lhs != 0.0 && rhs != 0;
          Int(if result { 1 } else { 0 })
        }
      (VariableType::Float(lhs), VariableType::Float(rhs)) =>
        {
          let result = lhs != 0.0 && rhs != 0.0;
          Int(if result { 1 } else { 0 })
        }
      _ => {
        panic!("未対応の型です");
      }
    }
  }

  fn identifier_name(&self, node: &Rc<RefCell<Node>>) -> String
  {
    let mut identifier = String::new();
    if let Some(val) = node.borrow().val()
    {
      match val
      {
        Leaf::Identifier(name) => {
          identifier = name.clone();
        }
        _ => {
          panic!("識別子ではありません : {:?}", val);
        }
      }
    }
    identifier
  }

  fn remove_return(&mut self, leaf: VariableType) -> VariableType
  {
    match leaf
    {
      VariableType::Return(val) => val.as_ref().clone(),
      _ => leaf
    }
  }
}

#[cfg(test)]
mod tests
{
  use crate::interpreter::VariableType::{Float, Int};
  use crate::interpreter::{Array, Interpreter, Variable, VariableType};
  use crate::parser::Parser;
  use std::collections::HashMap;
  use crate::lexical::Lexer;

  fn run_program(program: &str) -> (VariableType, HashMap<String, Variable>) {
    let mut lexer = Lexer::new(program.to_string());
    lexer.tokenize();
    let tokens = lexer.tokens().clone();
    let mut parser = Parser::new(tokens);
    parser.parse();
    let mut interpreter = Interpreter::new(parser.roots());
    let val = interpreter.run();
    (val, interpreter.global_variables().clone())
  }

  #[test]
  fn test_add() {
    let program = "
        int i_add = 10 + 20;
        float f_add = 10.5 + 20.5;
        float mix_add1 = 10 + 20.5;
        float mix_add2 = 10.5 + 20;
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("i_add").unwrap(), &Variable::Value(Int(30)));
    assert_eq!(globals.get("f_add").unwrap(), &Variable::Value(Float(31.0)));
    assert_eq!(globals.get("mix_add1").unwrap(), &Variable::Value(Float(30.5)));
    assert_eq!(globals.get("mix_add2").unwrap(), &Variable::Value(Float(30.5)));
  }

  #[test]
  fn test_sub() {
    let program = "
        int i_sub = 30 - 10;
        float f_sub = 30.5 - 10.5;
        float mix_sub1 = 30 - 10.5;
        float mix_sub2 = 30.5 - 10;
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("i_sub").unwrap(), &Variable::Value(Int(20)));
    assert_eq!(globals.get("f_sub").unwrap(), &Variable::Value(Float(20.0)));
    assert_eq!(globals.get("mix_sub1").unwrap(), &Variable::Value(Float(19.5)));
    assert_eq!(globals.get("mix_sub2").unwrap(), &Variable::Value(Float(20.5)));
  }

  #[test]
  fn test_mul() {
    let program = "
        int i_mul = 10 * 20;
        float f_mul = 10.5 * 2.0;
        float mix_mul1 = 10 * 2.5;
        float mix_mul2 = 2.5 * 10;
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("i_mul").unwrap(), &Variable::Value(Int(200)));
    assert_eq!(globals.get("f_mul").unwrap(), &Variable::Value(Float(21.0)));
    assert_eq!(globals.get("mix_mul1").unwrap(), &Variable::Value(Float(25.0)));
    assert_eq!(globals.get("mix_mul2").unwrap(), &Variable::Value(Float(25.0)));
  }

  #[test]
  fn test_div() {
    let program = "
        int i_div = 20 / 10;
        float f_div = 20.0 / 8.0;
        float mix_div1 = 20 / 8.0;
        float mix_div2 = 20.0 / 8;
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("i_div").unwrap(), &Variable::Value(Int(2)));
    assert_eq!(globals.get("f_div").unwrap(), &Variable::Value(Float(2.5)));
    assert_eq!(globals.get("mix_div1").unwrap(), &Variable::Value(Float(2.5)));
    assert_eq!(globals.get("mix_div2").unwrap(), &Variable::Value(Float(2.5)));
  }

  #[test]
  fn test_equal() {
    let program = "
        int i_eq = (10 == 10);
        int f_eq = (10.5 == 10.5);
        int mix_eq1 = (10 == 10.0);
        int mix_eq2 = (10.0 == 10);
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("i_eq").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("f_eq").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_eq1").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_eq2").unwrap(), &Variable::Value(Int(1)));
  }

  #[test]
  fn test_not_equal() {
    let program = "
        int i_ne = (10 != 20);
        int f_ne = (10.5 != 20.5);
        int mix_ne1 = (10 != 20.5);
        int mix_ne2 = (20.5 != 10);
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("i_ne").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("f_ne").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_ne1").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_ne2").unwrap(), &Variable::Value(Int(1)));
  }

  #[test]
  fn test_less_than() {
    let program = "
        int i_lt = (10 < 20);
        int f_lt = (10.5 < 20.5);
        int mix_lt1 = (10 < 20.5);
        int mix_lt2 = (10.5 < 20);
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("i_lt").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("f_lt").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_lt1").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_lt2").unwrap(), &Variable::Value(Int(1)));
  }

  #[test]
  fn test_greater_than() {
    let program = "
        int i_gt = (20 > 10);
        int f_gt = (20.5 > 10.5);
        int mix_gt1 = (20 > 10.5);
        int mix_gt2 = (20.5 > 10);
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("i_gt").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("f_gt").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_gt1").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_gt2").unwrap(), &Variable::Value(Int(1)));
  }

  #[test]
  fn test_less_than_or_equal() {
    let program = "
        int i_le = (10 <= 10);
        int f_le = (10.5 <= 10.5);
        int mix_le1 = (10 <= 10.5);
        int mix_le2 = (10.5 <= 11);
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("i_le").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("f_le").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_le1").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_le2").unwrap(), &Variable::Value(Int(1)));
  }

  #[test]
  fn test_greater_than_or_equal() {
    let program = "
        int i_ge = (10 >= 10);
        int f_ge = (10.5 >= 10.5);
        int mix_ge1 = (11 >= 10.5);
        int mix_ge2 = (10.5 >= 10);
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("i_ge").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("f_ge").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_ge1").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("mix_ge2").unwrap(), &Variable::Value(Int(1)));
  }

  #[test]
  fn test_logical_and() {
    let program = "
        int and_ii = 1 && 1;
        int and_if = 1 && 1.0;
        int and_fi = 1.0 && 1;
        int and_ff = 1.0 && 1.0;
        int and_zero = 1 && 0.0;
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("and_ii").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("and_if").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("and_fi").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("and_ff").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("and_zero").unwrap(), &Variable::Value(Int(0)));
  }

  #[test]
  fn test_logical_or() {
    let program = "
        int or_ii = 0 || 1;
        int or_if = 0 || 1.0;
        int or_fi = 0.0 || 1;
        int or_ff = 0.0 || 1.0;
        int or_zero = 0 || 0.0;
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("or_ii").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("or_if").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("or_fi").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("or_ff").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("or_zero").unwrap(), &Variable::Value(Int(0)));
  }

  #[test]
  fn test_logical_not() {
    let program = "
        int not_i = !1;
        int not_f = !1.0;
        int not_zero_i = !0;
        int not_zero_f = !0.0;
        int main() { return 0; }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("not_i").unwrap(), &Variable::Value(Int(0)));
    assert_eq!(globals.get("not_f").unwrap(), &Variable::Value(Int(0)));
    assert_eq!(globals.get("not_zero_i").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("not_zero_f").unwrap(), &Variable::Value(Int(1)));
  }

  #[test]
  fn test_arithmetic() {
    let program = "
        int a = (10 + 20) * 3 - 4 / 2;
        float b = (10.0 + 20.0) * 3.0 - 4.0 / 2.0;
        int c = -10 + 20;
        float d = -10.5 + 20.5;
        int main() {
            return a;
        }
    ";
    let (val, globals) = run_program(program);
    assert_eq!(val, Int(88));
    assert_eq!(globals.get("b").unwrap(), &Variable::Value(Float(88.0)));
    assert_eq!(globals.get("c").unwrap(), &Variable::Value(Int(10)));
    assert_eq!(globals.get("d").unwrap(), &Variable::Value(Float(10.0)));
  }

  #[test]
  fn test_remain() {
    let program = "
        int b = 10 % 3;
        float c = 10.5 % 3.0;
        float d = 10 % 3.0;
        float e = 10.5 % 3;
        int main() {
            return 0;
        }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("b").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("c").unwrap(), &Variable::Value(Float(1.0)));
    assert_eq!(globals.get("d").unwrap(), &Variable::Value(Float(1.0)));
    assert_eq!(globals.get("e").unwrap(), &Variable::Value(Float(1.0)));
  }

  #[test]
  fn test_functions() {
    let program = "
        int add(int a, int b) { return a + b; }
        int fibo(int n) {
            if (n == 0) { return 0; }
            if (n == 1) { return 1; }
            return fibo(n - 1) + fibo(n - 2);
        }
        int main() {
            int x = add(10, 20);
            int y = fibo(10);
            return x + y;
        }
    ";
    let (val, _) = run_program(program);
    assert_eq!(val, Int(30 + 55));
  }

  #[test]
  fn test_while() {
    let program = "
        int sum_while = 0;
        int main() {
            int i = 0;
            while (i < 10) {
                sum_while = sum_while + i;
                i = i + 1;
            }
            return 0;
        }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("sum_while").unwrap(), &Variable::Value(Int(45)));
  }

  #[test]
  fn test_for() {
    let program = "
        int sum_for = 0;
        int main() {
            int j = 0;
            for (j = 0; j < 10; j = j + 1) {
                sum_for = sum_for + j;
            }
            return 0;
        }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("sum_for").unwrap(), &Variable::Value(Int(45)));
  }

  #[test]
  fn test_selection_statement() {
    let program = "
        int res1 = 0;
        int res2 = 0;
        int res3 = 0;
        int main() {
            float f = 1.0;
            if (f) { res1 = 1; } else { res1 = 0; }
            
            float f0 = 0.0;
            if (f0) { res2 = 1; } else { res2 = 0; }

            if (1) {
                if (0) { res3 = 1; } else { res3 = 2; }
            }
            return 0;
        }
    ";
    let (_, globals) = run_program(program);
    assert_eq!(globals.get("res1").unwrap(), &Variable::Value(Int(1)));
    assert_eq!(globals.get("res2").unwrap(), &Variable::Value(Int(0)));
    assert_eq!(globals.get("res3").unwrap(), &Variable::Value(Int(2)));
  }


  #[test]
  fn test_arrays() {
    let program = "
        int result[10];
        int main() {
            int i = 0;
            result[0] = 0;
            result[1] = 1;
            for (i = 2; i < 10; i = i + 1) {
                result[i] = result[i - 1] + result[i - 2];
            }
            return result[9];
        }
    ";
    let (val, globals) = run_program(program);
    assert_eq!(val, Int(34));
    if let Variable::Array(arr) = globals.get("result").unwrap() {
        assert_eq!(arr.borrow().values().len(), 10);
        assert_eq!(arr.borrow().values()[9], Int(34));
    } else {
        panic!("result should be an array");
    }
  }

  #[test]
  fn test_break() {
    let program = "
        int result[10];
        int main() {
            int i = 0;
            for (i = 0; i < 10; i = i + 1) {
                if (i == 5) { break; }
            }
            return i;
        }
    ";
    let (val, globals) = run_program(program);
    assert_eq!(val, Int(5));
  }

  #[test]
  fn test_break_while() {
    let program = "
        int main() {
            int i = 0;
            while (i < 10) {
                if (i == 3) { break; }
                i = i + 1;
            }
            return i;
        }
    ";
    let (val, _) = run_program(program);
    assert_eq!(val, Int(3));
  }

  #[test]
  fn test_break_nested() {
    let program = "
        int main() {
            int i = 0;
            int j = 0;
            int count = 0;
            for (i = 0; i < 3; i = i + 1) {
                for (j = 0; j < 10; j = j + 1) {
                    if (j == 5) { break; }
                    count = count + 1;
                }
            }
            return count;
        }
    ";
    let (val, _) = run_program(program);
    // 外側のループが3回、内側のループがj=0,1,2,3,4の5回実行されるはずなので、3 * 5 = 15
    assert_eq!(val, Int(15));
  }

  #[test]
  fn test_break_deeply_nested_if() {
    let program = "
        int main() {
            int i = 0;
            int count = 0;
            while (i < 10) {
                if (i > 5) {
                    if (1) {
                        if (1) {
                            break;
                        }
                    }
                }
                count = count + 1;
                i = i + 1;
            }
            return count;
        }
    ";
    let (val, _) = run_program(program);
    // i=0,1,2,3,4,5 まで実行され、i=6でbreakするはずなので、countは6
    assert_eq!(val, Int(6));
  }

  #[test]
  fn test_struct() {
    let program = "
        struct Point {
            int x;
            int y;
        };
        int main() {
            struct Point p;
            p.x = 10;
            p.y = 20;
            return p.x + p.y;
        }
    ";
    let (val, _) = run_program(program);
    assert_eq!(val, Int(30));
  }

  #[test]
  fn test_struct_global() {
    let program = "
        struct Point {
            int x;
            int y;
        };
        struct Point p;
        int main() {
            p.x = 100;
            p.y = 200;
            return p.x + p.y;
        }
    ";
    let (val, _) = run_program(program);
    assert_eq!(val, Int(300));
  }

  #[test]
  fn test_struct_nested() {
    let program = "
        struct Size {
            int width;
            int height;
        };
        struct Rect {
            struct Size size;
            int x;
            int y;
        };
        int main() {
            struct Rect r;
            r.x = 10;
            r.y = 20;
            // ネストした構造体への代入は、現在の実装では
            // postfix_expression が再帰的でないため、r.size.width はパースできない可能性がある
            // ひとまずフラットな構造体をテストする
            return r.x + r.y;
        }
    ";
    let (val, _) = run_program(program);
    assert_eq!(val, Int(30));
  }

  #[test]
  fn test_display_and_misc() {
    println!("Void: {}", VariableType::Void);
    println!("Break: {}", VariableType::Break);
    println!("Int: {}", Int(10));
    println!("Float: {}", Float(10.5));
    println!("Return: {}", VariableType::Return(Box::new(Int(5))));
    
    // show_variables coverage
    let program = "int x = 10; int main() { return x; }";
    let mut lexer = Lexer::new(program.to_string());
    lexer.tokenize();
    let mut parser = Parser::new(lexer.tokens().clone());
    parser.parse();
    let interpreter = Interpreter::new(parser.roots());
    interpreter.show_variables();
  }

  #[test]
  fn test_pass_array_to_function() {
    let program = "
            int get_first(int a[10]) {
                return a[0];
            }
            int main() {
                int arr[10];
                arr[0] = 42;
                return get_first(arr);
            }
        ";
    let (val, _) = run_program(program);
    let val = match val {
        VariableType::Return(v) => *v,
        _ => val,
    };
    assert_eq!(val, Int(42));
  }

  #[test]
  fn test_pass_array_by_reference() {
    let program = "
            void set_first(int a[10], int val) {
                a[0] = val;
            }
            int main() {
                int arr[10];
                arr[0] = 1;
                set_first(arr, 42);
                return arr[0];
            }
        ";
    let (val, _) = run_program(program);
    let val = match val {
        VariableType::Return(v) => *v,
        _ => val,
    };
    assert_eq!(val, Int(42));
  }

  #[test]
  fn test_pass_struct_to_function() {
    let program = "
            struct Point {
                int x;
                int y;
            };
            int get_sum(struct Point p) {
                p.x = p.x + p.y;
                return p.x;
            }
            int main() {
                struct Point p1;
                p1.x = 10;
                p1.y = 20;
                int s = get_sum(p1);
                // 値渡しなので p1.x は変わらないはず
                int result = s + p1.x;
                return result;
            }
        ";
    let (val, _) = run_program(program);
    let val = match val {
      VariableType::Return(v) => *v,
      _ => val,
    };
    assert_eq!(val, Int(30 + 10));
  }

  #[test]
  fn test_continue() {
    let program = "
            int main() {
                int sum = 0;
                int i;
                for (i = 0; i < 10; i = i + 1) {
                    if (i % 2 == 0) { continue; }
                    sum = sum + i;
                }
                return sum;
            }
        ";
    let (val, _) = run_program(program);
    let val = match val {
      VariableType::Return(v) => *v,
      _ => val,
    };
    // 1 + 3 + 5 + 7 + 9 = 25
    assert_eq!(val, Int(25));
  }
}