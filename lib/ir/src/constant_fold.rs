use syntax::{
    token::{BinaryOperator, UnaryOperator},
    tree::{Expression, Statement},
};

pub fn statement(statement: Statement) -> Statement {
    match statement {
        Statement::Let {
            identifier,
            mutable,
            value,
        } => {
            let value = expression(value);
            Statement::Let {
                identifier,
                mutable,
                value,
            }
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
        Expression::Unary(ref operator, ref value) => {
            let value = self::expression(*value.clone());
            if let Expression::Integer(value) = value {
                match operator {
                    UnaryOperator::Negate => Expression::Integer(-value),
                    _ => todo!(),
                }
            } else {
                expression
            }
        }
        Expression::If {
            condition,
            true_,
            false_,
        } => {
            let condition = self::expression(*condition);
            let true_ = self::expression(*true_);
            let false_ = self::expression(*false_.unwrap());
            if let Expression::Boolean(condition) = condition {
                if condition {
                    true_
                } else {
                    false_
                }
            } else {
                Expression::If {
                    condition: Box::new(condition),
                    true_: Box::new(true_),
                    false_: Some(Box::new(false_)),
                }
            }
        }
        expression => expression,
    }
}
