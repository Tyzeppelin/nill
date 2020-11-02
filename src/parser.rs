
use ast;
use ast::{Sexpr, Lexpr, Litteral, Epsilon};
use error;
use token::{Token, TokenType};

pub struct Parser{
    tokens: Vec<Token>,
    current: u32,
}


// S -> (L) | litteral
// L -> SL | epsilon

impl Parser {

    pub fn new(t: Vec<Token>) -> Self {
        Self { tokens: t, current: 0 }
    }

    fn s_expr(&mut self) -> Sexpr {
        return match self.match_next(TokenType::LeftParen) {
            true => {
                let l_expr = Sexpr::L(Box::new(self.l_expr()));
                self.consume(TokenType::RightParen, "Missing ')' at the end of S-expr");
                return l_expr
            },
            false => Sexpr::A(self.litteral())
        }
    }

    fn l_expr(&mut self) -> Lexpr {
        return match self.is_at_end() {
            true => Lexpr::Eps(self.epsilon()),
            false => {
                let s = Box::new(self.s_expr());
                let l = Box::new(self.l_expr());
                return Lexpr::SL(s, l)
            }
        }
    }

    fn litteral(&mut self) -> Litteral {
        return Litteral::Value(self.advance().get_type());
    }

    fn epsilon(&self) -> Epsilon {
        return Epsilon::Eps
    }

    fn consume(&mut self, tok: TokenType, message: &str) -> &Token{
        //if self.check(tok) {
            return self.advance()
        //} else {
            // throw error
        //    return Token {}
        //}
    }

    fn match_next(&mut self, tok: TokenType) -> bool {
        if self.check(tok) {
            self.advance();
            return true;
        } else {
            return false;
        }
    }

    fn check(&mut self, tok: TokenType) -> bool {
        return self.is_at_end() || self.peek().get_type() == tok
    }

    fn advance(&mut self) -> &Token{
        if ! self.is_at_end() {
            self.current = self.current + 1;
        }
        return self.previous()
    }

    fn previous(&mut self) -> &Token{
        //if self.current == 0 {
        //    error::error("?", "Yo can't get previous of 0th token yo.");
        //}
        return &self.tokens[self.current as usize]
    }

    fn peek(&mut self) -> &Token{
        return &self.tokens[self.current as usize]
    }

    fn is_at_end(&mut self) -> bool {
        return self.peek().get_type() == TokenType::Eof
    }
}


