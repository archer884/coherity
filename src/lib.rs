mod characterize;

use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

pub use characterize::{Characterization, Characterizer};

fn characters(s: &str) -> usize {
    s.graphemes(true)
        .filter(|&s| s.chars().all(|u| u.is_alphabetic()))
        .count()
}

pub fn linsear_scoring(s: &[i32]) -> f64 {
    s.iter()
        .map(|&len| if len < 3 { 1.0f64 } else { 3.0 })
        .sum()
}

pub fn get_syllable_counts(s: &[String]) -> Vec<i32> {
    s.iter()
        .map(|word| get_syllable_count(word) as i32)
        .collect()
}

pub fn get_average_syllable_count(s: &[i32]) -> f64 {
    let sum: i32 = s.iter().sum();
    sum as f64 / s.len() as f64
}

pub fn wordcount_list(word_list: Vec<String>) -> i32 {
    word_list.len() as i32
}

pub fn sentence_average_word_count(s: &[usize]) -> f64 {
    let sum: usize = s.iter().sum();
    sum as f64 / s.len() as f64
}

pub fn linsear_write(string_to_analyze: &str) -> f64 {
    //compiling variables for linsear_write
    let all_words = get_all_words(string_to_analyze);
    let all_syls = get_syllable_counts(&all_words);
    let summed_score = linsear_scoring(&all_syls);
    let sent_count = split_into_sentences(string_to_analyze).len() as f64;

    // compute the score
    let provisional_result = summed_score / sent_count;

    // provisional adjustment
    if provisional_result < 20.0 {
        (provisional_result / 2.0) - 1.0
    } else {
        provisional_result / 2.0
    }
}

pub fn coleman_liau(string_to_analyze: &str) -> f64 {
    //total_words
    let all_words = get_all_words(string_to_analyze).len() as f64;
    // characters
    let chars = character_count(string_to_analyze);
    // sents
    let sent_count = split_into_sentences(string_to_analyze).len() as f64;

    0.0588 * ((chars / all_words) * 100.0) - 0.296 * ((sent_count / chars) * 100.0) - 15.8
}

pub fn automated_readability_index(string_to_analyze: &str) -> f64 {
    //total_words
    let all_words = get_all_words(string_to_analyze).len() as f64;
    // characters
    let chars = character_count(string_to_analyze);
    // sents
    let sent_count = split_into_sentences(string_to_analyze).len() as f64;

    (4.71 * (chars / all_words)) + (0.5 * (all_words / sent_count)) - 21.43
}

trait Average {
    type Output;
    fn average(self) -> Self::Output;
}

impl Average for &[f64] {
    type Output = f64;

    #[inline]
    fn average(self) -> Self::Output {
        let sum: f64 = self.iter().sum();
        sum / self.len() as f64
    }
}

impl Average for &[usize] {
    type Output = f64;

    #[inline]
    fn average(self) -> Self::Output {
        let sum: usize = self.iter().sum();
        sum as f64 / self.len() as f64
    }
}

impl Average for &[u8] {
    type Output = f64;

    #[inline]
    fn average(self) -> Self::Output {
        let sum: u8 = self.iter().sum();
        sum as f64 / self.len() as f64
    }
}

// FIXME Avoid allocation of normalized string.
fn get_syllable_count(word: &str) -> u8 {
    // Single syllables in words like bread and lead, but split in names like Breanne and Adreann
    // TODO: handle names, where we only use "ia" (again, original author's todo)
    static SPECIALS: &[(u8, u8)] = &[(b'i', b'a'), (b'e', b'a')];

    // Seperate syllables unless ending the word
    static SPECIALS_EXCEPT_END: &[(u8, u8)] =
        &[(b'i', b'e'), (b'y', b'a'), (b'e', b's'), (b'e', b'd')];

    fn is_vowel(u: u8) -> bool {
        static VOWELS: &[u8] = b"aeiouy";
        VOWELS.contains(&u)
    }

    let mut count = 0;

    let normalized = word.to_lowercase();
    let normalized_windows = std::iter::once(b' ').chain(normalized.bytes());

    for (previous, current) in normalized_windows.tuple_windows() {
        if !is_vowel(current) {
            continue;
        }

        let pair = (previous, current);
        if !is_vowel(previous) || SPECIALS.contains(&pair) || SPECIALS_EXCEPT_END.contains(&pair) {
            count += 1;
        }
    }

    if word.len() > 2 {
        let suffix = &word[word.len() - 2..];
        let tail = suffix.as_bytes()[1];

        if SPECIALS_EXCEPT_END.contains(&(suffix.as_bytes()[0], suffix.as_bytes()[1]))
            || suffix != "ee" && tail == b'e' && normalized != "the"
        {
            count -= 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use unicode_segmentation::UnicodeSegmentation;

    use crate::Characterizer;

    #[test]
    fn sentence_average_word_count() {
        let input = vec![5, 7, 4, 12];
        let expected = 7.0;
        let actual = super::sentence_average_word_count(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn linsear_scoring() {
        let input = vec![4, 3, 2, 1, 7, 1, 2, 4];
        let expected = 16.0;
        let actual = super::linsear_scoring(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_syllable_counts() {
        let text: Vec<_> = "In the beginning, God created the heaven and the earth."
            .unicode_words()
            .map(|s| s.into())
            .collect();
        let expected = [1, 1, 3, 1, 2, 1, 3, 1, 1, 2];
        let actual = super::get_syllable_counts(&text);
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_average_syllable_count() {
        let expected = 6.142857142857143;
        let actual = super::get_average_syllable_count(&[3, 4, 5, 6, 7, 8, 10]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_sentence_lengths() {
        let text = "I just thought it was strange. I've asked to be assigned to \
            your security detail--I just wanted you to know that. This coat was tailor-made for \
            me by my tailor, Chadwick, in Beverly Hills. Check it out: it's $17.50 a couple.";

        let characterizer = Characterizer::new();
        let expected = &[6, 16, 14, 7];
        let actual = characterizer.characterize(text).sentence_lengths;
        assert_eq!(actual, expected);
    }

    #[test]
    fn correct_syllable_count() {
        // TODO: Create a hashmap
        // Note: this is the original author's TODO item and I don't give a shit about it.
        assert_eq!(1, super::get_syllable_count("the"));
        assert_eq!(3, super::get_syllable_count("lucozade"));
        assert_eq!(1, super::get_syllable_count("love"));
        assert_eq!(2, super::get_syllable_count("dodo"));
        assert_eq!(1, super::get_syllable_count("world"));
        assert_eq!(2, super::get_syllable_count("atom"));
        assert_eq!(3, super::get_syllable_count("energy"));
        assert_eq!(4, super::get_syllable_count("combination"));
    }
}
