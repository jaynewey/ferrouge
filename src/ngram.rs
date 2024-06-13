use std::cmp::{max, min};
use std::collections::HashMap;

use crate::score::Score;

type NGram<'a, const N: usize> = &'a [String; N];
type NGramCounts<'a, const N: usize> = HashMap<NGram<'a, N>, u32>;

pub fn count_ngrams<const N: usize>(tokens: &[String]) -> NGramCounts<'_, N> {
    let mut ngram_counts: HashMap<&[String; N], u32> = HashMap::new();
    for window in tokens.windows(N).map(|i| i.try_into().unwrap()) {
        if let Some(count) = ngram_counts.get_mut(window) {
            *count += 1
        } else {
            ngram_counts.insert(window, 1);
        }
    }

    ngram_counts
}

pub fn score_ngrams<const N: usize>(
    target_ngrams: NGramCounts<'_, N>,
    prediction_ngrams: NGramCounts<'_, N>,
) -> Score {
    let intersection_count = target_ngrams
        .iter()
        .map(|(key, value)| min(value, prediction_ngrams.get(key).unwrap_or(&0)))
        .sum::<u32>() as f32;

    let target_count = target_ngrams.values().sum();
    let prediction_count = prediction_ngrams.values().sum();

    let precision = intersection_count / max(prediction_count, 1) as f32;
    let recall = intersection_count / max(target_count, 1) as f32;

    Score::new(precision, recall)
}

#[cfg(test)]
mod tests {
    use crate::{tokenizers::{WhitespaceTokenizer, Tokenizer}, ngram::count_ngrams};

    #[test]
    fn test_count_ngrams() {
        let tokenizer = WhitespaceTokenizer;
        let tokens: Vec<String> = tokenizer.tokenize("Hello, world!").collect();
        let unigram_counts = count_ngrams::<1>(&tokens);
        assert_eq!(unigram_counts.get(&["hello".to_string()]), Some(&1));
        assert_eq!(unigram_counts.get(&["world".to_string()]), Some(&1));
        let bigram_counts = count_ngrams::<2>(&tokens);
        assert_eq!(
            bigram_counts.get(&["hello".to_string(), "world".to_string()]),
            Some(&1)
        );
    }
}
