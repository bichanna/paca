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

pub(crate) use weird_while;
