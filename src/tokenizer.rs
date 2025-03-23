use std::fmt;

/// How can anyone call this a tokenizer? It doesn't tokenize anything!
pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(text: &str) -> impl Iterator<Item = Token> {
        text.trim_end().chars().map(Token)
    }
}

pub struct Token(char);

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
