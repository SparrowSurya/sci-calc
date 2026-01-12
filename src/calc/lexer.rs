use crate::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum LexerErr {
    IllegalChar(char, usize),
    InvalidValue(usize),
}

pub trait Tokeniser {
    fn next_token(&mut self) -> Result<Token, LexerErr>;
    fn peek_token(&mut self) -> Result<Token, LexerErr>;
}

#[derive(Debug)]
pub struct Lexer {
    expr: String,
    cursor: usize,
    next: Option<Result<Token, LexerErr>>,
}

impl Tokeniser for Lexer {
    fn next_token(&mut self) -> Result<Token, LexerErr> {
        match self.next.clone() {
            Option::None => self.read_next_token(),
            Option::Some(tok) => {
                self.next = Option::None;
                tok
            }
        }
    }

    fn peek_token(&mut self) -> Result<Token, LexerErr> {
        match self.next.clone() {
            Option::Some(tok) => tok.clone(),
            Option::None => {
                let tok = self.read_next_token();
                self.next = Option::Some(tok.clone());
                tok
            }
        }
    }
}

impl Lexer {
    pub fn new(expr: String) -> Lexer {
        Lexer {
            expr,
            cursor: 0,
            next: Option::None,
        }
    }

    pub fn tokenise(&mut self) -> Result<Vec<Token>, LexerErr> {
        let mut found_eof_token = false;
        let mut tokens: Vec<Token> = Vec::new();

        while !found_eof_token {
            let token = self.next_token()?;
            match token {
                Token::Eof(_) => found_eof_token = true,
                _ => {}
            }
            tokens.push(token);
        }

        return Ok(tokens);
    }

    fn read_next_token(&mut self) -> Result<Token, LexerErr> {
        let mut ch = self.curr_char();

        while ch == ' ' {
            ch = self.advance();
        }

        let start = self.cursor;

        if ch.is_digit(10) {

            let _ch = self.peek_char();
            let _radix = if ch == '0' { self.read_radix(_ch) } else { Option::None };
            if let Option::Some(r) = _radix {
                self.advance();
                ch = self.advance();
                while ch.is_digit(r) {
                    ch = self.advance();
                }

                let value = &self.expr[start+2..self.cursor];
                if value.is_empty() {
                    return Err(LexerErr::InvalidValue(self.cursor))
                }
                return Ok(Token::Int(value.to_string(), r, start));
            }

            ch = self.advance();
            while ch.is_digit(10) {
                ch = self.advance();
            }

            if ch != '.' && ch != 'e' && ch != 'E' {
                let value = &self.expr[start..self.cursor];
                return Ok(Token::Int(value.to_string(), 10, start));
            }

            if ch == '.' {
                ch = self.advance();
                while ch.is_digit(10) {
                    ch = self.advance();
                }
            }

            if ch == 'e' || ch == 'E' {
                ch = self.advance();
                if ch == '+' || ch == '-' {
                    ch = self.advance();
                }
                while ch.is_digit(10) {
                    ch = self.advance();
                }
            }

            let value = &self.expr[start..self.cursor];
            return Ok(Token::Float(value.to_string(), start));
        }

        if ch.is_ascii_alphabetic() {
            ch = self.advance();
            while ch.is_ascii_alphabetic() {
                ch = self.advance();
            }
            let value = &self.expr[start..self.cursor];
            return Ok(Token::Name(value.to_string(), start));
        }

        if ch == '+' {
            self.advance();
            return Ok(Token::Plus(start));
        }

        if ch == '-' {
            self.advance();
            return Ok(Token::Minus(start));
        }

        if ch == '*' {
            return match self.advance() {
                '*' => {
                    self.advance();
                    return Ok(Token::Pow(start));
                },
                _ => Ok(Token::Mul(start)),
            };
        }

        if ch == '/' {
            self.advance();
            return Ok(Token::Div(start));
        }

        if ch == '%' {
            self.advance();
            return Ok(Token::Mod(start));
        }

        if ch == '&' {
            self.advance();
            return Ok(Token::And(start));
        }

        if ch == '|' {
            self.advance();
            return Ok(Token::Or(start));
        }

        if ch == '~' {
            self.advance();
            return Ok(Token::Not(start));
        }

        if ch == '^' {
            self.advance();
            return Ok(Token::Xor(start));
        }

        if ch == ',' {
            self.advance();
            return Ok(Token::Comma(start));
        }

        if ch == '(' {
            self.advance();
            return Ok(Token::Lparen(start));
        }

        if ch == ')' {
            self.advance();
            return Ok(Token::Rparen(start));
        }

        if ch == '\0' {
            return Ok(Token::Eof(start));
        }

        return Err(LexerErr::IllegalChar(ch, start));
    }

