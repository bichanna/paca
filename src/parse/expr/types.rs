/// This enum represents all of the different possible types.
#[derive(Clone, Copy, Debug)]
pub enum Type {
    /// `str` type.
    String,

    /// `int` type.
    Int,

    /// `float` type.
    Float,

    /// `bool` type.
    Bool,

    /// `(...)` type.
    Tuple(Vec<Type>),

    /// `[]type` type.
    Array(Box<Type>),

    /// `enum` type.
    Enum(String),

    /// `struct` type.
    Struct(String),

    /// `def(...) -> type` type.
    Func {
        /// Function parameters.
        args: Vec<Type>,
        /// The return type of the function.
        return_type: Option<Box<Type>>,
    },

    /// Method for `struct` or `enum`.
    Method {
        /// The parent type.
        boss: Box<Type>,
        /// Method parameters.
        args: Vec<Type>,
        /// The return type of the method.
        return_type: Option<Box<Type>>,
    },

    /// `Self` type.
    BigSelf,

    /// `self`.
    LilSelf,
}
