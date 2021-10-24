use std::io::stdin;

use ansi_term::Colour::{Green, Red};
use rust_calculator::Calculator;

fn main() {
    let mut line = String::with_capacity(1024);
    let mut calc = Calculator::new();

    loop {
        line.clear();

        match stdin().read_line(&mut line) {
            Ok(_) => match calc.exec(&line) {
                Ok(Some(result)) => {
                    let f = format!(">> {}", result.to_string());
                    println!("{}", Green.paint(f).to_string());
                }
                Err(err) => {
                    eprintln!("{}", Red.paint(err.to_string()));
                }
                _ => {}
            },
            Err(_) => {
                eprintln!("unexpected error occured while reading input");
                break;
            }
        }
    }

    println!("you said: {}", line);
}
