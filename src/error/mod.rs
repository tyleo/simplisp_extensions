error_chain! {
    types { }

    links { }

    foreign_links { }

    errors {
        LispArgNotFound(func: String, current_arg_index: usize, number_of_args: usize) {
            description("Error calling lisp func. Not enough args supplied.")
            display(
                "{}{}{}{}{}{}{}",
                "Error calling lisp func. Not enough args supplied. The func, '",
                func,
                "' requires, '",
                number_of_args,
                "', args, only, '",
                current_arg_index,
                "', were given.",
            )
        }

        LispExtraArgFound(func: String, current_arg_index: usize, number_of_args: usize) {
            description("Error calling lisp func. Too many args supplied.")
            display(
                "{}{}{}{}{}{}{}",
                "Error calling lisp func. Too many args supplied. The func, '",
                func,
                "' only requires, '",
                number_of_args,
                "', args, but, '",
                current_arg_index,
                "', were given.",
            )
        }

        LispArgIncorrectType(func: String, arg_position: usize, current_type: String, required_type: String) {
            description("Error calling lisp func. Arg is of incorrect type.")
            display(
                "{}{}{}{}{}{}{}{}{}",
                "Error calling lisp func, '",
                func,
                "'. The arg in position, '",
                arg_position,
                "', is of incorrect type. The requried type is, '",
                required_type,
                "', but the given type is, '",
                current_type,
                "'.",
            )
        }
    }
}
