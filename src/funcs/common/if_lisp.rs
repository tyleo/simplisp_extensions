use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeObject;
use simplisp::Result as LispResult;
use simplisp::WrapError;

pub unsafe fn if_lisp<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "if".to_string();
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

    let third_arg = {
        match rest.next() {
            Some(third_arg) => try!(lisp_environment.evaluate(environment, &third_arg)),
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
        ExecutionTreeObject::Bool(first) => {
            if first {
                lisp_environment.evaluate(environment, &second_arg)
            } else {
                lisp_environment.evaluate(environment, &third_arg)
            }
        },
        _ => {
            let err: Error = ErrorKind::Msg("Cannot branch on if. First argument must be a bool.".to_string()).into();
            err.wrap_error_to_err()
        }
    }
}
