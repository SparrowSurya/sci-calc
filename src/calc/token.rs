#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    INT(String, usize),
    FLOAT(String, usize),
    NAME(String, usize),
    PLUS(usize),
    MINUS(usize),
    MUL(usize),
    DIV(usize),
    MOD(usize),
    POW(usize),
    LPAREN(usize),
    RPAREN(usize),
    COMMA(usize),
    EOF(usize),
}
