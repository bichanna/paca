mod lexer;

/// Types of errors tokenizing the source code.
#[derive(Clone, Debug)]
pub enum LexError {
    /// Encountered an invalid character.
    InvalidCharacter(SourceCodeLocation),
    /// Encountered an invalid character literal.
    InvalidCharacterLiteral(SourceCodeLocation),
    /// Encountered an invalid hexadecimal number literal.
    InvalidHexadecimalNumber(SourceCodeLocation),
    /// Encountered an invalid floating point number literal.
    InvalidFloatingPointNumber(SourceCodeLocation),
    /// Encountered an invalid whole number.
    InvalidInteger(SourceCodeLocation),
    /// Encountered an invalid string literal.
    InvalidString(SourceCodeLocation),
    /// Encountered an invalid token.
    InvalidToken {
        /// Expected tokens.
        expected: Vec<&'static str>,
        loc: SourceCodeLocation,
    },
}

impl LexError {
    /// Generate a properly formatted error message
    fn generate_error_message(self, source_code: &String) -> String {
        let parse_err = "\nParse Error: ";
        match self {
            Self::InvalidCharacter(loc) => {
                loc.line_in_source_code(source_code) + parse_err + "Invalid character."
            }
            Self::InvalidCharacterLiteral(loc) => {
                loc.line_in_source_code(source_code) + parse_err + "Invalid character literal."
            }
            Self::InvalidHexadecimalNumber(loc) => {
                loc.line_in_source_code(source_code)
                    + parse_err
                    + "Invalid hexadecimal number literal."
            }
            Self::InvalidFloatingPointNumber(loc) => {
                loc.line_in_source_code(source_code) + parse_err + "Invalid float literal."
            }
            Self::InvalidInteger(loc) => {
                loc.line_in_source_code(source_code) + parse_err + "Invalid integer literal."
            }
            Self::InvalidString(loc) => {
                loc.line_in_source_code(source_code) + parse_err + "Invalid string literal."
            }
            Self::InvalidToken { expected, loc } => {
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
    pub filename: Option<String>,
}

impl SourceCodeLocation {
    pub fn new(
        line: usize,
        column: usize,
        offset: usize,
        length: usize,
        filename: Option<String>,
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
        let mut lines = source_code.lines();
        let mut line = lines.nth(self.line - 1).unwrap().to_string();

        let mut hats = String::new();
        for _ in 0..self.column {
            hats.push(' ');
        }
        for _ in 0..self.length {
            hats.push('^');
        }

        line = line + "\n" + &hats;

        line
    }
}
