use crate::{Span, Spanned};

#[derive(PartialEq, Eq, Debug)]
pub enum Token<'a> {
    Int(&'a str),
    Float(&'a str),
    Plus,
    Minus,
    Times,
    Divide,
    Equals,
    DoubleEquals,
    Semicolon,
    LeftParen,
    RightParen,
    Error(char),
}
impl Token<'_> {
    pub(crate) fn to_operator(&self) -> crate::ast::Operator {
        match self {
            Token::Plus => crate::ast::Operator::Plus,
            Token::Minus => crate::ast::Operator::Minus,
            Token::Times => crate::ast::Operator::Times,
            Token::Divide => crate::ast::Operator::Divide,
            Token::Equals => todo!(),
            Token::DoubleEquals => todo!(),
            Token::Semicolon => crate::ast::Operator::Semicolon,
            _ => unreachable!(),
        }
    }
}

pub fn single_char_span<'a>(token: Token<'a>, position: usize) -> Spanned<Token<'a>> {
    Spanned::<Token>(
        token,
        Span {
            start: position,
            end: position,
        },
    )
}

fn consume_single_char<'a>(token: Token<'a>, position: &mut usize) -> Spanned<Token<'a>> {
    let spanned = single_char_span(token, *position);
    *position += 1;
    spanned
}

pub fn lex<'a>(input: &'a str) -> Vec<Spanned<Token<'a>>> {
    let mut tokens = Vec::new();
    let mut position: usize = 0;

    while position < input.len() {
        let Some(ch) = input.chars().nth(position) else {
            break;
        };
        tokens.push(match ch {
            '+' => consume_single_char(Token::Plus, &mut position),
            '-' => consume_single_char(Token::Minus, &mut position),
            '*' => consume_single_char(Token::Times, &mut position),
            '/' => consume_single_char(Token::Divide, &mut position),
            '=' => match peek(input, position) {
                Some('=') => {
                    position += 2;
                    Spanned(
                        Token::DoubleEquals,
                        Span {
                            start: position - 2,
                            end: position,
                        },
                    )
                }
                _ => consume_single_char(Token::Equals, &mut position),
            },
            ';' => consume_single_char(Token::Semicolon, &mut position),
            '\n' | '\t' | ' ' => {
                position += 1;
                continue;
            }
            x => {
                if x.is_numeric() {
                    consume_number(input, &mut position)
                } else {
                    consume_single_char(Token::Error(x), &mut position)
                }
            }
        })
    }

    tokens
}

fn peek(input: &str, position: usize) -> Option<char> {
    input.chars().nth(position + 1)
}

fn consume_number<'a>(input: &'a str, position: &mut usize) -> Spanned<Token<'a>> {
    let mut float = false;
    let start = *position;
    while let Some(ch) = input.chars().nth(*position) {
        match ch {
            '.' => {
                if !float {
                    float = true;
                } else {
                    *position -= 1;
                    break;
                }
            }
            ch => {
                if !ch.is_numeric() {
                    //*position -= 1;
                    break;
                }
            }
        }
        *position += 1;
    }
    let token_type = if float { Token::Float } else { Token::Int };
    let end = *position;

    Spanned(
        token_type(&input[start..end]),
        Span {
            start,
            end: end - 1,
        },
    )
}
