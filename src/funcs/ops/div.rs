use error;
use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeObject;
use simplisp::Result as LispResult;
use simplisp::WrapErr;
use simplisp::WrapError;
use std::ops::Div;

pub unsafe fn div<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "div".to_string();
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
            ExecutionTreeObject::F32(first) => div_f32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::F64(first) => div_f64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I8(first) => div_i8(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I16(first) => div_i16(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I32(first) => div_i32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I64(first) => div_i64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::ISize(first) => div_isize(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U8(first) => div_u8(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U16(first) => div_u16(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U32(first) => div_u32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U64(first) => div_u64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::USize(first) => div_usize(environment, lisp_environment, first, rest),
            _ =>  Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()),
        };

    result.wrap_err_to_err()
}

math_op_2!(div_f32, f32, F32, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
math_op_2!(div_f64, f64, F64, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
math_op_2!(div_i8, i8, I8, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
math_op_2!(div_i16, i16, I16, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
math_op_2!(div_i32, i32, I32, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
math_op_2!(div_i64, i64, I64, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
math_op_2!(div_isize, isize, ISize, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
math_op_2!(div_u8, u8, U8, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
math_op_2!(div_u16, u16, U16, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
math_op_2!(div_u32, u32, U32, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
math_op_2!(div_u64, u64, U64, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
math_op_2!(div_usize, usize, USize, div, { return Err(ErrorKind::Msg("Value cannot be dived.".to_string()).into()); });
