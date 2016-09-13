use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeObject;
use simplisp::Result as LispResult;
use simplisp::WrapError;
use std::cmp::PartialEq;

pub unsafe fn ne<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "ne".to_string();
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
            (ExecutionTreeObject::Bool(first), ExecutionTreeObject::Bool(second)) => first.ne(&second),
            (ExecutionTreeObject::F32(first), ExecutionTreeObject::F32(second)) => first.ne(&second),
            (ExecutionTreeObject::F64(first), ExecutionTreeObject::F64(second)) => first.ne(&second),
            (ExecutionTreeObject::I8(first), ExecutionTreeObject::I8(second)) => first.ne(&second),
            (ExecutionTreeObject::I16(first), ExecutionTreeObject::I16(second)) => first.ne(&second),
            (ExecutionTreeObject::I32(first), ExecutionTreeObject::I32(second)) => first.ne(&second),
            (ExecutionTreeObject::I64(first), ExecutionTreeObject::I64(second)) => first.ne(&second),
            (ExecutionTreeObject::ISize(first), ExecutionTreeObject::ISize(second)) => first.ne(&second),
            (ExecutionTreeObject::String(first), ExecutionTreeObject::String(second)) => first.ne(&second),
            (ExecutionTreeObject::U8(first), ExecutionTreeObject::U8(second)) => first.ne(&second),
            (ExecutionTreeObject::U16(first), ExecutionTreeObject::U16(second)) => first.ne(&second),
            (ExecutionTreeObject::U32(first), ExecutionTreeObject::U32(second)) => first.ne(&second),
            (ExecutionTreeObject::U64(first), ExecutionTreeObject::U64(second)) => first.ne(&second),
            (ExecutionTreeObject::USize(first), ExecutionTreeObject::USize(second)) => first.ne(&second),
            _ => {
                let err: Error = ErrorKind::Msg("Value cannot be compared.".to_string()).into();
                return err.wrap_error_to_err();
            }
        };

    Ok(ExecutionTreeObject::Bool(result))
}
