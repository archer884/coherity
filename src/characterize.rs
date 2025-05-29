use punktoo::{params::Standard, SentenceTokenizer, TrainingData};
use unicode_segmentation::UnicodeSegmentation;

use crate::{get_syllable_count, Average};

/// Used to characterize documents for reading analysis.
///
/// Caches tokenizer training data.
#[derive(Debug)]
pub struct Characterizer {
    training: TrainingData,
}

impl Characterizer {
    pub fn new() -> Self {
        Self {
            training: TrainingData::english(),
        }
    }

    /// Characterizes a document.
    pub fn characterize<'a>(&self, document: &'a str) -> Characterization<'a> {
        let sentences = self.sentences(document);
        let words: Vec<_> = document.unicode_words().collect();

        Characterization {
            sentence_lengths: sentences
                .iter()
                .map(|&s| s.unicode_words().count())
                .collect(),
            sentences,
            word_syllable_lengths: words.iter().map(|&word| get_syllable_count(word)).collect(),
            words,
        }
    }

    /// Splits a document into sentences.
    fn sentences<'a>(&self, doc: &'a str) -> Vec<&'a str> {
        SentenceTokenizer::<Standard>::new(doc, &self.training).collect()
    }
}

impl Default for Characterizer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Characterization<'a> {
    pub sentences: Vec<&'a str>,
    pub sentence_lengths: Vec<usize>,
    pub words: Vec<&'a str>,
    pub word_syllable_lengths: Vec<u8>,
}

impl Characterization<'_> {
    pub fn fk_grade_level(&self) -> f64 {
        let (average_sentence_length, average_word_syllables) = self.fk_values();
        (0.39 * average_sentence_length) + (11.8 * average_word_syllables) - 15.59
    }

    pub fn reading_ease(&self) -> f64 {
        let (average_sentence_length, average_word_syllables) = self.fk_values();
        206.835 - (1.015 * average_sentence_length) - (84.6 * average_word_syllables)
    }

    #[inline]
    fn fk_values(&self) -> (f64, f64) {
        (
            self.sentence_lengths.average(),
            self.word_syllable_lengths.average(),
        )
    }
}
