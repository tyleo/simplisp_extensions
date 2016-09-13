macro_rules! math_op_2 {
    ($fn_name: ident, $num_type: ty, $lisp_type: ident, $op: ident, $failure: expr) => {
        unsafe fn $fn_name<'a, TIter, TEnvironment>(
            environment: &TEnvironment,
            lisp_environment: &mut LispEnvironment<TEnvironment>,
            first: $num_type,
            rest: TIter) -> error::Result<ExecutionTreeObject>
            where TIter: Iterator<Item = &'a ExecutionTreeObject> {
            let mut result = first;
            for arg in rest {
                let evaluated_arg = lisp_environment.evaluate(environment, &arg).unwrap();
                match evaluated_arg {
                    ExecutionTreeObject::$lisp_type(value) => result = result.$op(value),
                    _ => $failure,
                }
            }

            Ok(ExecutionTreeObject::$lisp_type(result))
        }
    }
}
