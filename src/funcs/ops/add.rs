use error;
use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeObject;
use simplisp::Result as LispResult;
use simplisp::WrapErr;
use simplisp::WrapError;
use std::ops::Add;

pub unsafe fn add<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "add".to_string();
    let num_args = 1;
    let arg_index = 0;

    let mut rest = args.into_iter();
    let first_arg = {
        match rest.next() {
            Some(first_arg) => try!(lisp_environment.evaluate(environment, &first_arg)),
            None => {
                let err: Error = ErrorKind::LispArgNotFound(func_name, arg_index, num_args).into();
                return err.wrap_error_to_err();
            },
        }
    };

    let result =
        match first_arg {
            ExecutionTreeObject::F32(first) => add_f32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::F64(first) => add_f64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I8(first) => add_i8(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I16(first) => add_i16(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I32(first) => add_i32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I64(first) => add_i64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::ISize(first) => add_isize(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U8(first) => add_u8(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U16(first) => add_u16(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U32(first) => add_u32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U64(first) => add_u64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::USize(first) => add_usize(environment, lisp_environment, first, rest),
            _ =>  Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()),
        };

    result.wrap_err_to_err()
}

math_op_2!(add_f32, f32, F32, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
math_op_2!(add_f64, f64, F64, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
math_op_2!(add_i8, i8, I8, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
math_op_2!(add_i16, i16, I16, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
math_op_2!(add_i32, i32, I32, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
math_op_2!(add_i64, i64, I64, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
math_op_2!(add_isize, isize, ISize, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
math_op_2!(add_u8, u8, U8, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
math_op_2!(add_u16, u16, U16, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
math_op_2!(add_u32, u32, U32, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
math_op_2!(add_u64, u64, U64, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
math_op_2!(add_usize, usize, USize, add, { return Err(ErrorKind::Msg("Value cannot be added.".to_string()).into()); });
