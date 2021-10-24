use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CalculatorError {
    ExpectedNumberOrGroup,
    InvalidNumber,
    UndefinedVariable(String),
}

impl Error for CalculatorError {}

impl Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedVariable(name) => {
                write!(f, "unable to find variable with name {}", name)
            }
            Self::ExpectedNumberOrGroup => {
                write!(f, "Expected a number but got something else")
            }
            Self::InvalidNumber => {
                write!(f, "Invalid number")
            }
        }
    }
}
