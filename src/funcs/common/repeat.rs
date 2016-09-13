use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeNode;
use simplisp::ExecutionTreeObject;
use simplisp::Result as LispResult;
use simplisp::WrapError;

pub unsafe fn repeat<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "repeat".to_string();
    let num_args = 3;
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

    match first_arg {
        ExecutionTreeObject::I8(first) => repeat_internal(environment, lisp_environment, first as u64, second_arg),
        ExecutionTreeObject::I16(first) => repeat_internal(environment, lisp_environment, first as u64, second_arg),
        ExecutionTreeObject::I32(first) => repeat_internal(environment, lisp_environment, first as u64, second_arg),
        ExecutionTreeObject::I64(first) => repeat_internal(environment, lisp_environment, first as u64, second_arg),
        ExecutionTreeObject::ISize(first) => repeat_internal(environment, lisp_environment, first as u64, second_arg),
        ExecutionTreeObject::U8(first) => repeat_internal(environment, lisp_environment, first as u64, second_arg),
        ExecutionTreeObject::U16(first) => repeat_internal(environment, lisp_environment, first as u64, second_arg),
        ExecutionTreeObject::U32(first) => repeat_internal(environment, lisp_environment, first as u64, second_arg),
        ExecutionTreeObject::U64(first) => repeat_internal(environment, lisp_environment, first as u64, second_arg),
        ExecutionTreeObject::USize(first) => repeat_internal(environment, lisp_environment, first as u64, second_arg),
        _ => {
            let err: Error = ErrorKind::Msg("Cannot repeat. Try passing an integer then an expression.".to_string()).into();
            err.wrap_error_to_err()
        }
    }
}

unsafe fn repeat_internal<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    repeats: u64,
    expression: ExecutionTreeObject) -> LispResult<ExecutionTreeObject> {
    let mut result = Vec::new();
    for _ in 0..repeats {
        result.push(try!(lisp_environment.evaluate(environment, &expression)));
    }
    Ok(ExecutionTreeObject::Node(ExecutionTreeNode::new(result)))
}
