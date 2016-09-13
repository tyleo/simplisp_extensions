use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeObject;
use simplisp::Result as LispResult;
use simplisp::WrapError;
use std::ops::Not;

pub unsafe fn not<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "not".to_string();
    let num_args = 1;
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

    if let Some(_) = rest.next() {
        let err: Error = ErrorKind::LispExtraArgFound(func_name, num_args, arg_index + 1).into();
        return err.wrap_error_to_err();
    }

    let result =
        match first_arg {
            ExecutionTreeObject::Bool(first) => ExecutionTreeObject::Bool(first.not()),
            ExecutionTreeObject::I8(first) => ExecutionTreeObject::I8(first.not()),
            ExecutionTreeObject::I16(first) => ExecutionTreeObject::I16(first.not()),
            ExecutionTreeObject::I32(first) => ExecutionTreeObject::I32(first.not()),
            ExecutionTreeObject::I64(first) => ExecutionTreeObject::I64(first.not()),
            ExecutionTreeObject::ISize(first) => ExecutionTreeObject::ISize(first.not()),
            ExecutionTreeObject::U8(first) => ExecutionTreeObject::U8(first.not()),
            ExecutionTreeObject::U16(first) => ExecutionTreeObject::U16(first.not()),
            ExecutionTreeObject::U32(first) => ExecutionTreeObject::U32(first.not()),
            ExecutionTreeObject::U64(first) => ExecutionTreeObject::U64(first.not()),
            ExecutionTreeObject::USize(first) => ExecutionTreeObject::USize(first.not()),
            _ => {
                let err: Error = ErrorKind::Msg("Value cannot be inverted.".to_string()).into();
                return err.wrap_error_to_err();
            },
        };

    Ok(result)
}
