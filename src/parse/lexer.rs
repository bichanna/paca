use crate::parse::{LexError, SourceCodeLocation};
use crate::util::{escape_char, weird_while};
use std::iter::Peekable;
use std::str::Chars;

/// This trait is for lexers that returns a `Vec` of `Clone`-able, `SourceCodeLocation`-convertible tokens.
pub trait Tokenize {
    type TokenType: Clone + Into<SourceCodeLocation>;
    fn tokenize(self) -> Result<Vec<Self::TokenType>, LexError>;
}

/// Types of tokens.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    /// Identifier
    Ident(String),
    /// String
    Str(String),
    /// Character
    Char(char),
    /// Integer
    Int(i64),
    /// Floating point number
    Float(f64),
    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// \
    BackSlash,
    /// +
    Plus,
    /// +=
    PlusEq,
    /// -
    Minus,
    /// -=
    MinusEq,
    /// ->
    MinusGreaterThan,
    /// *
    Mul,
    /// *=
    MulEq,
    /// /
    Div,
    /// /=
    DivEq,
    /// %
    Rem,
    /// %=
    RemEq,
    /// ,
    Comma,
    /// ||
    DoubleVertical,
    /// &&
    DoubleAmp,
    /// >
    GreaterThan,
    /// <
    LessThan,
    /// >=
    GreaterThanOrEq,
    /// <=
    LessThanOrEq,
    /// !
    Bang,
    /// !=
    BangEq,
    /// =
    Eq,
    /// ==
    DoubleEq,
    /// .
    Dot,
    /// =>
    EqGreaterThan,
    /// ::
    DoubleColon,
    /// :
    Colon,
    /// ;
    SemiColon,
    /// Keywords are stored in this.
    Keyword(Keyword),
}

/// All keyword types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
    If,
    Else,
    Match,
    Def,
    Let,
    Str,
    Int,
    Float,
    Struct,
    Enum,
    Impl,
    For,
    While,
    LilSelf,
    BigSelf,
    Break,
    Return,
    Continue,
    Methods,
    Import,
    Export,
}

impl TryInto<Keyword> for String {
    type Error = ();

    fn try_into(self) -> Result<Keyword, Self::Error> {
        match self.as_str() {
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "match" => Ok(Keyword::Match),
            "def" => Ok(Keyword::Def),
            "let" => Ok(Keyword::Let),
            "str" => Ok(Keyword::Str),
            "int" => Ok(Keyword::Int),
            "float" => Ok(Keyword::Float),
            "struct" => Ok(Keyword::Struct),
            "enum" => Ok(Keyword::Enum),
            "impl" => Ok(Keyword::Impl),
            "for" => Ok(Keyword::For),
            "while" => Ok(Keyword::While),
            "self" => Ok(Keyword::LilSelf),
            "Self" => Ok(Keyword::BigSelf),
            "break" => Ok(Keyword::Break),
            "return" => Ok(Keyword::Return),
            "continue" => Ok(Keyword::Continue),
            "methods" => Ok(Keyword::Methods),
            "import" => Ok(Keyword::Import),
            "export" => Ok(Keyword::Export),
            _ => Err(()),
        }
    }
}

/// A struct representing each token in the source code.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// Type of the token.
    pub kind: TokenKind,
    pub loc: SourceCodeLocation,
}

impl Token {
    fn new(kind: TokenKind, loc: SourceCodeLocation) -> Self {
        Self { kind, loc }
    }
}

impl Into<SourceCodeLocation> for Token {
    fn into(self) -> SourceCodeLocation {
        self.loc
    }
}

/// A struct for holding all the information needed for tokenizing the source code.
pub struct Lexer<'src> {
    /// The file name of the source code, if there's one.
    filename: Option<String>,
    /// The source code to tokenize.
    source: Peekable<Chars<'src>>,
    /// All the tokens.
    tokens: Vec<Token>,
    /// Current character
    c: char,

    line: usize,
    column_c: usize,
    column: usize,
    offset: usize,
    length: usize,
}

impl<'src> Tokenize for Lexer<'src> {
    type TokenType = Token;

