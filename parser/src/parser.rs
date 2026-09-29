use crate::{
    Spanned,
    ast::{Expr, Literal, Operator},
    lexer::Token,
};

pub fn parse_expr(tokens: &[Token], position: &mut usize) -> Option<Expr> {
    let Some(token) = tokens.get(*position) else {
        return None;
    };
    match token {
        Token::Int(_) => todo!(),
        Token::Float(_) => todo!(),
        Token::Plus => todo!(),
        Token::Minus => todo!(),
        Token::Times => todo!(),
        Token::Divide => todo!(),
        Token::Equals => todo!(),
        Token::DoubleEquals => todo!(),
        Token::Semicolon => todo!(),
        Token::LeftParen => todo!(),
        Token::RightParen => todo!(),
        Token::Error(_) => todo!(),
    }
}

//type Parser<T> = dyn FnMut(&[Spanned<Token>], &mut usize) -> Option<T>;

pub fn parse_product(tokens: &[Spanned<Token>], position: &mut usize) -> Option<Spanned<Expr>> {
    let start_position = *position;

    let Some(Spanned(lhs, lhs_span)) = parse_atom(tokens, position) else {
        *position = start_position;
        return None;
    };
    let mut expression = Spanned(Box::new(lhs), lhs_span);

    while let Some((op, Spanned(rhs, rhs_span))) = parse_product_rhs(tokens, position) {
        expression = Spanned(
            Box::new(Expr::BinaryOp {
                lhs: expression,
                op,
                rhs: Spanned(rhs, rhs_span),
            }),
            crate::Span {
                start: lhs_span.start,
                end: rhs_span.end,
            },
        );
    }

    return Some(Spanned(*expression.0, expression.1));
}

fn parse_product_rhs(
    tokens: &[Spanned<Token>],
    position: &mut usize,
) -> Option<(Spanned<Operator>, Spanned<Box<Expr>>)> {
    let start_position = *position;
    let Some(Spanned(op, op_span)) = expect(&[Token::Times, Token::Divide], tokens, position)
    else {
        *position = start_position;
        return None;
    };
    let Some(Spanned(rhs, rhs_span)) = parse_atom(tokens, position) else {
        *position = start_position;
        return None;
    };
    return Some((
        Spanned(
            match op {
                Token::Times => Operator::Times,
                Token::Divide => Operator::Divide,
                _ => unreachable!(),
            },
            *op_span,
        ),
        Spanned(Box::new(rhs), rhs_span),
    ));
}

pub fn parse_sum(tokens: &[Spanned<Token>], position: &mut usize) -> Option<Spanned<Expr>> {
    let start_position = *position;

    let Some(Spanned(lhs, lhs_span)) = parse_product(tokens, position) else {
        *position = start_position;
        return None;
    };
    let mut expression = Spanned(Box::new(lhs), lhs_span);

    while let Some((op, Spanned(rhs, rhs_span))) = parse_sum_rhs(tokens, position) {
        expression = Spanned(
            Box::new(Expr::BinaryOp {
                lhs: expression,
                op,
                rhs: Spanned(rhs, rhs_span),
            }),
            crate::Span {
                start: lhs_span.start,
                end: rhs_span.end,
            },
        );
    }

    return Some(Spanned(*expression.0, expression.1));
}

fn parse_sum_rhs(
    tokens: &[Spanned<Token>],
    position: &mut usize,
) -> Option<(Spanned<Operator>, Spanned<Box<Expr>>)> {
    let start_position = *position;
    let Some(Spanned(op, op_span)) = expect(&[Token::Plus, Token::Minus], tokens, position) else {
        *position = start_position;
        return None;
    };
    let Some(Spanned(rhs, rhs_span)) = parse_product(tokens, position) else {
        *position = start_position;
        return None;
    };
    return Some((
        Spanned(
            match op {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                _ => unreachable!(),
            },
            *op_span,
        ),
        Spanned(Box::new(rhs), rhs_span),
    ));
}

pub fn parse_atom(tokens: &[Spanned<Token>], position: &mut usize) -> Option<Spanned<Expr>> {
    let Some(Spanned(token, span)) = tokens.get(*position) else {
        return None;
    };
    match token {
        Token::Int(int) => {
            *position += 1;
            int.parse::<i64>()
                .ok()
                .map(|it| Spanned(Expr::Literal(Spanned(Literal::Int(it), *span)), *span))
        }
        Token::Float(float) => {
            *position += 1;
            float
                .parse::<f64>()
                .ok()
                .map(|it| Spanned(Expr::Literal(Spanned(Literal::Float(it), *span)), *span))
        }
        _ => None,
    }
}

pub fn expect<'a>(
    one_of: &[Token],
    tokens: &'a [Spanned<Token<'a>>],
    position: &mut usize,
) -> Option<&'a Spanned<Token<'a>>> {
    if let Some(actual_token) = tokens.get(*position) {
        if one_of.contains(&actual_token.0) {
            *position += 1;
            return Some(actual_token);
        }
        println!("Got: {:?}", actual_token);
    }
    return None;
}
