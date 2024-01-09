use crate::parse::expr::Stmt;
use crate::parse::lexer::{Token, TokenKind};
use crate::parse::{ParseError, SourceCodeLocation};

/// This trait is for parsers that returns an AST of `Clone`-able, `SourceCodeLocation`-convertible.
pub trait Parse {
    type Type: Clone + Into<SourceCodeLocation>;
    fn parse(self) -> Result<Vec<Self::Type>, ParseError>;
}

/// This struct is used for holding all the information needed for parsing.
pub struct Parser {
    /// Length of the tokens.
    len: usize,
    /// Tokens parsed.
    tokens: Vec<Token>,
    /// The current index of the token parsed.
    idx: usize,
    /// A list of statements (AST).
    parsed: Vec<Stmt>,
}

impl Parse for Parser {
    type Type = Stmt;

    fn parse(self) -> Result<Vec<Self::Type>, ParseError> {
        todo!()
    }
}

impl Parser {
    /// Check whether the index has reached the end or not.
    fn is_end(&self) -> bool {
        self.idx >= self.len
    }

    /// Get the current token indexed by `idx`.
    fn current(&self) -> &Token {
        self.tokens.get(self.idx).unwrap()
    }

    /// Advance the index value.
    fn advance(&mut self) -> &Token {
        self.idx += 1;
        self.current()
    }
}
