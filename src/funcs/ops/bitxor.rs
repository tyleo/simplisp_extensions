use error;
use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeObject;
use simplisp::Result as LispResult;
use simplisp::WrapErr;
use simplisp::WrapError;
use std::ops::BitXor;

pub unsafe fn bitxor<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "bitxor".to_string();
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
            ExecutionTreeObject::Bool(first) => bitxor_bool(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I8(first) => bitxor_i8(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I16(first) => bitxor_i16(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I32(first) => bitxor_i32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::I64(first) => bitxor_i64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::ISize(first) => bitxor_isize(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U8(first) => bitxor_u8(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U16(first) => bitxor_u16(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U32(first) => bitxor_u32(environment, lisp_environment, first, rest),
            ExecutionTreeObject::U64(first) => bitxor_u64(environment, lisp_environment, first, rest),
            ExecutionTreeObject::USize(first) => bitxor_usize(environment, lisp_environment, first, rest),
            _ => Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()),
        };

    result.wrap_err_to_err()
}

math_op_2!(bitxor_bool, bool, Bool, bitxor, { return Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()); });
math_op_2!(bitxor_i8, i8, I8, bitxor, { return Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()); });
math_op_2!(bitxor_i16, i16, I16, bitxor, { return Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()); });
math_op_2!(bitxor_i32, i32, I32, bitxor, { return Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()); });
math_op_2!(bitxor_i64, i64, I64, bitxor, { return Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()); });
math_op_2!(bitxor_isize, isize, ISize, bitxor, { return Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()); });
math_op_2!(bitxor_u8, u8, U8, bitxor, { return Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()); });
math_op_2!(bitxor_u16, u16, U16, bitxor, { return Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()); });
math_op_2!(bitxor_u32, u32, U32, bitxor, { return Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()); });
math_op_2!(bitxor_u64, u64, U64, bitxor, { return Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()); });
math_op_2!(bitxor_usize, usize, USize, bitxor, { return Err(ErrorKind::Msg("Value cannot be bitxored.".to_string()).into()); });
