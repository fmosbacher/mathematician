use std::slice::Iter;

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Times,
    Div,
}

#[derive(Debug)]
pub enum Token {
    Integer(u32),
    Operator(Operator),
}

#[derive(Debug, Default)]
pub struct Lexer {
    remaining: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn tokens(&self) -> Iter<Token> {
        self.tokens.iter()
    }

    fn skip_whitespace(&mut self) {
        self.remaining = self.remaining.trim().to_string();
    }

    fn tokenize_operator(&mut self) -> Result<(), String> {
        self.skip_whitespace();
        let mut chars = self.remaining.chars();
        if let Some(op) = chars.next() {
            match op {
                '+' => self.tokens.push(Token::Operator(Operator::Add)),
                '-' => self.tokens.push(Token::Operator(Operator::Sub)),
                '*' => self.tokens.push(Token::Operator(Operator::Times)),
                '/' => self.tokens.push(Token::Operator(Operator::Div)),
                _ => return Err("Cannot find operator symbol".to_string()),
            }
            self.remaining = self.remaining[1..].to_string();
            Ok(())
        } else {
            Err("Found empty input".to_string())
        }
    }

    fn tokenize_integer(&mut self) -> Result<(), String> {
        self.skip_whitespace();
        let mut chars = self.remaining.chars();
        let mut acc = String::new();
        let digits = '0'..='9';
        while let Some(int) = chars.next() {
            if digits.contains(&int) {
                acc.push(int);
            } else {
                break;
            }
        }
        if let Ok(int) = acc.parse::<u32>() {
            self.tokens.push(Token::Integer(int));
            self.remaining = self.remaining[acc.len()..].to_string();
            Ok(())
        } else {
            Err("Cannot find digits".to_string())
        }
    }

    pub fn tokenize(&mut self, input: &str) -> Result<(), String> {
        self.remaining = input.to_string();
        loop {
            let result = self
                .tokenize_operator()
                .or_else(|_| self.tokenize_integer());
            if result.is_err() || self.remaining.len() == 0 {
                break result;
            }
        }
    }
}

// expression -> NUMBER | unary | binary | grouping;
// unary -> "-" expression;
// binary -> expression operator expression;
// grouping -> "(" expression ")";
// operator -> "+" | "-" | "*" | "/" | "^";
