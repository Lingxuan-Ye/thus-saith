/// Why call it Tokenizer? It does not tokenize anything but splits text.
pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(text: &str) -> impl Iterator<Item = &str> {
        text.split("")
    }
}
