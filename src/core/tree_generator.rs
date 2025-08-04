use crate::structure::operator_tree::Operator;
use crate::structure::token::Token;

fn parse_binary<F>(
    tokens: &mut Vec<Token>,
    next: F,
    ops: &[Token],
    make_nodes: fn(Token, Operator<f64>, Operator<f64>) -> Operator<f64>,
) -> Operator<f64>
where
    F: Fn(&mut Vec<Token>) -> Operator<f64>,
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

pub fn parse_expression(tokens: &mut Vec<Token>) -> Operator<f64> {
    parse_boolean(tokens)
}
fn parse_boolean(token: &mut Vec<Token>) -> Operator<f64> {
    parse_binary(
        token,
        parse_add_sub,
        &[
            Token::And,
            Token::Or,
            Token::Equals,
            Token::NotEqual,
            Token::Not,
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual
        ],
        |op, left, right| match op {
            Token::And => Operator::And(Box::new(left), Box::new(right)),
            Token::Or => Operator::Or(Box::new(left), Box::new(right)),
            Token::Equals => Operator::Equals(Box::new(left), Box::new(right)),
            Token::NotEqual => Operator::NotEquals(Box::new(left), Box::new(right)),
            Token::Greater => Operator::Greater(Box::new(left), Box::new(right)),
            Token::GreaterEqual => Operator::Greater(Box::new(left), Box::new(right)),
            Token::Less => Operator::Greater(Box::new(left), Box::new(right)),
            Token::LessEqual => Operator::Greater(Box::new(left), Box::new(right)),
            _ => unreachable!(),
        },
    )
}

fn parse_add_sub(token: &mut Vec<Token>) -> Operator<f64> {
    parse_binary(
        token,
        parse_mul_div,
        &[Token::Plus, Token::Minus],
        |op, left, right| match op {
            Token::Plus => Operator::Sum(Box::new(left), Box::new(right)),
            Token::Minus => Operator::Subtract(Box::new(left), Box::new(right)),
            _ => unreachable!(),
        },
    )
}

fn parse_mul_div(tokens: &mut Vec<Token>) -> Operator<f64> {
    parse_binary(
        tokens,
        parse_primary,
        &[Token::Multiply, Token::Divide],
        |op, left, right| match op {
            Token::Multiply => Operator::Multiply(Box::new(left), Box::new(right)),
            Token::Divide => Operator::Division(Box::new(left), Box::new(right)),
            _ => unreachable!(),
        },
    )
}

fn parse_primary(tokens: &mut Vec<Token>) -> Operator<f64> {
    match tokens.remove(0) {
        Token::Number(n) => Operator::Val(n),
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
