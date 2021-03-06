use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeObject;
use simplisp::Result as LispResult;
use simplisp::WrapError;
use std::ops::Shl;

pub unsafe fn shl<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "shl".to_string();
    let num_args = 2;
    let mut arg_index = 0;

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

    arg_index += 1;

    let second_arg = {
        match rest.next() {
            Some(second_arg) => try!(lisp_environment.evaluate(environment, &second_arg)),
            None => {
                let err: Error = ErrorKind::LispArgNotFound(func_name, arg_index, num_args).into();
                return err.wrap_error_to_err();
            },
        }
    };

    arg_index += 1;

    if let Some(_) = rest.next() {
        let err: Error = ErrorKind::LispExtraArgFound(func_name, num_args, arg_index + 1).into();
        return err.wrap_error_to_err();
    }

    let result =
        match (first_arg, second_arg) {
            (ExecutionTreeObject::I8(first), ExecutionTreeObject::I8(second)) => ExecutionTreeObject::I8(first.shl(second)),
            (ExecutionTreeObject::I8(first), ExecutionTreeObject::I16(second)) => ExecutionTreeObject::I8(first.shl(second)),
            (ExecutionTreeObject::I8(first), ExecutionTreeObject::I32(second)) => ExecutionTreeObject::I8(first.shl(second)),
            (ExecutionTreeObject::I8(first), ExecutionTreeObject::I64(second)) => ExecutionTreeObject::I8(first.shl(second)),
            (ExecutionTreeObject::I8(first), ExecutionTreeObject::ISize(second)) => ExecutionTreeObject::I8(first.shl(second)),
            (ExecutionTreeObject::I8(first), ExecutionTreeObject::U8(second)) => ExecutionTreeObject::I8(first.shl(second)),
            (ExecutionTreeObject::I8(first), ExecutionTreeObject::U16(second)) => ExecutionTreeObject::I8(first.shl(second)),
            (ExecutionTreeObject::I8(first), ExecutionTreeObject::U32(second)) => ExecutionTreeObject::I8(first.shl(second)),
            (ExecutionTreeObject::I8(first), ExecutionTreeObject::U64(second)) => ExecutionTreeObject::I8(first.shl(second)),
            (ExecutionTreeObject::I8(first), ExecutionTreeObject::USize(second)) => ExecutionTreeObject::I8(first.shl(second)),
            (ExecutionTreeObject::I16(first), ExecutionTreeObject::I8(second)) => ExecutionTreeObject::I16(first.shl(second)),
            (ExecutionTreeObject::I16(first), ExecutionTreeObject::I16(second)) => ExecutionTreeObject::I16(first.shl(second)),
            (ExecutionTreeObject::I16(first), ExecutionTreeObject::I32(second)) => ExecutionTreeObject::I16(first.shl(second)),
            (ExecutionTreeObject::I16(first), ExecutionTreeObject::I64(second)) => ExecutionTreeObject::I16(first.shl(second)),
            (ExecutionTreeObject::I16(first), ExecutionTreeObject::ISize(second)) => ExecutionTreeObject::I16(first.shl(second)),
            (ExecutionTreeObject::I16(first), ExecutionTreeObject::U8(second)) => ExecutionTreeObject::I16(first.shl(second)),
            (ExecutionTreeObject::I16(first), ExecutionTreeObject::U16(second)) => ExecutionTreeObject::I16(first.shl(second)),
            (ExecutionTreeObject::I16(first), ExecutionTreeObject::U32(second)) => ExecutionTreeObject::I16(first.shl(second)),
            (ExecutionTreeObject::I16(first), ExecutionTreeObject::U64(second)) => ExecutionTreeObject::I16(first.shl(second)),
            (ExecutionTreeObject::I16(first), ExecutionTreeObject::USize(second)) => ExecutionTreeObject::I16(first.shl(second)),
            (ExecutionTreeObject::I32(first), ExecutionTreeObject::I8(second)) => ExecutionTreeObject::I32(first.shl(second)),
            (ExecutionTreeObject::I32(first), ExecutionTreeObject::I16(second)) => ExecutionTreeObject::I32(first.shl(second)),
            (ExecutionTreeObject::I32(first), ExecutionTreeObject::I32(second)) => ExecutionTreeObject::I32(first.shl(second)),
            (ExecutionTreeObject::I32(first), ExecutionTreeObject::I64(second)) => ExecutionTreeObject::I32(first.shl(second)),
            (ExecutionTreeObject::I32(first), ExecutionTreeObject::ISize(second)) => ExecutionTreeObject::I32(first.shl(second)),
            (ExecutionTreeObject::I32(first), ExecutionTreeObject::U8(second)) => ExecutionTreeObject::I32(first.shl(second)),
            (ExecutionTreeObject::I32(first), ExecutionTreeObject::U16(second)) => ExecutionTreeObject::I32(first.shl(second)),
            (ExecutionTreeObject::I32(first), ExecutionTreeObject::U32(second)) => ExecutionTreeObject::I32(first.shl(second)),
            (ExecutionTreeObject::I32(first), ExecutionTreeObject::U64(second)) => ExecutionTreeObject::I32(first.shl(second)),
            (ExecutionTreeObject::I32(first), ExecutionTreeObject::USize(second)) => ExecutionTreeObject::I32(first.shl(second)),
            (ExecutionTreeObject::I64(first), ExecutionTreeObject::I8(second)) => ExecutionTreeObject::I64(first.shl(second)),
            (ExecutionTreeObject::I64(first), ExecutionTreeObject::I16(second)) => ExecutionTreeObject::I64(first.shl(second)),
            (ExecutionTreeObject::I64(first), ExecutionTreeObject::I32(second)) => ExecutionTreeObject::I64(first.shl(second)),
            (ExecutionTreeObject::I64(first), ExecutionTreeObject::I64(second)) => ExecutionTreeObject::I64(first.shl(second)),
            (ExecutionTreeObject::I64(first), ExecutionTreeObject::ISize(second)) => ExecutionTreeObject::I64(first.shl(second)),
            (ExecutionTreeObject::I64(first), ExecutionTreeObject::U8(second)) => ExecutionTreeObject::I64(first.shl(second)),
            (ExecutionTreeObject::I64(first), ExecutionTreeObject::U16(second)) => ExecutionTreeObject::I64(first.shl(second)),
            (ExecutionTreeObject::I64(first), ExecutionTreeObject::U32(second)) => ExecutionTreeObject::I64(first.shl(second)),
            (ExecutionTreeObject::I64(first), ExecutionTreeObject::U64(second)) => ExecutionTreeObject::I64(first.shl(second)),
            (ExecutionTreeObject::I64(first), ExecutionTreeObject::USize(second)) => ExecutionTreeObject::I64(first.shl(second)),
            (ExecutionTreeObject::ISize(first), ExecutionTreeObject::I8(second)) => ExecutionTreeObject::ISize(first.shl(second)),
            (ExecutionTreeObject::ISize(first), ExecutionTreeObject::I16(second)) => ExecutionTreeObject::ISize(first.shl(second)),
            (ExecutionTreeObject::ISize(first), ExecutionTreeObject::I32(second)) => ExecutionTreeObject::ISize(first.shl(second)),
            (ExecutionTreeObject::ISize(first), ExecutionTreeObject::I64(second)) => ExecutionTreeObject::ISize(first.shl(second)),
            (ExecutionTreeObject::ISize(first), ExecutionTreeObject::ISize(second)) => ExecutionTreeObject::ISize(first.shl(second)),
            (ExecutionTreeObject::ISize(first), ExecutionTreeObject::U8(second)) => ExecutionTreeObject::ISize(first.shl(second)),
            (ExecutionTreeObject::ISize(first), ExecutionTreeObject::U16(second)) => ExecutionTreeObject::ISize(first.shl(second)),
            (ExecutionTreeObject::ISize(first), ExecutionTreeObject::U32(second)) => ExecutionTreeObject::ISize(first.shl(second)),
            (ExecutionTreeObject::ISize(first), ExecutionTreeObject::U64(second)) => ExecutionTreeObject::ISize(first.shl(second)),
            (ExecutionTreeObject::ISize(first), ExecutionTreeObject::USize(second)) => ExecutionTreeObject::ISize(first.shl(second)),
            (ExecutionTreeObject::U8(first), ExecutionTreeObject::I8(second)) => ExecutionTreeObject::U8(first.shl(second)),
            (ExecutionTreeObject::U8(first), ExecutionTreeObject::I16(second)) => ExecutionTreeObject::U8(first.shl(second)),
            (ExecutionTreeObject::U8(first), ExecutionTreeObject::I32(second)) => ExecutionTreeObject::U8(first.shl(second)),
            (ExecutionTreeObject::U8(first), ExecutionTreeObject::I64(second)) => ExecutionTreeObject::U8(first.shl(second)),
            (ExecutionTreeObject::U8(first), ExecutionTreeObject::ISize(second)) => ExecutionTreeObject::U8(first.shl(second)),
            (ExecutionTreeObject::U8(first), ExecutionTreeObject::U8(second)) => ExecutionTreeObject::U8(first.shl(second)),
            (ExecutionTreeObject::U8(first), ExecutionTreeObject::U16(second)) => ExecutionTreeObject::U8(first.shl(second)),
            (ExecutionTreeObject::U8(first), ExecutionTreeObject::U32(second)) => ExecutionTreeObject::U8(first.shl(second)),
            (ExecutionTreeObject::U8(first), ExecutionTreeObject::U64(second)) => ExecutionTreeObject::U8(first.shl(second)),
            (ExecutionTreeObject::U8(first), ExecutionTreeObject::USize(second)) => ExecutionTreeObject::U8(first.shl(second)),
            (ExecutionTreeObject::U16(first), ExecutionTreeObject::I8(second)) => ExecutionTreeObject::U16(first.shl(second)),
            (ExecutionTreeObject::U16(first), ExecutionTreeObject::I16(second)) => ExecutionTreeObject::U16(first.shl(second)),
            (ExecutionTreeObject::U16(first), ExecutionTreeObject::I32(second)) => ExecutionTreeObject::U16(first.shl(second)),
            (ExecutionTreeObject::U16(first), ExecutionTreeObject::I64(second)) => ExecutionTreeObject::U16(first.shl(second)),
            (ExecutionTreeObject::U16(first), ExecutionTreeObject::ISize(second)) => ExecutionTreeObject::U16(first.shl(second)),
            (ExecutionTreeObject::U16(first), ExecutionTreeObject::U8(second)) => ExecutionTreeObject::U16(first.shl(second)),
            (ExecutionTreeObject::U16(first), ExecutionTreeObject::U16(second)) => ExecutionTreeObject::U16(first.shl(second)),
            (ExecutionTreeObject::U16(first), ExecutionTreeObject::U32(second)) => ExecutionTreeObject::U16(first.shl(second)),
            (ExecutionTreeObject::U16(first), ExecutionTreeObject::U64(second)) => ExecutionTreeObject::U16(first.shl(second)),
            (ExecutionTreeObject::U16(first), ExecutionTreeObject::USize(second)) => ExecutionTreeObject::U16(first.shl(second)),
            (ExecutionTreeObject::U32(first), ExecutionTreeObject::I8(second)) => ExecutionTreeObject::U32(first.shl(second)),
            (ExecutionTreeObject::U32(first), ExecutionTreeObject::I16(second)) => ExecutionTreeObject::U32(first.shl(second)),
            (ExecutionTreeObject::U32(first), ExecutionTreeObject::I32(second)) => ExecutionTreeObject::U32(first.shl(second)),
            (ExecutionTreeObject::U32(first), ExecutionTreeObject::I64(second)) => ExecutionTreeObject::U32(first.shl(second)),
            (ExecutionTreeObject::U32(first), ExecutionTreeObject::ISize(second)) => ExecutionTreeObject::U32(first.shl(second)),
            (ExecutionTreeObject::U32(first), ExecutionTreeObject::U8(second)) => ExecutionTreeObject::U32(first.shl(second)),
            (ExecutionTreeObject::U32(first), ExecutionTreeObject::U16(second)) => ExecutionTreeObject::U32(first.shl(second)),
            (ExecutionTreeObject::U32(first), ExecutionTreeObject::U32(second)) => ExecutionTreeObject::U32(first.shl(second)),
            (ExecutionTreeObject::U32(first), ExecutionTreeObject::U64(second)) => ExecutionTreeObject::U32(first.shl(second)),
            (ExecutionTreeObject::U32(first), ExecutionTreeObject::USize(second)) => ExecutionTreeObject::U32(first.shl(second)),
            (ExecutionTreeObject::U64(first), ExecutionTreeObject::I8(second)) => ExecutionTreeObject::U64(first.shl(second)),
            (ExecutionTreeObject::U64(first), ExecutionTreeObject::I16(second)) => ExecutionTreeObject::U64(first.shl(second)),
            (ExecutionTreeObject::U64(first), ExecutionTreeObject::I32(second)) => ExecutionTreeObject::U64(first.shl(second)),
            (ExecutionTreeObject::U64(first), ExecutionTreeObject::I64(second)) => ExecutionTreeObject::U64(first.shl(second)),
            (ExecutionTreeObject::U64(first), ExecutionTreeObject::ISize(second)) => ExecutionTreeObject::U64(first.shl(second)),
            (ExecutionTreeObject::U64(first), ExecutionTreeObject::U8(second)) => ExecutionTreeObject::U64(first.shl(second)),
            (ExecutionTreeObject::U64(first), ExecutionTreeObject::U16(second)) => ExecutionTreeObject::U64(first.shl(second)),
            (ExecutionTreeObject::U64(first), ExecutionTreeObject::U32(second)) => ExecutionTreeObject::U64(first.shl(second)),
            (ExecutionTreeObject::U64(first), ExecutionTreeObject::U64(second)) => ExecutionTreeObject::U64(first.shl(second)),
            (ExecutionTreeObject::U64(first), ExecutionTreeObject::USize(second)) => ExecutionTreeObject::U64(first.shl(second)),
            (ExecutionTreeObject::USize(first), ExecutionTreeObject::I8(second)) => ExecutionTreeObject::USize(first.shl(second)),
            (ExecutionTreeObject::USize(first), ExecutionTreeObject::I16(second)) => ExecutionTreeObject::USize(first.shl(second)),
            (ExecutionTreeObject::USize(first), ExecutionTreeObject::I32(second)) => ExecutionTreeObject::USize(first.shl(second)),
            (ExecutionTreeObject::USize(first), ExecutionTreeObject::I64(second)) => ExecutionTreeObject::USize(first.shl(second)),
            (ExecutionTreeObject::USize(first), ExecutionTreeObject::ISize(second)) => ExecutionTreeObject::USize(first.shl(second)),
            (ExecutionTreeObject::USize(first), ExecutionTreeObject::U8(second)) => ExecutionTreeObject::USize(first.shl(second)),
            (ExecutionTreeObject::USize(first), ExecutionTreeObject::U16(second)) => ExecutionTreeObject::USize(first.shl(second)),
            (ExecutionTreeObject::USize(first), ExecutionTreeObject::U32(second)) => ExecutionTreeObject::USize(first.shl(second)),
            (ExecutionTreeObject::USize(first), ExecutionTreeObject::U64(second)) => ExecutionTreeObject::USize(first.shl(second)),
            (ExecutionTreeObject::USize(first), ExecutionTreeObject::USize(second)) => ExecutionTreeObject::USize(first.shl(second)),
            _ => {
                let err: Error = ErrorKind::Msg("Value cannot be shifted.".to_string()).into();
                return err.wrap_error_to_err();
            }
        };

    Ok(result)
}
