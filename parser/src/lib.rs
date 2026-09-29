use crate::ast::Expr;

mod ast;
mod lexer;
mod parser;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct Spanned<T>(T, Span);

fn eval(expression: Expr) -> f64 {
    match expression {
        Expr::Literal(Spanned(literal, _)) => match literal {
            ast::Literal::Int(x) => x as f64,
            ast::Literal::Float(x) => x,
        },
        Expr::BinaryOp {
            lhs: Spanned(lhs, _),
            op: Spanned(op, _),
            rhs: Spanned(rhs, _),
        } => {
            let lhs = eval(*lhs);
            let rhs = eval(*rhs);
            match op {
                ast::Operator::Plus => lhs + rhs,
                ast::Operator::Minus => lhs - rhs,
                ast::Operator::Times => lhs * rhs,
                ast::Operator::Divide => lhs / rhs,
                ast::Operator::Semicolon => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Operator,
        lexer::{Token, lex, single_char_span},
    };

    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn int_lex() {
        let input = "5555";
        let tokens = lex(input);
        assert_eq!(
            vec![Spanned(Token::Int("5555"), Span { start: 0, end: 3 })],
            tokens
        );
    }
    #[test]
    fn op_lex() {
        let input = "+-*/";
        let tokens = lex(input);
        assert_eq!(
            vec![
                single_char_span(Token::Plus, 0),
                single_char_span(Token::Minus, 1),
                single_char_span(Token::Times, 2),
                single_char_span(Token::Divide, 3),
            ],
            tokens
        );
    }
    #[test]
    fn int_and_float_and_op_lex() {
        let input = "5+53.2234";
        let tokens = lex(input);
        assert_eq!(
            vec![
                single_char_span(Token::Int("5"), 0),
                single_char_span(Token::Plus, 1),
                Spanned(Token::Float("53.2234"), Span { start: 2, end: 8 })
            ],
            tokens
        );
    }
    #[test]
    fn int_atom_parse() {
        let input = "521";
        let tokens = lex(input);
        let mut position = 0;
        let atom = parser::parse_atom(&tokens, &mut position);
        assert_eq!(
            Some(Spanned(
                ast::Expr::Literal(Spanned(ast::Literal::Int(521), Span { start: 0, end: 2 })),
                Span { start: 0, end: 2 }
            )),
            atom
        );
    }
    #[test]
    fn simple_sum_parse() {
        let input = "521+2";
        let tokens = lex(input);
        let mut position = 0;
        let atom = parser::parse_sum(&tokens, &mut position);
        assert_eq!(
            Some(Spanned(
                ast::Expr::BinaryOp {
                    lhs: Spanned(
                        Box::new(ast::Expr::Literal(Spanned(
                            ast::Literal::Int(521),
                            Span { start: 0, end: 2 }
                        ))),
                        Span { start: 0, end: 2 }
                    ),
                    op: Spanned(Operator::Plus, Span { start: 3, end: 3 }),
                    rhs: Spanned(
                        Box::new(ast::Expr::Literal(Spanned(
                            ast::Literal::Int(2),
                            Span { start: 4, end: 4 }
                        ))),
                        Span { start: 4, end: 4 }
                    )
                },
                Span { start: 0, end: 4 }
            )),
            atom
        );
    }
    #[test]
    fn double_sum_parse() {
        let input = "521+2";
        let tokens = lex(input);
        let mut position = 0;
        let atom = parser::parse_sum(&tokens, &mut position);
        assert_eq!(
            Some(Spanned(
                ast::Expr::BinaryOp {
                    lhs: Spanned(
                        Box::new(ast::Expr::Literal(Spanned(
                            ast::Literal::Int(521),
                            Span { start: 0, end: 2 }
                        ))),
                        Span { start: 0, end: 2 }
                    ),
                    op: Spanned(Operator::Plus, Span { start: 3, end: 3 }),
                    rhs: Spanned(
                        Box::new(ast::Expr::Literal(Spanned(
                            ast::Literal::Int(2),
                            Span { start: 4, end: 4 }
                        ))),
                        Span { start: 4, end: 4 }
                    )
                },
                Span { start: 0, end: 4 }
            ),),
            atom
        );
    }
    #[test]
    fn double_sum_eval() {
        let input = "2-2+521";
        let tokens = lex(input);
        let mut position = 0;
        let expr = parser::parse_sum(&tokens, &mut position).expect("Failed to parse");
        assert_eq!(eval(expr.0), 521.);
    }
    #[test]
    fn sum_and_product_eval() {
        let input = "521+2*2";
        let tokens = lex(input);
        let mut position = 0;
        let expr = parser::parse_sum(&tokens, &mut position).expect("Failed to parse");
        dbg!(&expr);
        assert_eq!(eval(expr.0), 525.);
    }
//    #[test]
//    fn semicolon_eval() {
//        let input = "521+12;32+14";
//        let tokens = lex(input);
//        let mut position = 0;
//        let expr = parser::parse_sum(&tokens, &mut position).expect("Failed to parse");
//        dbg!(&expr);
//        assert_eq!(eval(expr.0), 525.);
//    }
}
