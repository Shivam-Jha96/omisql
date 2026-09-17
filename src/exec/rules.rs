use crate::parser::{Expression, SelectStatement};
use crate::exec::eval::{evaluate_expression, EvalValue};

pub enum ExecIssue {
    Error(String),
    Warning(String),
}

pub fn check_execution_rules(ast: &SelectStatement) -> Vec<ExecIssue> {
    let mut issues = Vec::new();

    // Check where clause for Impossible Filters and traverse for Divide by Zero
    if let Some(ref expr) = ast.where_clause {
        // Evaluate the condition
        let val = evaluate_expression(expr);
        if let EvalValue::Boolean(false) = val {
            issues.push(ExecIssue::Warning("Impossible Filter: WHERE clause statically evaluates to FALSE".to_string()));
        }
        
        // Traverse to find divide by zero
        check_expr_rules(expr, &mut issues);
    }
    
    for join in &ast.joins {
        check_expr_rules(&join.on, &mut issues);
    }

    issues
}

fn check_expr_rules(expr: &Expression, issues: &mut Vec<ExecIssue>) {
    match expr {
        Expression::BinaryOp { left, operator, right } => {
            if operator == "/" {
                let r_val = evaluate_expression(right);
                if let EvalValue::Number(0) = r_val {
                    issues.push(ExecIssue::Error("Divide by Zero: Attempted to divide by literal 0".to_string()));
                }
            }
            check_expr_rules(left, issues);
            check_expr_rules(right, issues);
        }
        _ => {}
    }
}

