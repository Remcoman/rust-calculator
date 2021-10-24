use std::{cell::RefCell, collections::HashMap, iter::Peekable};

mod error;
mod parser;
mod types;

use error::CalculatorError;
use parser::{parse, Token};
use types::Number;

pub type Result = std::result::Result<Option<Number>, CalculatorError>;

type SolverResult = std::result::Result<Number, CalculatorError>;

struct Solver<'a> {
    tokens: Vec<Token>,
    token_stack: RefCell<Vec<String>>,
    calculator: &'a mut Calculator,
}

impl<'a> Solver<'a> {
    pub fn new(calculator: &'a mut Calculator, tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            calculator,
            token_stack: RefCell::new(vec![]),
        }
    }

    fn solve_token(&self, token: &Token) -> SolverResult {
        match token {
            Token::Value(v) => Ok(v.clone()),
            Token::Group(tokens) => self.solve_token_list(tokens),
            Token::Identifier(ident) => {
                if self.token_stack.borrow().contains(ident) {
                    return Err(CalculatorError::SolveError(
                        "A used variable was already used as part of the calculation",
                    ));
                }

                // push the used token on the stack
                {
                    self.token_stack.borrow_mut().push(ident.clone())
                }

                let tokens = self
                    .calculator
                    .assignments
                    .get(ident)
                    .ok_or_else(|| CalculatorError::UndefinedVariable(ident.to_owned()))?;

                let solved = self.solve_token_list(tokens);

                // remove it again from the stack
                {
                    self.token_stack.borrow_mut().pop();
                }

                solved
            }
            _ => unimplemented!(),
        }
    }

    fn solve_token_list(&self, tokens: &[Token]) -> SolverResult {
        let mut iter = tokens.iter().peekable();

        let first_token = {
            let mut token = iter.next().unwrap();
            if let Some(Token::Assignment) = iter.peek() {
                token = iter.nth(1).ok_or(CalculatorError::SolveError(
                    "Expected token after assignment",
                ))?;
            }
            token
        };

        let lhs = self.solve_token(first_token)?;
        self.solve_precendence(lhs, &mut iter, 0)
    }

    fn solve_precendence<'c, I: Iterator<Item = &'c Token>>(
        &self,
        mut lhs: Number,
        iter: &mut Peekable<I>,
        min_precedence: u8,
    ) -> SolverResult {
        let mut lookahead: Option<&Token> = iter.peek().copied();

        while let Some(op) = lookahead
            .and_then(|i| i.to_operator())
            .filter(|op| op.precedence() >= min_precedence)
        {
            iter.next();

            let mut rhs = iter
                .next()
                .ok_or(CalculatorError::SolveError("Expected value after operator"))
                .and_then(|op| self.solve_token(op))?;

            lookahead = iter.peek().copied();

            while lookahead
                .and_then(|i| i.to_operator())
                .filter(|op2| op2.precedence() > op.precedence())
                .is_some()
            {
                rhs = self.solve_precendence(rhs, iter, min_precedence + 1)?;

                lookahead = iter.next();
            }

            lhs = op.apply(lhs, rhs);
        }

        Ok(lhs)
    }

    pub fn solve(&self) -> SolverResult {
        let l = &self.tokens[..];
        self.solve_token_list(l)
    }
}

pub struct Calculator {
    assignments: HashMap<String, Vec<Token>>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            assignments: HashMap::new(),
        }
    }

    pub fn exec(&mut self, s: &str) -> Result {
        let (_, tokens) = parse(s, false)?;

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
