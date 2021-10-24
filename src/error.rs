use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CalculatorError {
    ParseError(&'static str),
    SolveError(&'static str),
    UndefinedVariable(String),
}

impl Error for CalculatorError {}

impl Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedVariable(name) => {
                write!(f, "unable to find variable with name {}", name)
            }
            Self::ParseError(message) | Self::SolveError(message) => {
                write!(f, "{}", message)
            }
        }
    }
}
