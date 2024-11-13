use syntax::{token::BinaryOperator, tree::{Expression, Statement}};

pub fn statement(statement: Statement) -> Statement {
    match statement {
        Statement::Let { identifier, mutable, value } => {
            let value = expression(value);
            Statement::Let { identifier, mutable, value }
        }
        _ => todo!(),
    }
}

pub fn expression(expression: Expression) -> Expression {
    match expression {
        Expression::Binary(ref operator, ref left, ref right) => {
            let left = self::expression(*left.clone());
            let right = self::expression(*right.clone());
            if let (Expression::Integer(left), Expression::Integer(right)) = (left, right) {
                match operator {
                    BinaryOperator::Add => Expression::Integer(left + right),
                    BinaryOperator::Subtract => Expression::Integer(left - right),
                    BinaryOperator::Multiply => Expression::Integer(left * right),
                    BinaryOperator::Divide => Expression::Integer(left / right),
                    BinaryOperator::Greater => Expression::Boolean(left > right),
                    BinaryOperator::Less => Expression::Boolean(left < right),
                    BinaryOperator::Equal => Expression::Boolean(left == right),
                }
            } else {
                expression
            }
        }
        Expression::Unary(operator, value) => todo!(),
        Expression::If { condition, true_, false_ } => todo!(),
        expression => expression,
    }
}
