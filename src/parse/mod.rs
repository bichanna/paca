use crate::util::GenerateErrorMessage;
use std::rc::Rc;

mod lexer;

/// Struct for lexer errors.
#[derive(Clone, Debug)]
pub struct LexError {
    r#type: LexErrorType,
    loc: SourceCodeLocation,
}

impl LexError {
    pub fn new(t: LexErrorType, loc: SourceCodeLocation) -> Self {
        Self { r#type: t, loc }
    }
}

/// Types of errors tokenizing the source code.
#[derive(Clone, Debug)]
pub enum LexErrorType {
    /// Encountered an invalid character.
    InvalidCharacter,
    /// Encountered an invalid character literal.
    InvalidCharacterLiteral,
    /// Encountered an invalid hexadecimal number literal.
    InvalidHexadecimalNumber,
    /// Encountered an invalid floating point number literal.
    InvalidFloatingPointNumber,
    /// Encountered an invalid whole number.
    InvalidInteger,
    /// Encountered an invalid string literal.
    InvalidString,
    /// Encountered an invalid token.
    InvalidToken(Vec<&'static str>),
}

impl GenerateErrorMessage for LexError {
    /// Generate a properly formatted error message
    fn generate_error_message(self, source_code: &String) -> String {
        let parse_err = "\nParse Error: ";
        let loc = self.loc;
        match self.r#type {
            LexErrorType::InvalidCharacter => {
                loc.line_in_source_code(source_code) + parse_err + "Invalid character."
            }
            LexErrorType::InvalidCharacterLiteral => {
                loc.line_in_source_code(source_code) + parse_err + "Invalid character literal."
            }
            LexErrorType::InvalidHexadecimalNumber => {
                loc.line_in_source_code(source_code)
                    + parse_err
                    + "Invalid hexadecimal number literal."
            }
            LexErrorType::InvalidFloatingPointNumber => {
                loc.line_in_source_code(source_code) + parse_err + "Invalid float literal."
            }
            LexErrorType::InvalidInteger => {
                loc.line_in_source_code(source_code) + parse_err + "Invalid integer literal."
            }
            LexErrorType::InvalidString => {
                loc.line_in_source_code(source_code) + parse_err + "Invalid string literal."
            }
            LexErrorType::InvalidToken(expected) => {
                loc.line_in_source_code(source_code)
                    + parse_err
                    + "Expected "
                    + &expected.join(", ")
            }
        }
    }
}

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
