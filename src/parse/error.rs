use crate::parse::SourceCodeLocation;
use crate::util::GenerateErrorMessage;

/// Struct for parser errors.
#[derive(Clone, Debug)]
pub struct ParseError {
    r#type: ParseErrorType,
    loc: SourceCodeLocation,
}

impl ParseError {
    pub fn new(t: ParseErrorType, loc: SourceCodeLocation) -> Self {
        Self { r#type: t, loc }
    }
}

#[derive(Clone, Debug)]
pub enum ParseErrorType {
    /// Unexpected random token.
    InvalidToken(String),
    /// Expected some tokens, but got different some other token.
    UnexpectedToken(Vec<&'static str>),
    /// Unexpected end of a block.
    UnexpectedEndOfBlock,
}

/// Struct for lexer errors.
#[derive(Clone, Debug)]
pub struct LexError {
    r#type: LexErrorType,
    loc: SourceCodeLocation,
}

impl GenerateErrorMessage for ParseError {
    fn generate_error_message(self, source_code: &String) -> String {
        let msg = self.loc.line_in_source_code(source_code) + "\nParse Error: ";
        match self.r#type {
            ParseErrorType::InvalidToken(token) => {
                let result = format!("Unexpected token: {}.", token);
                msg + &result
            }
            ParseErrorType::UnexpectedToken(expected) => {
                let result = format!("Expected {}.", expected.join(" or "));
                msg + &result
            }
            ParseErrorType::UnexpectedEndOfBlock => {
                msg + "Unexpected end of input while parsing a block"
            }
        }
    }
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
        let msg = self.loc.line_in_source_code(source_code) + "\nParse Error: ";
        match self.r#type {
            LexErrorType::InvalidCharacter => msg + "Invalid character.",
            LexErrorType::InvalidCharacterLiteral => msg + "Invalid character literal.",
            LexErrorType::InvalidHexadecimalNumber => msg + "Invalid hexadecimal number literal.",
            LexErrorType::InvalidFloatingPointNumber => msg + "Invalid float literal.",
            LexErrorType::InvalidInteger => msg + "Invalid integer literal.",
            LexErrorType::InvalidString => msg + "Invalid string literal.",
            LexErrorType::InvalidToken(expected) => {
                msg + &format!("Expected {}.", expected.join(" or "))
            }
        }
    }
}