    fn curr_char(&mut self) -> char {
        self.expr.chars().nth(self.cursor).unwrap_or('\0')
    }

    fn read_radix(&self, ch: char) -> Option<u32> {
        match ch {
            'b' | 'B' => Option::Some(2),
            'o' | 'O' => Option::Some(8),
            'x' | 'X' => Option::Some(16),
            _ => Option::None,
        }
    }

    fn peek_char(&mut self) -> char {
        self.expr.chars().nth(self.cursor+1).unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        if self.cursor >= self.expr.len() {
            return '\0';
        }
        self.cursor += 1;
        self.curr_char()
    }
}

#[allow(dead_code)]
pub fn tokenise(expr: String) -> Result<Vec<Token>, LexerErr> {
    let mut lexer = Lexer::new(expr);
    lexer.tokenise()
}

// TODO - tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenise_valid_operators() {
        assert_eq!(
            tokenise("+".to_string()),
            Ok(vec![Token::Plus(0), Token::Eof(1)])
        );
        assert_eq!(
            tokenise("-".to_string()),
            Ok(vec![Token::Minus(0), Token::Eof(1)])
        );
        assert_eq!(
            tokenise("*".to_string()),
            Ok(vec![Token::Mul(0), Token::Eof(1)])
        );
        assert_eq!(
            tokenise("/".to_string()),
            Ok(vec![Token::Div(0), Token::Eof(1)])
        );
        assert_eq!(
            tokenise("%".to_string()),
            Ok(vec![Token::Mod(0), Token::Eof(1)])
        );
        assert_eq!(
            tokenise("**".to_string()),
            Ok(vec![Token::Pow(0), Token::Eof(1)])
        );
        assert_eq!(
            tokenise("&".to_string()),
            Ok(vec![Token::And(0), Token::Eof(1)])
        );
        assert_eq!(
            tokenise("|".to_string()),
            Ok(vec![Token::Or(0), Token::Eof(1)])
        );
        assert_eq!(
            tokenise("~".to_string()),
            Ok(vec![Token::Not(0), Token::Eof(1)])
        );
        assert_eq!(
            tokenise("^".to_string()),
            Ok(vec![Token::Xor(0), Token::Eof(1)])
        );
    }

    #[test]
    fn tokenise_valid_delimiters() {
        assert_eq!(
            tokenise("(".to_string()),
            Ok(vec![Token::Lparen(0), Token::Eof(1)])
        );
        assert_eq!(
            tokenise(")".to_string()),
            Ok(vec![Token::Rparen(0), Token::Eof(1)])
        );
        assert_eq!(
            tokenise(",".to_string()),
            Ok(vec![Token::Comma(0), Token::Eof(1)])
        );
    }

    #[test]
    fn tokenise_valid_integer() {
        assert_eq!(
            tokenise("23".to_string()),
            Ok(vec![Token::Int("23".to_string(), 10, 0), Token::Eof(2)])
        );
        assert_eq!(
            tokenise("0023".to_string()),
            Ok(vec![Token::Int("0023".to_string(), 10, 0), Token::Eof(4)])
        );
        assert_eq!(
            tokenise("0230".to_string()),
            Ok(vec![Token::Int("0230".to_string(), 10, 0), Token::Eof(4)])
        );
    }

    #[test]
    fn tokenise_valid_float() {
        assert_eq!(
            tokenise("23.5".to_string()),
            Ok(vec![Token::Float("23.5".to_string(), 0), Token::Eof(4)])
        );
        assert_eq!(
            tokenise("23.500".to_string()),
            Ok(vec![Token::Float("23.500".to_string(), 0), Token::Eof(6)])
        );
        assert_eq!(
            tokenise("0.05".to_string()),
            Ok(vec![Token::Float("0.05".to_string(), 0), Token::Eof(4)])
        );
    }

    #[test]
    fn tokenise_valid_scientific_format() {
        assert_eq!(
            tokenise("5e10".to_string()),
            Ok(vec![Token::Float("5e10".to_string(), 0), Token::Eof(4)])
        );
        assert_eq!(
            tokenise("20.0E3".to_string()),
            Ok(vec![Token::Float("20.0E3".to_string(), 0), Token::Eof(6)])
        );
        assert_eq!(
            tokenise("5e+1".to_string()),
            Ok(vec![Token::Float("5e+1".to_string(), 0), Token::Eof(4)])
        );
        assert_eq!(
            tokenise("5e-10".to_string()),
            Ok(vec![Token::Float("5e-10".to_string(), 0), Token::Eof(5)])
        );
    }
}
