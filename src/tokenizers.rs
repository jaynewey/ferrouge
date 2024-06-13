pub trait Tokenizer {
    fn tokenize<'a>(&self, text: &'a str) -> Box<dyn Iterator<Item = String> + 'a>;
}

pub struct WhitespaceTokenizer;

impl Tokenizer for WhitespaceTokenizer {
    fn tokenize<'a>(&self, text: &'a str) -> Box<dyn Iterator<Item = String> + 'a> {
        Box::new(text.split_whitespace().map(|token| {
            token
                .replace(|c: char| !c.is_alphanumeric(), "")
                .to_lowercase()
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizers::{WhitespaceTokenizer, Tokenizer};

    #[test]
    fn test_whitespace_tokenizer() {
        let tokenizer = WhitespaceTokenizer;
        let tokens: Vec<String> = tokenizer.tokenize("Hello, world!").collect();
        assert_eq!(tokens, vec!["hello", "world"]);
    }
}
