use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeObject;
use simplisp::Result as LispResult;
use simplisp::WrapError;

pub unsafe fn isize<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "isize".to_string();
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
            ExecutionTreeObject::I8(first) => first as isize,
            ExecutionTreeObject::I16(first) => first as isize,
            ExecutionTreeObject::I32(first) => first as isize,
            ExecutionTreeObject::I64(first) => first as isize,
            ExecutionTreeObject::ISize(first) => first as isize,
            ExecutionTreeObject::U8(first) => first as isize,
            ExecutionTreeObject::U16(first) => first as isize,
            ExecutionTreeObject::U32(first) => first as isize,
            ExecutionTreeObject::U64(first) => first as isize,
            ExecutionTreeObject::USize(first) => first as isize,
            _ =>  {
                let err: Error = ErrorKind::Msg("Value cannot be converted to isize.".to_string()).into();
                return err.wrap_error_to_err();
            },
        };

    Ok(ExecutionTreeObject::ISize(result))
}
