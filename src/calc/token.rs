use std::fmt;

#[derive(PartialEq, Clone)]
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

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::INT(s, _) => write!(f, "Token::Int({})", s),
            Token::FLOAT(s, _) => write!(f, "Token::FLOAT({})", s),
            Token::NAME(s, _) => write!(f, "Token::NAME({})", s),

            Token::PLUS(_) => write!(f, "Token::PLUS"),
            Token::MINUS(_) => write!(f, "Token::MINUS"),
            Token::MUL(_) => write!(f, "Token::MUL"),
            Token::DIV(_) => write!(f, "Token::DIV"),
            Token::MOD(_) => write!(f, "Token::MOD"),
            Token::POW(_) => write!(f, "Token::POW"),
            Token::LPAREN(_) => write!(f, "Token::LPAREN"),
            Token::RPAREN(_) => write!(f, "Token::RPAREN"),
            Token::COMMA(_) => write!(f, "Token::COMMA"),
            Token::EOF(_) => write!(f, "Token::EOF"),
        }
    }
}
