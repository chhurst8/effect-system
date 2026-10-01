use crate::ast::Expr;

mod ast;
mod ast_parser;
mod lexer;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct Spanned<T>(T, Span);

#[cfg(test)]
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
                ast::Operator::Semicolon => rhs,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast_parser::AtomParser;
    use crate::{
        ast::{Literal, Operator},
        ast_parser::parser::{OneOfParser, Parser, Repeated, SequenceParser},
        lexer::{Token, lex, single_char_span},
    };

    use super::*;
    use pretty_assertions::assert_eq;

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
    fn atom_parse() {
        let input = "502";
        let tokens = lex(input);
        let parsed = AtomParser {}.parse(&tokens, 0).unwrap();
        assert_eq!(
            (
                1,
                Spanned(
                    Expr::Literal(Spanned(Literal::Int(502), Span { start: 0, end: 2 })),
                    Span { start: 0, end: 2 }
                )
            ),
            parsed
        );
    }
    #[test]
    fn op_parse() {
        let input = "*";
        let tokens = lex(input);
        let parsed = OneOfParser::new(&[Token::Times]).parse(&tokens, 0).unwrap();
        assert_eq!(
            (1, Spanned(&Token::Times, Span { start: 0, end: 0 })),
            parsed
        );
    }
    #[test]
    fn sequence_parser() {
        let input = "*5";
        let tokens = lex(input);
        let atom_parser = AtomParser {};
        let op_parser = OneOfParser::new(&[Token::Times]);
        let parsed = SequenceParser::new(&op_parser, &atom_parser)
            .parse(&tokens, 0)
            .unwrap();
        assert_eq!(
            (
                2,
                (
                    Spanned(&Token::Times, Span { start: 0, end: 0 }),
                    Spanned(
                        Expr::Literal(Spanned(Literal::Int(5), Span { start: 1, end: 1 })),
                        Span { start: 1, end: 1 }
                    )
                )
            ),
            parsed
        );
    }

    #[test]
    fn repeated_parser() {
        let input = "**";
        let tokens = lex(input);
        let op_parser = OneOfParser::new(&[Token::Times]);
        let parsed = Repeated::new(op_parser).parse(&tokens, 0).unwrap();
        assert_eq!(
            (
                2,
                vec![
                    Spanned(&Token::Times, Span { start: 0, end: 0 }),
                    Spanned(&Token::Times, Span { start: 1, end: 1 })
                ],
            ),
            parsed
        );
    }

    #[test]
    fn repeated_product() {
        let input = "5*53.2234";
        let tokens = lex(input);
        let (_, Spanned(expr, _)) = ast_parser::ProductParser {}.parse(&tokens, 0).unwrap();
        assert_eq!(
            Expr::BinaryOp {
                lhs: Spanned(
                    Box::new(Expr::Literal(Spanned(
                        ast::Literal::Int(5),
                        Span { start: 0, end: 0 }
                    ))),
                    Span { start: 0, end: 0 }
                ),
                op: Spanned(Operator::Times, Span { start: 1, end: 1 }),
                rhs: Spanned(
                    Box::new(Expr::Literal(Spanned(
                        ast::Literal::Float(53.2234),
                        Span { start: 2, end: 8 }
                    ))),
                    Span { start: 2, end: 8 }
                ),
            },
            expr
        )
    }
    #[test]
    fn int_atom_parse() {
        let input = "521";
        let tokens = lex(input);
        let (_, atom) = ast_parser::AtomParser {}.parse(&tokens, 0).unwrap();
        assert_eq!(
            Spanned(
                ast::Expr::Literal(Spanned(ast::Literal::Int(521), Span { start: 0, end: 2 })),
                Span { start: 0, end: 2 }
            ),
            atom
        );
    }
    #[test]
    fn simple_sum_parse() {
        let input = "521+2";
        let tokens = lex(input);
        let (_, atom) = ast_parser::SumParser {}.parse(&tokens, 0).unwrap();
        assert_eq!(
            Spanned(
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
            ),
            atom
        );
    }
    #[test]
    fn double_sum_parse() {
        let input = "521+2";
        let tokens = lex(input);
        let (_, atom) = ast_parser::SumParser {}.parse(&tokens, 0).unwrap();
        assert_eq!(
            Spanned(
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
            ),
            atom
        );
    }
    #[test]
    fn double_product_eval() {
        let input = "2*2*521";
        let tokens = lex(input);
        let (_, expr) = ast_parser::ProductParser {}
            .parse(&tokens, 0)
            .expect("Failed to parse");
        dbg!(&expr.0);
        assert_eq!(eval(expr.0), 2. * 2. * 521.);
    }
    #[test]
    fn sum_and_product_eval() {
        let input = "521+2*2";
        let tokens = lex(input);
        let (_, expr) = ast_parser::SumParser {}
            .parse(&tokens, 0)
            .expect("Failed to parse");
        dbg!(&expr);
        assert_eq!(eval(expr.0), 525.);
    }
    #[test]
    fn semicolon_eval() {
        let input = "521 + 12;\n15/2/2;\n32+14";
        let tokens = lex(input);
        let (_, expr) = ast_parser::SemicolonParser {}
            .parse(&tokens, 0)
            .expect("Failed to parse");
        assert_eq!(eval(expr.0), 32. + 14.);
    }
}
