/// Idea taken from https://gist.github.com/huonw/8435502.
macro_rules! weird_while {
    ($body:expr, $cond:expr) => {
        while {
            $body;
            $cond
        } {}
    };

    ($body:expr, $cond:expr, $special:expr) => {
        while {
            $body;
            if $cond {
                $special;
            }
            $cond
        } {}
    };
}

/// Boilerplate for escaping characters.
macro_rules! escape_char {
    ($self:expr, $str:expr) => {
        match $self.peek() {
            Some(&'\\') => {
                $str.push($self.c);
                $self.next();
            }
            Some(&'0') => {
                $str.push('\0');
                $self.next();
            }
            Some(&'"') => {
                $str.push('"');
                $self.next();
            }
            Some(&'n') => {
                $str.push('\n');
                $self.next();
            }
            Some(&'r') => {
                $str.push('\r');
                $self.next();
            }
            Some(&'t') => {
                $str.push('\t');
                $self.next();
            }
            Some(_) | None => {}
        }
    };
}

pub(crate) use {escape_char, weird_while};

/// This trait is for error enums and structs to properly format error messages.
pub trait GenerateErrorMessage: Clone {
    fn generate_error_message(self, source_code: &String) -> String;
}
