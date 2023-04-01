use std::slice::Iter;

#[derive(Debug)]
pub enum Token {
    Integer(i64),
    Plus,
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
            if op == '+' {
                self.tokens.push(Token::Plus);
                self.remaining = self.remaining[1..].to_string();
                Ok(())
            } else {
                Err("Cannot find plus symbol".to_string())
            }
        } else {
            Err("Found empty input".to_string())
        }
    }

    fn tokenize_positive_integer(&mut self) -> Result<(), String> {
        self.skip_whitespace();
        let mut chars = self.remaining.chars();
        let mut acc = String::new();
        let digits = '0'..='9';
        while let Some(possible_digit) = chars.next() {
            if digits.contains(&possible_digit) {
                acc.push(possible_digit);
            } else {
                break;
            }
        }
        if let Ok(num) = acc.parse::<i64>() {
            self.tokens.push(Token::Integer(num));
            self.remaining = self.remaining[acc.len()..].to_string();
            Ok(())
        } else {
            Err("Cannot find digits".to_string())
        }
    }

    fn tokenize_negative_integer(&mut self) -> Result<(), String> {
        self.skip_whitespace();
        let mut chars = self.remaining.chars();
        let mut acc = String::new();
        let digits = '0'..='9';
        if let Some('-') = chars.next() {
            acc.push('-');
            while let Some(possible_digit) = chars.next() {
                if digits.contains(&possible_digit) {
                    acc.push(possible_digit);
                } else {
                    break;
                }
            }
            if let Ok(num) = acc.parse::<i64>() {
                self.tokens.push(Token::Integer(num));
                self.remaining = self.remaining[acc.len()..].to_string();
                Ok(())
            } else {
                Err("Cannot find digits after minus symbol".to_string())
            }
        } else {
            Err("Cannot find minus symbol".to_string())
        }
    }

    fn tokenize_integer(&mut self) -> Result<(), String> {
        self.tokenize_negative_integer()
            .or(self.tokenize_positive_integer())
    }

    pub fn tokenize(&mut self, input: &str) -> Result<(), String> {
        self.remaining = input.to_string();
        self.tokenize_integer()
            .and(self.tokenize_operator())
            .and(self.tokenize_integer())
    }
}
