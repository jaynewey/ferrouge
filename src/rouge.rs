use std::collections::HashSet;

use crate::ngram::{count_ngrams, score_ngrams};
use crate::stemmers::porter_stem_english;
use crate::stopwords;
use crate::tokenizers::{Tokenizer, WhitespaceTokenizer};
use crate::Score;

pub struct Rouge<'a> {
    tokenizer: &'a dyn Tokenizer,
    stemmer: Option<fn(String) -> String>,
    stopwords: Option<HashSet<&'a str>>,
}

impl<'a> Default for Rouge<'a> {
    fn default() -> Self {
        Rouge {
            tokenizer: &WhitespaceTokenizer,
            stemmer: Some(porter_stem_english),
            stopwords: stopwords::StopWords::Rouge155.get(),
        }
    }
}

impl<'a> Rouge<'a> {
    fn tokenize(&self, text: impl AsRef<str> + 'a) -> Vec<String> {
        let tokens = self.tokenizer.tokenize(text.as_ref());
        let tokens = tokens.filter(|token| {
            self.stopwords
                .as_ref()
                .map_or(true, |stopwords| !stopwords.contains(token.as_str()))
        });
        let tokens = tokens.map(|token| {
            if let Some(stemmer) = self.stemmer {
                stemmer(token)
            } else {
                token
            }
        });
        tokens.collect()
    }

    pub fn score<const N: usize>(
        &self,
        target: impl AsRef<str> + 'a,
        prediction: impl AsRef<str> + 'a,
    ) -> Score {
        let target_tokens = self.tokenize(target);
        let target_ngrams = count_ngrams::<N>(&target_tokens);

        let prediction_tokens = self.tokenize(prediction);
        let prediction_ngrams = count_ngrams::<N>(&prediction_tokens);
        score_ngrams(target_ngrams, prediction_ngrams)
    }
}

#[cfg(test)]
mod tests {
    use crate::Rouge;

    #[test]
    fn test_rouge_n() {
        let rouge = Rouge::default();
        let mut score = rouge.score::<1>(
            "the cat was under the bed",
            "the cat was found under the bed",
        );
        println!("{:?}", score);

        score = rouge.score::<2>(
            "the cat was under the bed",
            "the cat was found under the bed",
        );
        println!("{:?}", score);
    }
}
