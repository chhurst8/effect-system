use crate::{
    Spanned,
    ast_parser::ParserError,
    lexer::Token,
};

pub trait Parser<'input, T> {
    const name: &'static str;

    fn parse(
        &self,
        tokens: &'input [Spanned<Token>],
        position: usize,
    ) -> Result<(usize, T), ParserError>;
}

pub struct SequenceParser<'a, L, R> {
    lhs: &'a L,
    rhs: &'a R,
}
impl<'a, L, R> SequenceParser<'a, L, R> {
    pub fn new(lhs: &'a L, rhs: &'a R) -> Self {
        Self { lhs, rhs }
    }
}

impl<'input, 'a, L, R, A, B> Parser<'input, (A, B)> for SequenceParser<'a, L, R>
where
    L: Parser<'input, A>,
    R: Parser<'input, B>,
{
    const name: &'static str = "SequenceParser";
    fn parse(
        &self,
        tokens: &'input [Spanned<Token>],
        position: usize,
    ) -> Result<(usize, (A, B)), ParserError> {
        let (position, l) = self.lhs.parse(tokens, position)?;
        let (position, r) = self.rhs.parse(tokens, position)?;

        return Ok((position, (l, r)));
    }
}

pub struct Repeated<P> {
    inner: P,
}

impl<P> Repeated<P> {
    pub fn new(inner: P) -> Self {
        Self { inner }
    }
}

impl<'input, P, T> Parser<'input, Vec<T>> for Repeated<P>
where
    P: Parser<'input, T>,
{
    const name: &'static str = "Repeated";
    fn parse(
        &self,
        tokens: &'input [Spanned<Token>],
        mut position: usize,
    ) -> Result<(usize, Vec<T>), ParserError> {
        let mut items = vec![];
        while let Ok((new_position, v)) = self.inner.parse(tokens, position) {
            position = new_position;
            items.push(v);
        }

        return Ok((position, items));
    }
}

pub struct OneOfParser<'a, 'b> {
    options: &'a [Token<'b>],
}

impl<'a, 'b> OneOfParser<'a, 'b> {
    pub fn new(options: &'a [Token<'b>]) -> Self {
        Self { options }
    }
}

impl<'a> Parser<'a, Spanned<&'a Token<'a>>> for OneOfParser<'_, 'a> {
    const name: &'static str = "OneOfParser";

    fn parse(
        &self,
        tokens: &'a [Spanned<Token>],
        position: usize,
    ) -> Result<(usize, Spanned<&'a Token<'a>>), ParserError> {
        let Some(Spanned(token, span)) = tokens.get(position) else {
            return Err(ParserError::EOF);
        };

        if !self.options.contains(token) {
            return Err(ParserError::ExpectedGot());
        }

        return Ok((position + 1, Spanned(token, *span)));
    }
}
