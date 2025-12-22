use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Token {
    Int(String, u32, usize),
    Float(String, usize),
    Name(String, usize),
    Plus(usize),
    Minus(usize),
    Mul(usize),
    Div(usize),
    Mod(usize),
    Pow(usize),
    Lparen(usize),
    Rparen(usize),
    Comma(usize),
    Eof(usize),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Int(s, radix, _) => write!(f, "Token::Int({}, radix={})", s, radix),
            Token::Float(s, _) => write!(f, "Token::FLOAT({})", s),
            Token::Name(s, _) => write!(f, "Token::NAME({})", s),

            Token::Plus(_) => write!(f, "Token::PLUS"),
            Token::Minus(_) => write!(f, "Token::MINUS"),
            Token::Mul(_) => write!(f, "Token::MUL"),
            Token::Div(_) => write!(f, "Token::DIV"),
            Token::Mod(_) => write!(f, "Token::MOD"),
            Token::Pow(_) => write!(f, "Token::POW"),
            Token::Lparen(_) => write!(f, "Token::LPAREN"),
            Token::Rparen(_) => write!(f, "Token::RPAREN"),
            Token::Comma(_) => write!(f, "Token::COMMA"),
            Token::Eof(_) => write!(f, "Token::EOF"),
        }
    }
}
