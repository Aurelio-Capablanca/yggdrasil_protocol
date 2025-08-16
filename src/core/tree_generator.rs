use crate::structure::operator_tree::Operator;
use crate::structure::token::Token;
use crate::structure::calculus_structure::Expression;

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
            Token::Not,
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual
        ],
        |op, left, right| match op {
            Token::And => { Expression::Binary{op, left:Box::new(left),  right:Box::new(right)} }
            Token::Or => {Expression::Binary {op, left:Box::new(left), right:Box::new(right)}}
            Token::Equals => {Expression::Binary {op, left:Box::new(left), right:Box::new(right)}}
            Token::NotEqual => {Expression::Binary {op, left:Box::new(left), right:Box::new(right)}}
            Token::Greater => {Expression::Binary {op, left:Box::new(left), right:Box::new(right)}}
            Token::GreaterEqual => {Expression::Binary {op, left:Box::new(left), right:Box::new(right)}}
            Token::Less => {Expression::Binary {op, left:Box::new(left), right:Box::new(right)}}
            Token::LessEqual => {Expression::Binary {op, left:Box::new(left), right:Box::new(right)}}
            
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
            Token::Plus => {Expression::Binary {op, left:Box::new(left), right:Box::new(right)}},
            Token::Minus => {Expression::Binary {op, left:Box::new(left), right:Box::new(right)}},
            _ => unreachable!(),
        },
    )
}

fn parse_mul_div(tokens: &mut Vec<Token>) -> Expression {
    parse_binary(
        tokens,
        parse_primary,
        &[Token::Multiply, Token::Divide],
        |op, left, right| match op {
            Token::Multiply => {Expression::Binary {op, left:Box::new(left), right:Box::new(right)}},
            Token::Divide => {Expression::Binary {op, left:Box::new(left), right:Box::new(right)}},
            _ => unreachable!(),
        },
    )
}

fn parse_primary(tokens: &mut Vec<Token>) -> Expression {
    match tokens.remove(0) {
        Token::Number(n) => Expression::Number(n),
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
