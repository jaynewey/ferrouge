use std::collections::HashSet;

pub enum StopWords {
    Rouge155,
}

impl StopWords {
    pub fn get(&self) -> Option<HashSet<&str>> {
        match self {
            StopWords::Rouge155 => {
                let bytes = include_bytes!("stopwords/rouge_155.txt");
                Some(HashSet::from_iter(std::str::from_utf8(bytes).ok()?.lines()))
            }
        }
    }
}
