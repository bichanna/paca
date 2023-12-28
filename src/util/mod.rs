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

macro_rules! escape_char {
    ($self:expr, $str:expr) => {
        match $self.force_peek() {
            '\\' => {
                $str.push($self.c);
                $self.next();
            }
            '0' => {
                $str.push('\0');
                $self.next();
            }
            '"' => {
                $str.push('"');
                $self.next();
            }
            'n' => {
                $str.push('\n');
                $self.next();
            }
            'r' => {
                $str.push('\r');
                $self.next();
            }
            't' => {
                $str.push('\t');
                $self.next();
            }
            _ => {}
        }
    };
}

pub(crate) use {escape_char, weird_while};
