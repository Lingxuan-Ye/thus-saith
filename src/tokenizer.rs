/// Why name it Tokenizer? It does't tokenize anything but splits text!
pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(text: &str) -> impl Iterator<Item = &str> {
        text.split("")
    }
}
