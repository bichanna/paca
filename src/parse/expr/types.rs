use crate::parse::SourceCodeLocation;

/// This struct is for only for holding `loc`.
#[derive(Clone, Debug)]
pub struct TypeStruct {
    loc: SourceCodeLocation,
    r#type: Type,
}

impl Into<SourceCodeLocation> for TypeStruct {
    fn into(self) -> SourceCodeLocation {
        self.loc
    }
}

/// This enum represents all of the different possible types.
#[derive(Clone, Debug)]
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

    /// `\(...) -> type` type.
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
