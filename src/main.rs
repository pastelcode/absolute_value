use evalexpr::eval;
use std::env;

struct Expression {
    input: String,
    evaluated: f64,
}

fn main() {
    // let  numbers = env::args().collect::<Vec<String>>();
    let expressions = env::args()
        .map(|arg| -> Option<Expression> {
            match eval(arg.as_str()) {
                Ok(evaluated) => {
                    if let Ok(integer) = evaluated.as_int() {
                        let parsed_to_float = integer as f64;
                        Some(Expression {
                            input: arg,
                            evaluated: parsed_to_float,
                        })
                    } else if let Ok(float) = evaluated.as_float() {
                        Some(Expression {
                            input: arg,
                            evaluated: float,
                        })
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        })
        .collect::<Vec<Option<Expression>>>();

    for expression_option in expressions {
        if let Some(expression) = expression_option {
            println!(
                "| {} | -> {}",
                expression.input,
                determine_absolute_value(expression.evaluated)
            );
        }
    }
}

fn determine_absolute_value(number: f64) -> f64 {
    if number >= 0.0 {
        number
    } else {
        number * -1.0
    }
}
