// src/lexer.rs

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Let,
    Fn,
    If,
    Else,
    Return,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        match self.current_char() {
            '+' => self.advance_and_return(Token::Plus),
            '-' => self.advance_and_return(Token::Minus),
            '*' => self.advance_and_return(Token::Asterisk),
            '/' => self.advance_and_return(Token::Slash),
            '=' => self.advance_and_return(Token::Equals),
            '(' => self.advance_and_return(Token::LeftParen),
            ')' => self.advance_and_return(Token::RightParen),
            '{' => self.advance_and_return(Token::LeftBrace),
            '}' => self.advance_and_return(Token::RightBrace),
            ';' => self.advance_and_return(Token::Semicolon),
            '"' => self.read_string(),
            c if c.is_alphabetic() => self.read_identifier(),
            c if c.is_digit(10) => self.read_number(),
            _ => panic!("Unexpected character: {}", self.current_char()),
        }
    }

    fn current_char(&self) -> char {
        self.input[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn advance_and_return(&mut self, token: Token) -> Token {
        self.advance();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        //need to understand the below line in depth
        let identifier = self.input[start..self.position].iter().collect::<String>();
        match identifier.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Identifier(identifier),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        let mut is_float = false;
        while self.position < self.input.len() && (self.current_char().is_digit(10) || self.current_char() == '.') {
            if self.current_char() == '.' {
                is_float = true;
            }
            self.advance();
        }
        let number_str = self.input[start..self.position].iter().collect::<String>();
        if is_float {
            Token::Float(number_str.parse().unwrap())
        } else {
            Token::Integer(number_str.parse().unwrap())
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // Skip opening quote
        let start = self.position;
        while self.position < self.input.len() && self.current_char() != '"' {
            self.advance();
        }
        let string = self.input[start..self.position].iter().collect::<String>();
        self.advance(); // Skip closing quote
        Token::String(string)
    }
}