use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use inkwell::OptimizationLevel;
use std::error::Error;
use std::path::Path;
use core::tree_viewer::TreeViewer;
use core::lexical::Lexer;
use core::parser::Parser;
use core::llvm_ir::compile;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn jit_compile_helloworld(&self) -> FunctionValue<'ctx> {
        let void_type = self.context.void_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::from(0));

        // printf を宣言
        let printf_type = i8_ptr_type.fn_type(&[i8_ptr_type.into()], /* is_var_arg */ true);
        let printf = self.module.add_function("printf", printf_type, None);

        // hello_world 関数を定義
        let function_type = void_type.fn_type(&[], false);
        let function = self.module.add_function("hello_world", function_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let hello_str = self.builder.build_global_string_ptr("hello, world\n", "hello_str");

        // printf 呼び出し
        self.builder.build_call(
            printf,
            &[hello_str.expect("REASON").as_pointer_value().into()],
            "printf_call",
        );

        self.builder.build_return(None);

        // 今までは function.as_global_value().as_pointer_value() を返していたが、
        // build_call が期待するのは FunctionValue なので、そのまま 'function' を返す
        function
    }


    fn add_main_function(&self, hello_world_func: FunctionValue<'ctx>) {
        let void_type = self.context.void_type();

        // main 関数を定義
        let function_type = void_type.fn_type(&[], false);
        let function = self.module.add_function("main", function_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        // hello_world 関数を呼び出し
        self.builder.build_call(hello_world_func, &[], "call_hello_world");

        // main を終了
        self.builder.build_return(None);
    }
}

fn easy_compiler() -> Result<(), Box<dyn Error>>
{
    let context = Context::create();
    let module = context.create_module("hello_world");
    let builder = context.create_builder();
    let codegen = CodeGen {
        context: &context,
        module,
        builder,
    };

    // hello_world 関数を生成
    let hello_world_func = codegen.jit_compile_helloworld();

    // main 関数を追加
    codegen.add_main_function(hello_world_func);

    // ビットコードをファイルに保存
    let path = Path::new("hello_world.bc");
    codegen
        .module
        .write_bitcode_to_path(path);

    // IR を表示
    println!("----------------------");
    codegen.module.print_to_stderr();
    println!("----------------------");

    println!("LLVM bitcode written to {:?}", path);

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let program = String::from("
int add(int x, int y) {
    int z = x + y;
    return z;
}

int main()
{
    int a = add(1, 2);
    return 1;
}
    ");


    let mut lexer = Lexer::new(program);
    lexer.tokenize();

    lexer.show_tokens();

    println!("----------------------");

    let tokens = lexer.tokens().clone();
    let mut parser = Parser::new(tokens);
    parser.parse();

    println!("----------------------");
    parser.show_tree();


    let mut tree_viewer = TreeViewer::new();

    for (i, root) in parser.roots().iter().enumerate() {
        tree_viewer.make_tree(root);
    }
    tree_viewer.output_dot("trees/output.dot");

    let roots = parser.roots();
    compile(roots)?;

    Ok(())
}
