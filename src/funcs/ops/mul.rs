use error;
use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeObject;
use simplisp::Result as LispResult;
use simplisp::WrapErr;
use simplisp::WrapError;
use std::ops::Mul;

pub unsafe fn mul<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "mul".to_string();
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
            ExecutionTreeObject::F32(first) => mul_f32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::F64(first) => mul_f64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I8(first) => mul_i8(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I16(first) => mul_i16(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I32(first) => mul_i32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I64(first) => mul_i64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::ISize(first) => mul_isize(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U8(first) => mul_u8(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U16(first) => mul_u16(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U32(first) => mul_u32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U64(first) => mul_u64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::USize(first) => mul_usize(environment, lisp_environment, first, rest),
            _ =>  Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()),
        };

    result.wrap_err_to_err()
}

math_op_2!(mul_f32, f32, F32, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
math_op_2!(mul_f64, f64, F64, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
math_op_2!(mul_i8, i8, I8, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
math_op_2!(mul_i16, i16, I16, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
math_op_2!(mul_i32, i32, I32, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
math_op_2!(mul_i64, i64, I64, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
math_op_2!(mul_isize, isize, ISize, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
math_op_2!(mul_u8, u8, U8, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
math_op_2!(mul_u16, u16, U16, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
math_op_2!(mul_u32, u32, U32, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
math_op_2!(mul_u64, u64, U64, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
math_op_2!(mul_usize, usize, USize, mul, { return Err(ErrorKind::Msg("Value cannot be muled.".to_string()).into()); });
