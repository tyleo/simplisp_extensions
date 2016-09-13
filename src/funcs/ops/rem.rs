use error;
use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeObject;
use simplisp::Result as LispResult;
use simplisp::WrapErr;
use simplisp::WrapError;
use std::ops::Rem;

pub unsafe fn rem<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "rem".to_string();
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
            ExecutionTreeObject::F32(first) => rem_f32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::F64(first) => rem_f64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I8(first) => rem_i8(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I16(first) => rem_i16(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I32(first) => rem_i32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I64(first) => rem_i64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::ISize(first) => rem_isize(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U8(first) => rem_u8(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U16(first) => rem_u16(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U32(first) => rem_u32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U64(first) => rem_u64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::USize(first) => rem_usize(environment, lisp_environment, first, rest),
            _ => Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()),
        };

    result.wrap_err_to_err()
}

math_op_2!(rem_f32, f32, F32, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
math_op_2!(rem_f64, f64, F64, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
math_op_2!(rem_i8, i8, I8, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
math_op_2!(rem_i16, i16, I16, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
math_op_2!(rem_i32, i32, I32, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
math_op_2!(rem_i64, i64, I64, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
math_op_2!(rem_isize, isize, ISize, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
math_op_2!(rem_u8, u8, U8, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
math_op_2!(rem_u16, u16, U16, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
math_op_2!(rem_u32, u32, U32, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
math_op_2!(rem_u64, u64, U64, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
math_op_2!(rem_usize, usize, USize, rem, { return Err(ErrorKind::Msg("Value cannot be remed.".to_string()).into()); });
