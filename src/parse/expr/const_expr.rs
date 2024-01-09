use crate::parse::expr::expr::Expr;

pub enum ConstExpr {
    Identifier(String),
    String(String),
    Character(char),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Expr>),
    Tuple(Vec<Expr>),
}
