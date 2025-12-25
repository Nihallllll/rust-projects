use std::collections::HashMap;

struct TextAnalyzer {
    text: String,
}

impl TextAnalyzer {
    fn new(text: String) -> Self {
        Self { text }
    }

    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    fn sentence_count(&self) -> usize {
        self.text.matches('.').count()
    }

    fn longest_word(&self) -> Option<&str> {
        self.text.split_whitespace().max_by_key(|w| w.len())
    }

    fn shortest_word(&self) -> Option<&str> {
        self.text.split_whitespace().min_by_key(|w| w.len())
    }

    fn word_frequency(&self) -> HashMap<&str, usize> {
        let mut freq = HashMap::new();
        for word in self.text.split_whitespace() {
            *freq.entry(word).or_insert(0) += 1;
        }
        freq
    }
}
