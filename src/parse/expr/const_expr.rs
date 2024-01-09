pub enum ConstExpr {
    Identifier(String),
    String(String),
    Character(char),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}
