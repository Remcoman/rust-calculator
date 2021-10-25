use std::collections::HashMap;

mod error;
mod parser;
mod solver;
mod types;

use error::CalculatorError;
use parser::{parse, Token};
use solver::Solver;
use types::Number;

pub type Result = std::result::Result<Option<Number>, CalculatorError>;

pub struct Calculator {
    assignments: HashMap<String, Vec<Token>>,
}

impl Calculator {
    /// Create a new calculator
    pub fn new() -> Self {
        Self {
            assignments: HashMap::new(),
        }
    }

    /// Calculates the result for the given expression
    /// # Arguments
    /// * `expression` - the calculation that needs to be performed
    ///
    /// # Examples
    /// let calculator = Calculator::new();
    /// match calculator.exec("1+1") {
    ///    Ok(Some(result)) => println!("The answer was {}", result),
    ///    Err(e) => eprintln!("An error occured: {}", e),
    ///    _ => {}, // an assignment (eg a = 1+1) will not yield any result
    /// }
    pub fn exec(&mut self, expression: &str) -> Result {
        let tokens = parse(expression)?;

        let result = match &tokens[..] {
            // test for assignment
            [Token::Identifier(ident), Token::Assignment, ..] => {
                self.assignments.insert(ident.to_owned(), tokens);
                None
            }
            _ => Solver::new(self, tokens).solve().map(Some)?,
        };

        Ok(result)
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{error::CalculatorError, types::Number, Calculator, Result};

    #[test]
    fn can_solve_calculations() {
        let mut calculator = Calculator::new();
        assert!(matches!(
            calculator.exec("1+1"),
            Ok(Some(Number::Integer(2)))
        ));
    }

    #[test]
    fn can_use_variables() {
        let mut calculator = Calculator::new();
        assert!(matches!(calculator.exec("a = 1+1"), Ok(None)));

        assert!(matches!(calculator.exec("a"), Ok(Some(Number::Integer(2)))));
    }

    #[test]
    fn crashes_on_infinite_loops() {
        let mut calculator = Calculator::new();
        assert!(matches!(calculator.exec("a = a + 1"), Ok(None)));

        assert!(matches!(
            calculator.exec("a"),
            Err(CalculatorError::SolveError(_))
        ));
    }

    #[test]
    fn crashes_on_undefined_variables() {
        let mut calculator = Calculator::new();
        assert!(
            matches!(calculator.exec("b + 1"), Err(CalculatorError::UndefinedVariable(var)) if var == "b")
        );
    }
}
