use crate::parse::expr::const_expr::ConstExpr;
use crate::parse::expr::stmt::Stmt;
use crate::parse::expr::types::TypeStruct;
use crate::parse::SourceCodeLocation;

/// All binary operations.
#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Div,
    Mul,
    Rem,
    GreaterThan,
    LessThan,
    GreaterThanOrEq,
    LessThanOrEq,
    NotEq,
    Eq,
    And,
    Or,
}

/// All unary operations.
#[derive(Clone, Copy, Debug)]
pub enum UnaryOp {
    NegBool,
    NegNum,
}

#[derive(Clone, Debug)]
pub struct ExprStruct {
    loc: SourceCodeLocation,
    expr: Expr,
}

impl Into<SourceCodeLocation> for ExprStruct {
    fn into(self) -> SourceCodeLocation {
        self.loc
    }
}
/// All expression types.
#[derive(Clone, Debug)]
pub enum Expr {
    /// A constant expression.
    Const(ConstExpr),

    /// Binary operation.
    Binary(Box<Self>, BinaryOp, Box<Self>),

    /// Grouping expression.
    Group(Box<Self>),

    /// Unary operations.
    Unary(UnaryOp, Box<Self>),

    /// A block of expressions. The last statement expression in the block is the value of the type.
    Block(Vec<Stmt>),

    /// If-then-else expression.
    If(Box<Self>, Box<Self>, Option<Box<Self>>),

    /// A function, without a name. The type of the function is the type of the expression or whatever `return` returns.
    Function(Vec<(String, TypeStruct)>, Box<Self>, Option<TypeStruct>),

    /// Apply a function with arguments.
    Apply(Box<Self>, Vec<Self>),

    /// Index an array or a tuple with an expression that evaluates to an integer at runtime.
    Index(Box<Self>, ConstExpr),
}
