use crate::parser::{Statement, Expression};
use crate::semantic::SchemaRegistry;
use crate::exec::eval::{evaluate_expression, EvalValue};

pub enum CostIssue {
    Error(String),
    Warning(String),
}

pub fn check_cost_rules(stmt: &Statement, registry: &SchemaRegistry) -> Vec<CostIssue> {
    let mut issues = Vec::new();
    
    if let Statement::Select(select) = stmt {
        // MissingPartitionFilter
        let mut all_queried_tables = vec![select.table.clone()];
        for join in &select.joins {
            all_queried_tables.push(join.table.clone());
        }

        for t in &all_queried_tables {
            if let Some(table_def) = registry.tables.get(t) {
                if let Some(ref partition_col) = table_def.partition_column {
                    // check if this partition_col is present in the where_clause
                    let has_filter = if let Some(ref expr) = select.where_clause {
                        expr_contains_column(expr, partition_col)
                    } else {
                        false
                    };

                    if !has_filter {
                        issues.push(CostIssue::Error(format!(
                            "MissingPartitionFilter: Table '{}' is partitioned by '{}', but no filter was provided.", t, partition_col
                        )));
                    }
                }
            }
        }

        // CartesianJoinWarning
        for join in &select.joins {
            let val = evaluate_expression(&join.on);
            let is_constant_truthy = match val {
                EvalValue::Number(n) if n != 0 => true,
                EvalValue::Boolean(b) if b => true,
                _ => false,
            };
            
            if is_constant_truthy {
                issues.push(CostIssue::Warning(format!(
                    "CartesianJoinWarning: JOIN ON condition for table '{}' evaluates to a truthy constant, which may cause a cartesian product.", join.table
                )));
            }
        }
    }
    
    issues
}

fn expr_contains_column(expr: &Expression, target_col: &str) -> bool {
    match expr {
        Expression::Column(col_ref) => col_ref.name == target_col,
        Expression::Number(_) => false,
        Expression::BinaryOp { left, right, .. } => {
            expr_contains_column(left, target_col) || expr_contains_column(right, target_col)
        }
    }
}
