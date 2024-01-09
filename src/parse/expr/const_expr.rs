use crate::parse::expr::expr::Expr;

#[derive(Clone, Debug)]
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
