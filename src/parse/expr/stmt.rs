use crate::parse::expr::expr::Expr;
use crate::parse::expr::ExprStruct;
use crate::parse::SourceCodeLocation;

#[derive(Clone, Debug)]
pub struct StmtStruct {
    pub(crate) loc: SourceCodeLocation,
    pub(crate) stmt: Stmt,
}

impl Into<SourceCodeLocation> for StmtStruct {
    fn into(self) -> SourceCodeLocation {
        self.loc
    }
}

/// All statement types.
#[derive(Clone, Debug)]
pub enum Stmt {
    /// An expression
    Expr(ExprStruct),

    /// An if-then-else statement.
    If(Expr, Box<Self>, Box<Self>, Option<Box<Self>>),

    /// A while loop. All for loops are converted to this.
    While(Expr, Box<Self>),

    /// A break statement.
    Break,

    /// A continue statement.
    Continue,

    /// A function definition.
    Function(String, Expr),

    /// A return statement.
    Return(Option<Expr>),
}
