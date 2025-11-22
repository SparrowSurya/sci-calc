use crate::calc::common::{Float, Integer};
use crate::calc::nodes::{Atom, BinOp, Expr, UnOp};
use crate::calc::token::Token;

#[derive(Debug, PartialEq)]
pub enum ParserErr {
    SyntaxError(String, usize),
    ParseIntError(String, usize),
    ParseFloatError(String, usize),
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, cursor: 0 }
    }

    /// Return the current token and move cursor
    fn consume(&mut self) -> Token {
        let token = self.current_token();
        self.advance();
        return token;
    }

    /// read current token
    fn current_token(&self) -> Token {
        self.tokens.get(self.cursor).unwrap().clone()
    }

    /// move to next token if possible
    /// stops at EOF token
    fn advance(&mut self) {
        if self.cursor + 1 < self.tokens.len() {
            self.cursor += 1;
        }
    }

    /// parse expression
    pub fn parse(&mut self) -> Result<Expr, ParserErr> {
        self.parse_expr(0.0)
    }

    fn parse_expr(&mut self, min_bp: f32) -> Result<Expr, ParserErr> {
        let mut lhs = match self.consume() {
            Token::INT(v, _) => Expr::ATOM(self.parse_int(v)?),
            Token::FLOAT(v, _) => Expr::ATOM(self.parse_float(v)?),
            Token::NAME(v, _) => match self.current_token() {
                Token::LPAREN(_) => Expr::ATOM(Atom::FUNC(v, self.parse_args()?)),
                _ => Expr::ATOM(Atom::CONST(v)),
            },
            Token::LPAREN(_) => {
                let expr = self.parse_expr(0.0);
                return match self.consume() {
                    Token::RPAREN(_) => expr,
                    _ => {
                        return Result::Err(ParserErr::SyntaxError(
                            "expected ')' character".to_string(),
                            self.cursor - 1,
                        ))
                    }
                };
            }
            Token::PLUS(_) => Expr::UNOP(UnOp::POS, Box::new(self.parse_unary()?)),
            Token::MINUS(_) => Expr::UNOP(UnOp::NEG, Box::new(self.parse_unary()?)),
            _ => {
                return Result::Err(ParserErr::SyntaxError(
                    "SyntaxError".to_string(),
                    self.cursor - 1,
                ))
            }
        };

        loop {
            let op = match self.current_token() {
                Token::PLUS(_) => BinOp::PLUS,
                Token::MINUS(_) => BinOp::MINUS,
                Token::MUL(_) => BinOp::MUL,
                Token::DIV(_) => BinOp::DIV,
                Token::MOD(_) => BinOp::MOD,
                Token::POW(_) => BinOp::POW,
                Token::EOF(_) => break,
                Token::COMMA(_) => break,
                Token::RPAREN(_) => break,
                _ => {
                    return Result::Err(ParserErr::SyntaxError(
                        "expected operator".to_string(),
                        self.cursor,
                    ));
                }
            };

            let (lbp, rbp) = self.infix_binding_power(op.clone());
            if lbp < min_bp {
                break;
            }

            self.consume();
            let rhs = self.parse_expr(rbp)?;
            lhs = Expr::BINOP(op, Box::new(lhs), Box::new(rhs));
        }

        Result::Ok(lhs)
    }

    fn parse_int(&self, value: String) -> Result<Atom, ParserErr> {
        match value.parse::<Integer>() {
            Result::Ok(v) => Result::Ok(Atom::INT(v)),
            Result::Err(e) => Result::Err(ParserErr::ParseIntError(e.to_string(), self.cursor - 1)),
        }
    }

    fn parse_float(&self, value: String) -> Result<Atom, ParserErr> {
        match value.parse::<Float>() {
            Result::Ok(v) => Result::Ok(Atom::FLOAT(v)),
            Result::Err(e) => {
                Result::Err(ParserErr::ParseFloatError(e.to_string(), self.cursor - 1))
            }
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, ParserErr> {
        match self.consume() {
            Token::LPAREN(_) => {
                let mut args: Vec<Expr> = Vec::new();
                loop {
                    match self.current_token() {
                        Token::RPAREN(_) => {
                            self.consume();
                            break;
                        }
                        Token::COMMA(_) => {
                            self.consume();
                            continue;
                        }
                        _ => args.push(self.parse_expr(0.0)?),
                    };
                }
                Result::Ok(args)
            }
            _ => Result::Err(ParserErr::SyntaxError(
                "SyntaxError".to_string(),
                self.cursor - 1,
            )),
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, ParserErr> {
        match self.consume() {
            Token::INT(v, _) => Result::Ok(Expr::ATOM(self.parse_int(v)?)),
            Token::FLOAT(v, _) => Result::Ok(Expr::ATOM(self.parse_float(v)?)),
            Token::NAME(v, _) => match self.current_token() {
                Token::LPAREN(_) => Result::Ok(Expr::ATOM(Atom::FUNC(v, self.parse_args()?))),
                _ => Result::Ok(Expr::ATOM(Atom::CONST(v))),
            },
            Token::LPAREN(_) => {
                let expr = self.parse_expr(0.0)?;
                match self.consume() {
                    Token::RPAREN(_) => Result::Ok(expr),
                    _ => Result::Err(ParserErr::SyntaxError(
                        "expected ')'".to_string(),
                        self.cursor - 1,
                    )),
                }
            }
            Token::PLUS(_) => Result::Ok(Expr::UNOP(UnOp::POS, Box::new(self.parse_unary()?))),
            Token::MINUS(_) => Result::Ok(Expr::UNOP(UnOp::NEG, Box::new(self.parse_unary()?))),
            _ => Result::Err(ParserErr::SyntaxError(
                "expected primary expression".to_string(),
                self.cursor - 1,
            )),
        }
    }

    fn infix_binding_power(&self, op: BinOp) -> (f32, f32) {
        match op {
            BinOp::PLUS | BinOp::MINUS => (1.0, 1.1),
            BinOp::MUL | BinOp::DIV | BinOp::MOD => (2.0, 2.1),
            BinOp::POW => (3.1, 3.0),
        }
    }
}