    fn tokenize(mut self) -> Result<Vec<Self::TokenType>, LexError> {
        self.next();
        while !self.is_end() {
            match self.c {
                // An identifier or keyword
                n if n.is_alphabetic() || n == '_' => {
                    let mut ident = String::new();
                    ident.push(self.c);
                    self.next();

                    if self.is_end() || (!self.c.is_alphanumeric() && self.c != '_') {
                        self.push(TokenKind::Ident(ident));
                        continue;
                    }

                    weird_while! {
                        ident.push(self.c),
                        !self.is_end() && (self.c.is_alphanumeric() || self.c == '_'),
                        self.next()
                    }

                    // let keyword: Result<Keyword, ()>;
                    // if Ok(keyword) = ident.try_into() {}
                    if let Ok(keyword) = ident.clone().try_into() {
                        self.push(TokenKind::Keyword(keyword));
                    } else {
                        self.push(TokenKind::Ident(ident));
                    }
                }

                // A string
                '"' => {
                    self.next();

                    let mut str = String::new();

                    weird_while! {
                        {
                            if self.c == '\\' {
                                escape_char!(self, str);
                            } else {
                                str.push(self.c);
                            }
                        },
                        !self.is_end() && self.c != '"',
                        self.next()
                    }

                    self.push(TokenKind::Str(str));
                }

                // A character
                '\'' => {
                    self.next();

                    let mut char = String::new();

                    weird_while! {
                        {
                            if self.c == '\\' {
                                escape_char!(self, char);
                            } else {
                                char.push(self.c);
                            }
                        },
                        !self.is_end() && self.c != '\'',
                        self.next()
                    }

                    if char.len() != 1 {
                        return Err(LexError::InvalidCharacterLiteral(self.generate_loc()));
                    }
                    self.push(TokenKind::Char(char.chars().nth(0).unwrap()));
                }

                // An integer, a float, or a hexadecimal number.
                n if n.is_ascii_digit() => {
                    if self.c == '0' && self.peek() == Some(&'x') {
                        // A hexadecimal number!
                        self.next();
                        self.next();

                        let mut hex = String::new();

                        if self.is_end() || !self.force_peek().is_ascii_hexdigit() {
                            return Err(LexError::InvalidHexadecimalNumber(self.generate_loc()));
                        }

                        weird_while! {
                            hex.push(self.c),
                            !self.is_end() && self.c.is_ascii_hexdigit(),
                            self.next()
                        }

                        if let Ok(hex) = i64::from_str_radix(&hex, 16) {
                            self.push(TokenKind::Int(hex));
                        } else {
                            return Err(LexError::InvalidHexadecimalNumber(self.generate_loc()));
                        }
                    } else {
                        // An integer or a float.
                        let mut num = String::new();
                        let mut has_dot = false;

                        weird_while! {
                            {
                                num.push(self.c);
                                if self.peek() == Some(&'.') {
                                    if has_dot {
                                        return Err(LexError::InvalidFloatingPointNumber(self.generate_loc()));
                                    } else {
                                        has_dot = true;
                                        num.push('.');
                                        self.next();
                                    }
                                }
                            },
                            !self.is_end() && (self.c.is_ascii_digit() || self.c == '.'),
                            self.next()
                        }

                        if has_dot {
                            if let Ok(float) = str::parse::<f64>(&num) {
                                self.push(TokenKind::Float(float));
                            } else {
                                return Err(LexError::InvalidFloatingPointNumber(
                                    self.generate_loc(),
                                ));
                            }
                        } else {
                            if let Ok(int) = str::parse::<i64>(&num) {
                                self.push(TokenKind::Int(int));
                            } else {
                                return Err(LexError::InvalidInteger(self.generate_loc()));
                            }
                        }
                    }
                }

                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.column_c = 1;
                }
                n if n.is_whitespace() => {}
                '(' => self.push(TokenKind::LeftParen),
                ')' => self.push(TokenKind::RightParen),
                '{' => self.push(TokenKind::LeftBrace),
                '}' => self.push(TokenKind::RightBrace),
                '[' => self.push(TokenKind::LeftBracket),
                ']' => self.push(TokenKind::RightBracket),
                '\\' => self.push(TokenKind::BackSlash),
                '+' => match self.force_peek() {
                    '=' => self.push_and_consume(TokenKind::PlusEq),
                    _ => self.push(TokenKind::Plus),
                },
                '-' => match self.force_peek() {
                    '=' => self.push_and_consume(TokenKind::MinusEq),
                    '>' => self.push_and_consume(TokenKind::MinusGreaterThan),
                    _ => self.push(TokenKind::Minus),
                },
                '*' => match self.force_peek() {
                    '=' => self.push_and_consume(TokenKind::MulEq),
                    _ => self.push(TokenKind::Mul),
                },
                '/' => match self.force_peek() {
                    '=' => self.push_and_consume(TokenKind::DivEq),
                    '/' => {
                        // Comment!
                        while self.c != '\n' && !self.is_end() {
                            self.next();
                        }
                        self.next();
                    }
                    _ => self.push(TokenKind::Div),
                },
                '%' => match self.force_peek() {
                    '=' => self.push_and_consume(TokenKind::RemEq),
                    _ => self.push(TokenKind::Rem),
                },
                ',' => self.push(TokenKind::Comma),
                '.' => self.push(TokenKind::Dot),
                '|' => {
                    if {
                        self.next();
                        self.c == '|'
                    } {
                        self.push(TokenKind::DoubleAmp);
                    } else {
                        return Err(LexError::InvalidToken {
                            expected: vec!["||"],
                            loc: self.generate_loc(),
                        });
                    }
                }
                '&' => {
                    if {
                        self.next();
                        self.c == '&'
                    } {
                        self.push(TokenKind::DoubleVertical);
                    } else {
                        return Err(LexError::InvalidToken {
                            expected: vec!["&&"],
                            loc: self.generate_loc(),
                        });
                    }
                }
                '>' => match self.force_peek() {
                    '=' => self.push_and_consume(TokenKind::GreaterThanOrEq),
                    _ => self.push(TokenKind::GreaterThan),
                },
                '<' => match self.force_peek() {
                    '=' => self.push_and_consume(TokenKind::LessThanOrEq),
                    _ => self.push(TokenKind::LessThan),
                },
                '!' => match self.force_peek() {
                    '=' => self.push_and_consume(TokenKind::BangEq),
                    _ => self.push(TokenKind::Bang),
                },
                '=' => match self.force_peek() {
                    '=' => self.push_and_consume(TokenKind::DoubleEq),
                    '>' => self.push_and_consume(TokenKind::EqGreaterThan),
                    _ => self.push(TokenKind::Eq),
                },
                ':' => match self.force_peek() {
                    ':' => self.push_and_consume(TokenKind::DoubleColon),
                    _ => self.push(TokenKind::Colon),
                },
                ';' => self.push(TokenKind::SemiColon),
                _ => return Err(LexError::InvalidCharacter(self.generate_loc())),
            }
            self.next();
        }
        Ok(self.tokens)
    }
}

