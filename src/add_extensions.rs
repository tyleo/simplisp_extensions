use simplisp::Environment as LispEnvironment;
use simplisp::Symbol;
use funcs::*;

pub fn add_extensions<TEnvironment>(lisp_environment: &mut LispEnvironment<TEnvironment>) {
    lisp_environment.get_global_frame().insert("f32".to_string(), Symbol::BuiltInFunc(f32));
    lisp_environment.get_global_frame().insert("f64".to_string(), Symbol::BuiltInFunc(f64));
    lisp_environment.get_global_frame().insert("u8".to_string(), Symbol::BuiltInFunc(u8));
    lisp_environment.get_global_frame().insert("u16".to_string(), Symbol::BuiltInFunc(u16));
    lisp_environment.get_global_frame().insert("u32".to_string(), Symbol::BuiltInFunc(u32));
    lisp_environment.get_global_frame().insert("u64".to_string(), Symbol::BuiltInFunc(u64));
    lisp_environment.get_global_frame().insert("usize".to_string(), Symbol::BuiltInFunc(usize));
    lisp_environment.get_global_frame().insert("i8".to_string(), Symbol::BuiltInFunc(i8));
    lisp_environment.get_global_frame().insert("i16".to_string(), Symbol::BuiltInFunc(i16));
    lisp_environment.get_global_frame().insert("i32".to_string(), Symbol::BuiltInFunc(i32));
    lisp_environment.get_global_frame().insert("i64".to_string(), Symbol::BuiltInFunc(i64));
    lisp_environment.get_global_frame().insert("isize".to_string(), Symbol::BuiltInFunc(isize));

    lisp_environment.get_global_frame().insert("if".to_string(), Symbol::BuiltInFunc(if_lisp));
    lisp_environment.get_global_frame().insert("let".to_string(), Symbol::BuiltInFunc(let_lisp));
    lisp_environment.get_global_frame().insert("repeat".to_string(), Symbol::BuiltInFunc(repeat));

    lisp_environment.get_global_frame().insert("+".to_string(), Symbol::BuiltInFunc(add));
    lisp_environment.get_global_frame().insert("&".to_string(), Symbol::BuiltInFunc(bitand));
    lisp_environment.get_global_frame().insert("|".to_string(), Symbol::BuiltInFunc(bitor));
    lisp_environment.get_global_frame().insert("^".to_string(), Symbol::BuiltInFunc(bitxor));
    lisp_environment.get_global_frame().insert("/".to_string(), Symbol::BuiltInFunc(div));
    lisp_environment.get_global_frame().insert("==".to_string(), Symbol::BuiltInFunc(eq));
    lisp_environment.get_global_frame().insert(">=".to_string(), Symbol::BuiltInFunc(ge));
    lisp_environment.get_global_frame().insert(">".to_string(), Symbol::BuiltInFunc(gt));
    lisp_environment.get_global_frame().insert("<=".to_string(), Symbol::BuiltInFunc(le));
    lisp_environment.get_global_frame().insert("<".to_string(), Symbol::BuiltInFunc(lt));
    lisp_environment.get_global_frame().insert("*".to_string(), Symbol::BuiltInFunc(mul));
    lisp_environment.get_global_frame().insert("!=".to_string(), Symbol::BuiltInFunc(ne));
    lisp_environment.get_global_frame().insert("!".to_string(), Symbol::BuiltInFunc(not));
    lisp_environment.get_global_frame().insert("%".to_string(), Symbol::BuiltInFunc(rem));
    lisp_environment.get_global_frame().insert("<<".to_string(), Symbol::BuiltInFunc(shl));
    lisp_environment.get_global_frame().insert(">>".to_string(), Symbol::BuiltInFunc(shr));
    lisp_environment.get_global_frame().insert("-".to_string(), Symbol::BuiltInFunc(sub));
}
