use crate::calc::token::Token;
use crate::calc::lexer::{Tokeniser, LexerErr};
use crate::calc::common::{Float, Integer};
use crate::calc::nodes::{Atom, BinOp, Expr, UnOp};


#[derive(Debug, PartialEq)]
pub enum ParserErr {
    SyntaxError(String),
    ParseIntError(String),
    ParseFloatError(String),
    LexerErr(LexerErr),
}


#[derive(Debug)]
pub struct Parser<T: Tokeniser> {
    lexer: T,
}

impl<T> Parser<T>
where
    T: Tokeniser
{
    pub fn new(lexer: T) -> Parser<T> {
        Parser { lexer }
    }

    fn consume(&mut self) -> Result<Token, ParserErr> {
        match self.lexer.next_token() {
            Result::Ok(t) => {
                Result::Ok(t)
            },
            Result::Err(e) => Result::Err(ParserErr::LexerErr(e))
        }
    }

    fn peek(&mut self) -> Result<Token, ParserErr> {
        match self.lexer.peek_token() {
            Result::Ok(t) => Result::Ok(t),
            Result::Err(e) => Result::Err(ParserErr::LexerErr(e))
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParserErr> {
        self.parse_expr(0.0)
    }

    fn parse_expr(&mut self, min_bp: f32) -> Result<Expr, ParserErr> {
        let mut lhs = match self.consume()? {
            Token::Int(v, _) => Expr::Atom(self.parse_int(v)?),
            Token::Float(v, _) => Expr::Atom(self.parse_float(v)?),
            Token::Name(v, _) => match self.peek()? {
                Token::Lparen(_) => Expr::Atom(Atom::Func(v, self.parse_args()?)),
                _ => Expr::Atom(Atom::Const(v)),
            },
            Token::Lparen(_) => {
                let expr = self.parse_expr(0.0);
                return match self.consume()? {
                    Token::Rparen(_) => expr,
                    _ => {
                        return Result::Err(ParserErr::SyntaxError(
                            "expected ')' character".to_string(),
                        ))
                    }
                };
            }
            Token::Plus(_) => Expr::UnOp(UnOp::Pos, Box::new(self.parse_unary()?)),
            Token::Minus(_) => Expr::UnOp(UnOp::Neg, Box::new(self.parse_unary()?)),
            _ => {
                return Result::Err(ParserErr::SyntaxError(
                    "SyntaxError".to_string(),
                ))
            }
        };

        loop {
            let op = match self.peek()? {
                Token::Plus(_) => BinOp::Plus,
                Token::Minus(_) => BinOp::Minus,
                Token::Mul(_) => BinOp::Mul,
                Token::Div(_) => BinOp::Div,
                Token::Mod(_) => BinOp::Mod,
                Token::Pow(_) => BinOp::Pow,
                Token::Eof(_) => break,
                Token::Comma(_) => break,
                Token::Rparen(_) => break,
                _ => {
                    return Result::Err(ParserErr::SyntaxError(
                        "expected operator".to_string(),
                    ));
                }
            };

            let (lbp, rbp) = self.infix_binding_power(op.clone());
            if lbp < min_bp {
                break;
            }

            self.consume()?;
            let rhs = self.parse_expr(rbp)?;
            lhs = Expr::BinOp(op, Box::new(lhs), Box::new(rhs));
        }

        Result::Ok(lhs)
    }

    fn parse_int(&self, value: String) -> Result<Atom, ParserErr> {
        match value.parse::<Integer>() {
            Result::Ok(v) => Result::Ok(Atom::Int(v)),
            Result::Err(e) => Result::Err(ParserErr::ParseIntError(e.to_string())),
        }
    }

    fn parse_float(&self, value: String) -> Result<Atom, ParserErr> {
        match value.parse::<Float>() {
            Result::Ok(v) => Result::Ok(Atom::Float(v)),
            Result::Err(e) => {
                Result::Err(ParserErr::ParseFloatError(e.to_string()))
            }
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, ParserErr> {
        match self.consume()? {
            Token::Lparen(_) => {
                let mut args: Vec<Expr> = Vec::new();
                loop {
                    match self.peek()? {
                        Token::Rparen(_) => {
                            self.consume()?;
                            break;
                        }
                        Token::Comma(_) => {
                            self.consume()?;
                            continue;
                        }
                        _ => args.push(self.parse_expr(0.0)?),
                    };
                }
                Result::Ok(args)
            }
            _ => Result::Err(ParserErr::SyntaxError(
                "SyntaxError".to_string(),
            )),
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, ParserErr> {
        match self.consume()? {
            Token::Int(v, _) => Result::Ok(Expr::Atom(self.parse_int(v)?)),
            Token::Float(v, _) => Result::Ok(Expr::Atom(self.parse_float(v)?)),
            Token::Name(v, _) => match self.peek()? {
                Token::Lparen(_) => Result::Ok(Expr::Atom(Atom::Func(v, self.parse_args()?))),
                _ => Result::Ok(Expr::Atom(Atom::Const(v))),
            },
            Token::Lparen(_) => {
                let expr = self.parse_expr(0.0)?;
                match self.consume()? {
                    Token::Rparen(_) => Result::Ok(expr),
                    _ => Result::Err(ParserErr::SyntaxError(
                        "expected ')'".to_string(),
                    )),
                }
            }
            Token::Plus(_) => Result::Ok(Expr::UnOp(UnOp::Pos, Box::new(self.parse_unary()?))),
            Token::Minus(_) => Result::Ok(Expr::UnOp(UnOp::Neg, Box::new(self.parse_unary()?))),
            _ => Result::Err(ParserErr::SyntaxError(
                "expected primary expression".to_string(),
            )),
        }
    }

    fn infix_binding_power(&self, op: BinOp) -> (f32, f32) {
        match op {
            BinOp::Plus | BinOp::Minus => (1.0, 1.1),
            BinOp::Mul | BinOp::Div | BinOp::Mod => (2.0, 2.1),
            BinOp::Pow => (3.1, 3.0),
        }
    }
}
