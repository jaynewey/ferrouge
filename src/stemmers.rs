use rust_stemmers::{Algorithm, Stemmer};

pub fn porter_stem_english<S: AsRef<str>>(word: S) -> String {
    let stemmer = Stemmer::create(Algorithm::English);
    stemmer.stem(word.as_ref()).to_string()
}
