use crate::structure::calculus_structure::Expression;
use crate::structure::token::Token;

fn parse_binary<F>(
    tokens: &mut Vec<Token>,
    next: F,
    ops: &[Token],
    make_nodes: fn(Token, Expression, Expression) -> Expression,
) -> Expression
where
    F: Fn(&mut Vec<Token>) -> Expression,
{
    let mut node = next(tokens);

    while let Some(token) = tokens.first() {
        if ops.contains(token) {
            let operator = tokens.remove(0);
            let right = next(tokens);
            node = make_nodes(operator, node, right);
        } else {
            break;
        }
    }

    node
}

fn parse_unary(tokens: &mut Vec<Token>) -> Expression {
    if let Some(token) = tokens.first() {
        match token {
            Token::Not => {
                tokens.remove(0);
                let expression = parse_unary(tokens);
                Expression::Unary {op:Token::Not, expr: Box::new(expression)}
            }
            _ => parse_primary(tokens),
        }
    } else {
        panic!("Unexpected At Unary Parsing!!!")
    }
}

pub fn parse_expression(tokens: &mut Vec<Token>) -> Expression {
    parse_boolean(tokens)
}
fn parse_boolean(token: &mut Vec<Token>) -> Expression {
    parse_binary(
        token,
        parse_add_sub,
        &[
            Token::And,
            Token::Or,
            Token::Equals,
            Token::NotEqual,
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
            Token::Convert
        ],
        |op, left, right| match op {
            Token::And => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::Or => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::Equals => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::NotEqual => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::Greater => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::GreaterEqual => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::Less => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::LessEqual => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::Convert => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            _ => unreachable!(),
        },
    )
}

fn parse_add_sub(token: &mut Vec<Token>) -> Expression {
    parse_binary(
        token,
        parse_mul_div,
        &[Token::Plus, Token::Minus],
        |op, left, right| match op {
            Token::Plus => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::Minus => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            _ => unreachable!(),
        },
    )
}

fn parse_mul_div(tokens: &mut Vec<Token>) -> Expression {
    parse_binary(
        tokens,
        parse_unary,
        &[Token::Multiply, Token::Divide],
        |op, left, right| match op {
            Token::Multiply => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::Divide => Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            _ => unreachable!(),
        },
    )
}

fn parse_primary(tokens: &mut Vec<Token>) -> Expression {
    match tokens.remove(0) {
        Token::Number(n) => Expression::Number(n),
        Token::Boolean(b) => Expression::Boolean(b),
        Token::LParenthesis => {
            let node = parse_expression(tokens);
            match tokens.remove(0) {
                Token::RParenthesis => node,
                _ => panic!("Expected closing parenthesis"),
            }
        }
        other => panic!("Unexpected token: {:?}", other),
    }
}
