pub mod eval;
pub mod rules;

#[cfg(test)]
mod tests {
    use crate::parser::{Expression, ColumnRef, SelectStatement};
    use crate::exec::eval::{evaluate_expression, EvalValue};
    use crate::exec::rules::{check_execution_rules, ExecIssue};

    #[test]
    fn test_eval_constant_folding() {
        // 1 + 2
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::Number("1".to_string())),
            operator: "+".to_string(),
            right: Box::new(Expression::Number("2".to_string())),
        };
        assert_eq!(evaluate_expression(&expr), EvalValue::Number(3));
    }
    
    #[test]
    fn test_eval_divide_by_zero() {
        // x / 0
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::Column(ColumnRef { table: None, name: "x".to_string() })),
            operator: "/".to_string(),
            right: Box::new(Expression::Number("0".to_string())),
        };
        assert_eq!(evaluate_expression(&expr), EvalValue::Unknown);
    }
    
    #[test]
    fn test_impossible_filter_warning() {
        // WHERE 1 = 0
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::Number("1".to_string())),
            operator: "=".to_string(),
            right: Box::new(Expression::Number("0".to_string())),
        };
        
        let ast = SelectStatement {
            columns: vec![],
            table: "t1".to_string(),
            joins: vec![],
            where_clause: Some(expr),
        };
        
        let issues = check_execution_rules(&ast);
        assert_eq!(issues.len(), 1);
        if let ExecIssue::Warning(msg) = &issues[0] {
            assert!(msg.contains("Impossible Filter"));
        } else {
            panic!("Expected warning");
        }
    }
    
    #[test]
    fn test_divide_by_zero_error() {
        // WHERE 1 / 0 = 1
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::Number("1".to_string())),
                operator: "/".to_string(),
                right: Box::new(Expression::Number("0".to_string())),
            }),
            operator: "=".to_string(),
            right: Box::new(Expression::Number("1".to_string())),
        };
        
        let ast = SelectStatement {
            columns: vec![],
            table: "t1".to_string(),
            joins: vec![],
            where_clause: Some(expr),
        };
        
        let issues = check_execution_rules(&ast);
        assert_eq!(issues.len(), 1);
        if let ExecIssue::Error(msg) = &issues[0] {
            assert!(msg.contains("Divide by Zero"));
        } else {
            panic!("Expected error");
        }
    }
}

