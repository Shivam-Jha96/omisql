use crate::parser::Expression;

#[derive(Debug, PartialEq, Clone)]
pub enum EvalValue {
    Number(i64),
    Boolean(bool),
    Unknown,
}

pub fn evaluate_expression(expr: &Expression) -> EvalValue {
    match expr {
        Expression::Number(n) => {
            if let Ok(val) = n.parse::<i64>() {
                EvalValue::Number(val)
            } else {
                EvalValue::Unknown
            }
        }
        Expression::Column(_) => EvalValue::Unknown,
        Expression::BinaryOp { left, operator, right } => {
            let l_val = evaluate_expression(left);
            let r_val = evaluate_expression(right);

            match (l_val, r_val) {
                (EvalValue::Number(l), EvalValue::Number(r)) => {
                    match operator.as_str() {
                        "+" => EvalValue::Number(l + r),
                        "-" => EvalValue::Number(l - r),
                        "*" => EvalValue::Number(l * r),
                        "/" => {
                            if r == 0 {
                                // Division by zero handled in rules, but we return Unknown here
                                EvalValue::Unknown
                            } else {
                                EvalValue::Number(l / r)
                            }
                        }
                        "=" => EvalValue::Boolean(l == r),
                        "!=" => EvalValue::Boolean(l != r),
                        ">" => EvalValue::Boolean(l > r),
                        "<" => EvalValue::Boolean(l < r),
                        ">=" => EvalValue::Boolean(l >= r),
                        "<=" => EvalValue::Boolean(l <= r),
                        _ => EvalValue::Unknown,
                    }
                }
                (EvalValue::Boolean(l), EvalValue::Boolean(r)) => {
                    match operator.to_uppercase().as_str() {
                        "AND" => EvalValue::Boolean(l && r),
                        "OR" => EvalValue::Boolean(l || r),
                        _ => EvalValue::Unknown,
                    }
                }
                _ => EvalValue::Unknown,
            }
        }
    }
}

