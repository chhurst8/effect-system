use crate::ast::Literal;
use crate::ast_parser::parser::{OneOfParser, Parser, Repeated, SequenceParser};
use crate::lexer::Token;
use crate::{Expr, Spanned};

pub mod parser;

#[derive(Debug)]
pub enum ParserError {
    EOF,
    InvalidInt(String),
    InvalidFloat(String),
    ExpectedGot(),
}
struct BinaryParser<'a, T> {
    term_parser: T,
    valid_ops: &'a [Token<'a>],
}

impl<T> BinaryParser<'static, T> {
    pub fn new(term_parser: T, valid_ops: &'static [Token<'static>]) -> Self {
        Self {
            term_parser,
            valid_ops,
        }
    }
}

impl<'input, 'a, T> Parser<'input, Spanned<Expr>> for BinaryParser<'static, T>
where
    T: Parser<'input, Spanned<Expr>> + Clone,
{
    const name: &'static str = "BinaryParser";

    fn parse(
        &self,
        tokens: &'input [Spanned<Token>],
        position: usize,
    ) -> Result<(usize, Spanned<Expr>), ParserError> {
        let (position, Spanned(lhs, lhs_span)) = self.term_parser.parse(tokens, position)?;
        let op_parser = OneOfParser::new(self.valid_ops);
        let rhs_parser = Repeated::new(SequenceParser::new(&op_parser, &self.term_parser));
        let (position, rhs_s) = rhs_parser.parse(tokens, position)?;
        let mut expression = Spanned(Box::new(lhs), lhs_span);

        for (Spanned(op, op_span), Spanned(rhs, rhs_span)) in rhs_s {
            expression = Spanned(
                Box::new(Expr::BinaryOp {
                    lhs: expression,
                    op: Spanned(op.to_operator(), op_span),
                    rhs: Spanned(Box::new(rhs), rhs_span),
                }),
                crate::Span {
                    start: lhs_span.start,
                    end: rhs_span.end,
                },
            );
        }

        return Ok((position, Spanned(*expression.0, expression.1)));
    }
}
#[derive(Copy, Clone)]
pub struct ProductParser;
impl<'input> Parser<'input, Spanned<Expr>> for ProductParser {
    const name: &'static str = "ProductParser";

    fn parse(
        &self,
        tokens: &'input [Spanned<Token>],
        position: usize,
    ) -> Result<(usize, Spanned<Expr>), ParserError> {
        let term_parser = AtomParser {};
        let binary_parser = BinaryParser::new(term_parser, &[Token::Times, Token::Divide]);
        binary_parser.parse(tokens, position)
    }
}
#[derive(Copy, Clone)]
pub struct SemicolonParser;
impl<'input> Parser<'input, Spanned<Expr>> for SemicolonParser {
    const name: &'static str = "SemicolonParser";

    fn parse(
        &self,
        tokens: &'input [Spanned<Token>],
        position: usize,
    ) -> Result<(usize, Spanned<Expr>), ParserError> {
        let term_parser = SumParser {};
        let binary_parser = BinaryParser::new(term_parser, &[Token::Semicolon]);
        binary_parser.parse(tokens, position)
    }
}
#[derive(Copy, Clone)]
pub struct SumParser;
impl<'input> Parser<'input, Spanned<Expr>> for SumParser {
    const name: &'static str = "SumParser";

    fn parse(
        &self,
        tokens: &'input [Spanned<Token>],
        position: usize,
    ) -> Result<(usize, Spanned<Expr>), ParserError> {
        let term_parser = ProductParser {};
        let binary_parser = BinaryParser::new(term_parser, &[Token::Plus, Token::Minus]);
        binary_parser.parse(tokens, position)
    }
}
#[derive(Clone, Copy)]
pub struct AtomParser;

impl Parser<'_, Spanned<Expr>> for AtomParser {
    const name: &'static str = "AtomParser";

    fn parse(
        &self,
        tokens: &[Spanned<Token>],
        mut position: usize,
    ) -> Result<(usize, Spanned<Expr>), ParserError> {
        let Some(Spanned(token, span)) = tokens.get(position) else {
            return Err(ParserError::EOF);
        };
        match token {
            Token::Int(int) => {
                position += 1;
                int.parse::<i64>()
                    .map_err(|_| ParserError::InvalidInt(int.to_string()))
                    .map(|it| {
                        (
                            position,
                            Spanned(Expr::Literal(Spanned(Literal::Int(it), *span)), *span),
                        )
                    })
            }
            Token::Float(float) => {
                position += 1;
                float
                    .parse::<f64>()
                    .map_err(|_| ParserError::InvalidFloat(float.to_string()))
                    .map(|it| {
                        (
                            position,
                            Spanned(Expr::Literal(Spanned(Literal::Float(it), *span)), *span),
                        )
                    })
            }
            _ => Err(ParserError::ExpectedGot()),
        }
    }
}
