use token::{ Token, TokenType, BasicType };
use error;
use misc;

use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: u32,
    current: u32,
    line: u32,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(s: String) -> Self{
        let mut kw = HashMap::new();
        kw.insert("true".to_string(), TokenType::True);
        kw.insert("false".to_string(), TokenType::False);
        kw.insert("and".to_string(), TokenType::And);
        kw.insert("or".to_string(), TokenType::Or);
        kw.insert("nil".to_string(), TokenType::Nil);
        kw.insert("fun".to_string(), TokenType::Fun);
        kw.insert("let".to_string(), TokenType::Let);
        kw.insert("cond".to_string(), TokenType::Cond);
        kw.insert("cons".to_string(), TokenType::Cons);

        Self { source: s, tokens: Vec::new(), start: 0, current: 0, line: 1, keywords: kw}
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {

        while !self.is_at_end() {

            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "eof".to_string(), BasicType::Nill, self.line));
        return &self.tokens;
    }

    fn scan_token(&mut self) {

        let c: char = self.advance();
        match c {
            '(' => self.add_non_lit_token(TokenType::LeftParen),
            ')' => self.add_non_lit_token(TokenType::RightParen),
            '{' => self.add_non_lit_token(TokenType::LeftBrace),
            '}' => self.add_non_lit_token(TokenType::RightBrace),
            ',' => self.add_non_lit_token(TokenType::Comma),
            '.' => self.add_non_lit_token(TokenType::Dot),
            '-' => self.add_non_lit_token(TokenType::Minus),
            '+' => self.add_non_lit_token(TokenType::Plus),
            ';' => self.add_non_lit_token(TokenType::Semicolon),
            '*' => self.add_non_lit_token(TokenType::Star),

            '!' if self.match_next('=') => self.add_non_lit_token(TokenType::BangEqual),
            '!' => self.add_non_lit_token(TokenType::Bang),
            '=' if self.match_next('=') => self.add_non_lit_token(TokenType::EqualEqual),
            '=' => self.add_non_lit_token(TokenType::Equal),
            '<' if self.match_next('=')=> self.add_non_lit_token(TokenType::LessEqual),
            '<' => self.add_non_lit_token(TokenType::Less),
            '>' if self.match_next('=') => self.add_non_lit_token(TokenType::GreaterEqual),
            '>' => self.add_non_lit_token(TokenType::Greater),

            '/' if self.match_next('/') => {while self.peek() != '\n' && !self.is_at_end() {self.advance();} },
            '/' => self.add_non_lit_token(TokenType::Slash),

            ' '|'\r'|'\t' => {},
            '\n' => self.line = self.line + 1,

            '"' => self.match_string(),

            _   => {
                if misc::is_digit(c) {
                    self.match_number()
                } else if misc::is_alpha(c) {
                    self.match_identifier()
                } else {
                    error::error(self.line, "Unexpected character")
                }
            }
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current as usize).unwrap() != expected  {
            return false
        } else {
        self.current = self.current + 1;
        return true
        }
    }

    fn match_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            } else {
                self.advance();
            }
        }
        if self.is_at_end() {
            error::error(self.line, "Unterminated string!");
        } else {
            self.advance();
            let str_beg: usize = self.start as usize + 1;
            let str_end: usize = self.current as usize - 1;
            //let value: String = self.source[str_beg..str_end].to_string();
            self.add_token(TokenType::String, BasicType::String(self.source[str_beg..str_end].to_string()));
        }
    }

    fn match_number(&mut self) {
        while misc::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && misc::is_digit(self.peek_next()) {
            self.advance();
            while misc::is_digit(self.peek()) {
                self.advance();
            }
        }
        let str_beg: usize = self.start as usize;
        let str_end: usize = self.current as usize;
        self.add_token(TokenType::Number, BasicType::Number(self.source[str_beg..str_end].to_string().parse::<f32>().unwrap()));
    }

    fn match_identifier(&mut self) {
        while misc::is_alphanum(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start as usize..self.current as usize];
        let token_type = match self.keywords.get(text) {
            Some(t) => *t,
            None => TokenType::Identifier
        };
        self.add_non_lit_token(token_type);
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }
        return self.source.chars().nth(self.current as usize).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current as usize + 1 >= self.source.len() {
            return '\0'
        }
        return self.source.chars().nth(self.current as usize + 1).unwrap()
    }

    fn advance(&mut self) -> char {
        self.current = self.current + 1;
        return self.source.chars().nth(self.current as usize - 1).unwrap()
    }

    fn add_non_lit_token(&mut self, t: TokenType) {
        self.add_token(t, BasicType::Nill);
    }

    fn add_token(&mut self, t: TokenType, lit: BasicType) {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(t, text.to_string(), lit, self.line));
    }

    fn is_at_end(&self) -> bool {
        return self.current as usize >= self.source.len()
    }
 }
