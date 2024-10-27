#[cfg(test)]
mod tests {
    use std::collections::hash_map::Values;
    use NagatoInterpreter::{Interpreter, Value};
    use super::*;

    #[test]
    fn test_interpreter_new() {
        let source_code = ("a = 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10");
        let mut interpreter = Interpreter::new(source_code);

        interpreter.run();

        assert_eq!(interpreter.get_variable("a"), Value::Int(55));
    }

    }