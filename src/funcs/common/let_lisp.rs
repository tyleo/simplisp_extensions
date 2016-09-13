use error::*;
use simplisp::Environment as LispEnvironment;
use simplisp::ExecutionTreeNode;
use simplisp::ExecutionTreeObject;
use simplisp::Frame;
use simplisp::Result as LispResult;
use simplisp::Symbol;
use simplisp::WrapError;

pub unsafe fn let_lisp<TEnvironment>(
    environment: &TEnvironment,
    lisp_environment: &mut LispEnvironment<TEnvironment>,
    args: Vec<&ExecutionTreeObject>) -> LispResult<ExecutionTreeObject> {
    let func_name = "let".to_string();
    let num_args = 1;
    let arg_index = 0;

    let mut rest = args.into_iter();
    let first_arg = {
        match rest.next() {
            Some(first_arg) => first_arg,
            None => {
                let err: Error = ErrorKind::LispArgNotFound(func_name, arg_index, num_args).into();
                return err.wrap_error_to_err();
            },
        }
    };

    let vars =
        match first_arg {
            &ExecutionTreeObject::Node(ref first) => {
                let mut result = Vec::new();
                for object in first.get_objects() {
                    match object {
                        &ExecutionTreeObject::Node(ref inner) => {
                            let mut inner_objects = inner.get_objects().into_iter();
                            let symbol =
                                match inner_objects.next() {
                                    Some(&ExecutionTreeObject::Symbol(ref symbol)) => {
                                        symbol.clone()
                                    },
                                    _ => {
                                        let err: Error = ErrorKind::Msg("Let binding item does not start with a symbol string.".to_string()).into();
                                        return err.wrap_error_to_err();
                                    },
                                };

                            let value =
                                match inner_objects.next() {
                                    Some(inner_object) => {
                                        Symbol::Object(try!(lisp_environment.evaluate(environment, &inner_object)))
                                    },
                                    _ => {
                                        let err: Error = ErrorKind::Msg("Let binding item does not end with a symbol value.".to_string()).into();
                                        return err.wrap_error_to_err();
                                    },
                                };

                            result.push((symbol, value));
                        },
                        _ => {
                            let err: Error = ErrorKind::Msg("Let binding item is not a symbol value pair.".to_string()).into();
                            return err.wrap_error_to_err();
                        },
                    }
                }
                result
            },
            arg => {
                let err: Error =
                    ErrorKind::LispArgIncorrectType(
                        func_name,
                        arg_index,
                        arg.enum_to_string().to_string(),
                        ExecutionTreeObject::string_str().to_string(),
                    ).into();
                return err.wrap_error_to_err();
            },
        };

    let mut new_frame = Frame::new();
    for (symbol, value) in vars {
        new_frame.insert(symbol, value);
    }

    let result =
        lisp_environment.with_frame(
            new_frame,
            |lisp_environment| -> LispResult<ExecutionTreeObject> {
                let mut result = Vec::new();
                for object in rest {
                    result.push(try!(lisp_environment.evaluate(environment, &object)));
                }
                Ok(ExecutionTreeObject::Node(ExecutionTreeNode::new(result)))
            }
        );

    result
}
