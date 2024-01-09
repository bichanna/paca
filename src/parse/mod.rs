use crate::util::GenerateErrorMessage;
use std::rc::Rc;

mod error;
mod expr;
mod lexer;
mod parser;

pub use error::*;

/// A struct that represents a location in the input source code.
/// Used for properly format errors.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceCodeLocation {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub length: usize,
    pub filename: Option<Rc<str>>,
}

impl SourceCodeLocation {
    pub fn new(
        line: usize,
        column: usize,
        offset: usize,
        length: usize,
        filename: Option<Rc<str>>,
    ) -> Self {
        Self {
            line,
            column,
            offset,
            length,
            filename,
        }
    }

    /// Generate a string with two lines: the line at which the error occurred and a line
    /// with ^'s, pointing at precise location of the error.
    pub fn line_in_source_code(&self, source_code: &String) -> String {
        let SourceCodeLocation {
            line,
            column,
            offset,
            length,
            filename,
        } = self.clone();

        let loc = format!(
            "{}:{}:{}:{}",
            filename.unwrap_or(Rc::from("unknown")).to_string(),
            line,
            column,
            offset
        );
        let mut lines = source_code.lines();
        let mut line = lines.nth(line - 1).unwrap().to_string();

        let mut hats = String::new();
        for _ in 1..column {
            hats.push(' ');
        }
        for _ in 0..length {
            hats.push('^');
        }

        line = format!("Error at {}", loc) + "\n\n" + &line;
        line = line + "\n" + &hats;

        line
    }
}