impl<'src> Lexer<'src> {
    /// Create a new `Lexer` object.
    pub fn new(filename: Option<String>, source: &'src String) -> Self {
        Self {
            filename,
            source: source.chars().peekable(),
            tokens: Vec::new(),
            c: '\0',
            line: 1,
            column: 1,
            column_c: 1,
            offset: 0,
            length: 0,
        }
    }

    /// Check whether it reached the end of the source code or not.
    #[inline]
    fn is_end(&self) -> bool {
        self.c == '\0'
    }

    /// Advances the `source` iterator.
    /// May panic.
    fn next(&mut self) -> Option<char> {
        self.offset += 1;
        let n = self.source.next();
        self.c = n.unwrap_or('\0');
        self.length += 1;
        self.column_c += 1;
        return n;
    }

    /// Return a reference to the next character without consuming it.
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    /// Forcibly unwrap the next character using `peek` method.
    /// May panic, so use it with care!
    fn force_peek(&mut self) -> char {
        if let Some(c) = self.peek() {
            *c
        } else {
            // If there's no next character, this will just panic for now although
            // this is a very bad practice.
            panic!();
        }
    }

    /// Create a new `Token` and then append it to `tokens` vector.
    fn push(&mut self, kind: TokenKind) {
        let loc = self.generate_loc();
        let token = Token::new(kind, loc);
        self.length = 0;
        self.column = self.column_c;
        self.tokens.push(token);
    }

