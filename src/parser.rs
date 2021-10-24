use super::error::CalculatorError;

use super::types::{Number, Operator};

enum State {
    Idle,
    Number(usize),
    Identifier(usize),
    FromOperator,
    FromNumber,
    FromIdentifier,
    FromAssignment,
}

#[derive(Debug, Clone)]
pub enum Token {
    Value(Number),
    Identifier(String),
    Operator(Operator),
    Assignment,
    Group(Vec<Token>),
}

impl Token {
    pub fn to_operator(&self) -> Option<Operator> {
        match self {
            Token::Operator(op) => Some(op.clone()),
            _ => None,
        }
    }
}

fn is_numeric(c: u8) -> bool {
    matches!(c, b'0'..=b'9' | b'.' | b'-')
}

fn is_alpha(c: u8) -> bool {
    matches!(c, b'a'..=b'z' | b'A'..=b'Z')
}

fn is_whitespace(c: u8) -> bool {
    matches!(c, b' ')
}

fn get_operator(c: u8) -> Option<Operator> {
    match c {
        b'+' => Some(Operator::Add),
        b'*' => Some(Operator::Multiply),
        b'/' => Some(Operator::Div),
        b'-' => Some(Operator::Subtract),
        _ => None,
    }
}

pub fn parse(s: &str, in_group: bool) -> Result<(usize, Vec<Token>), CalculatorError> {
    let mut state: State = State::Idle;
    let mut tokens: Vec<Token> = vec![];
    let mut consumed = 0;
    let bytes = s.as_bytes();

    while consumed < bytes.len() {
        let c = bytes[consumed];

        match state {
            State::Idle | State::FromOperator | State::FromAssignment => {
                if is_numeric(c) {
                    state = State::Number(consumed);
                    continue;
                } else if is_alpha(c) {
                    state = State::Identifier(consumed);
                } else if !is_whitespace(c) {
                    if c == b'(' {
                        let (items_read, group_tokens) = parse(&s[consumed + 1..], true)?;
                        tokens.push(Token::Group(group_tokens));
                        consumed += items_read;
                        state = State::FromNumber;
                    } else {
                        return Err(CalculatorError::ExpectedNumberOrGroup);
                    }
                }
            }
            State::Number(start_index) => {
                if !is_numeric(c) {
                    let v: Number = {
                        let num = &s[start_index..consumed];
                        if num.find('.').is_some() {
                            Number::Float(num.parse().or(Err(CalculatorError::InvalidNumber))?)
                        } else {
                            Number::Integer(num.parse().or(Err(CalculatorError::InvalidNumber))?)
                        }
                    };
                    tokens.push(Token::Value(v));

                    state = State::FromNumber;
                    continue;
                }
            }
            State::FromIdentifier | State::FromNumber => {
                if let Some(op) = get_operator(c) {
                    tokens.push(Token::Operator(op));
                    state = State::FromOperator;
                } else {
                    match c {
                        b')' => {
                            consumed += 1;
                            assert!(in_group);
                            break;
                        }
                        b'=' => {
                            tokens.push(Token::Assignment);
                            state = State::FromAssignment;
                            assert!(!in_group);
                        }
                        _ => {}
                    }
                }
            }
            State::Identifier(start_index) => {
                if !is_numeric(c) && !is_alpha(c) {
                    let ident = s[start_index..consumed].to_owned();
                    tokens.push(Token::Identifier(ident));
                    state = State::FromIdentifier;
                    continue;
                }
            }
        }

        consumed += 1;
    }

    Ok((consumed, tokens))
}
