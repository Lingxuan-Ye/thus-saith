/// Why name it Tokenizer? It does't tokenize anything but splits text!
pub(crate) struct Tokenizer;

impl Tokenizer {
    pub(crate) fn tokenize(text: &str) -> impl Iterator<Item = &str> {
        text.split("")
    }
}