    /// Do whatever `push` method does and then `next`.
    fn push_and_consume(&mut self, kind: TokenKind) {
        self.push(kind);
        self.next();
    }

    /// Generate a `SourceCodeLocation` with the current location information stored in the lexer struct.
    fn generate_loc(&self) -> SourceCodeLocation {
        SourceCodeLocation {
            line: self.line,
            column: self.column,
            offset: self.offset,
            length: self.length - 1,
            filename: self.filename.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords_and_types() {
        let src = "if else match def let str int float struct enum impl for while self Self break return continue methods import export".to_string();
        let lexer = Lexer::new(None, &src);
        let tokens = lexer.tokenize();

        assert!(matches!(tokens, Ok(_)));

        let tokens = tokens.unwrap();
        assert_eq!(tokens.len(), 21);

        assert_eq!(tokens[0].kind, TokenKind::Keyword(Keyword::If));
        assert_eq!(tokens[1].kind, TokenKind::Keyword(Keyword::Else));
        assert_eq!(tokens[2].kind, TokenKind::Keyword(Keyword::Match));
        assert_eq!(tokens[3].kind, TokenKind::Keyword(Keyword::Def));
        assert_eq!(tokens[4].kind, TokenKind::Keyword(Keyword::Let));
        assert_eq!(tokens[5].kind, TokenKind::Keyword(Keyword::Str));
        assert_eq!(tokens[6].kind, TokenKind::Keyword(Keyword::Int));
        assert_eq!(tokens[7].kind, TokenKind::Keyword(Keyword::Float));
        assert_eq!(tokens[8].kind, TokenKind::Keyword(Keyword::Struct));
        assert_eq!(tokens[9].kind, TokenKind::Keyword(Keyword::Enum));
        assert_eq!(tokens[10].kind, TokenKind::Keyword(Keyword::Impl));
        assert_eq!(tokens[11].kind, TokenKind::Keyword(Keyword::For));
        assert_eq!(tokens[12].kind, TokenKind::Keyword(Keyword::While));
        assert_eq!(tokens[13].kind, TokenKind::Keyword(Keyword::LilSelf));
        assert_eq!(tokens[14].kind, TokenKind::Keyword(Keyword::BigSelf));
        assert_eq!(tokens[15].kind, TokenKind::Keyword(Keyword::Break));
        assert_eq!(tokens[16].kind, TokenKind::Keyword(Keyword::Return));
        assert_eq!(tokens[17].kind, TokenKind::Keyword(Keyword::Continue));
        assert_eq!(tokens[18].kind, TokenKind::Keyword(Keyword::Methods));
        assert_eq!(tokens[19].kind, TokenKind::Keyword(Keyword::Import));
        assert_eq!(tokens[20].kind, TokenKind::Keyword(Keyword::Export));
    }

    #[test]
    fn literals() {
        let src = "123 1.23 0xabc \"Hello, world\" '\\n' identifier".to_string();
        let lexer = Lexer::new(None, &src);
        let tokens = lexer.tokenize();

        assert!(matches!(tokens, Ok(_)));

        let tokens = tokens.unwrap();
        assert_eq!(tokens.len(), 6);

        assert_eq!(tokens[0].kind, TokenKind::Int(123));
        assert_eq!(tokens[1].kind, TokenKind::Float(1.23));
        assert_eq!(tokens[2].kind, TokenKind::Int(2748));
        assert_eq!(tokens[3].kind, TokenKind::Str("Hello, world".to_string()));
        assert_eq!(tokens[4].kind, TokenKind::Char('\n'));
        assert_eq!(tokens[5].kind, TokenKind::Ident("identifier".to_string()));
    }
}
