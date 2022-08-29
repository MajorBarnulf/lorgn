mod module;
pub use module::Module;

mod function;
pub use function::Function;

mod runtime;
pub use runtime::Runtime;

mod value;
pub use value::Value;

#[test]
fn test_runtime() {
    use lorgn_lang::ast::{Expr, FnCall, Path};

    let mut runtime = Runtime::default();
    let mut module = Module::new_empty("std");
    module.push_native("print".into(), |[item]: [Value; 1]| {
        println!("{item:?}");
        Value::None
    });
    runtime.register(module);
    runtime.evaluate(Expr::FnCall(FnCall {
        fn_path: Path {
            module: "std".into(),
            item: "print".into(),
        },
        arguments: vec![Expr::Litteral("hello yorld".into()).boxed()],
    }));
}
