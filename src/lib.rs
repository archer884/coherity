mod characterize;

use itertools::Itertools;

pub use characterize::{Characterization, Characterizer};

pub fn wordcount_list(word_list: Vec<String>) -> i32 {
    word_list.len() as i32
}

pub fn sentence_average_word_count(s: &[usize]) -> f64 {
    let sum: usize = s.iter().sum();
    sum as f64 / s.len() as f64
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
    use crate::Characterizer;

    #[test]
    fn sentence_average_word_count() {
        let input = vec![5, 7, 4, 12];
        let expected = 7.0;
        let actual = super::sentence_average_word_count(&input);
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
