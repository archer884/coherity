use itertools::Itertools;
use punkt::{params::Standard, SentenceTokenizer, Trainer, TrainingData};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

fn get_all_words(s: &str) -> Vec<String> {
    s.unicode_words().map(|s| s.to_owned()).collect()
}

pub fn long_words(word_list: &[String]) -> f64 {
    word_list.iter().filter(|&s| s.len() >= 6).count() as f64
}

pub fn percent_long_words(word_list: Vec<String>) -> f64 {
    let list_count = word_list.len() as i32;
    let long_words_count = long_words(&word_list);
    long_words_count / list_count as f64
}

pub fn character_count(string_to_analyze: &str) -> f64 {
    let re = Regex::new(r"[^\w]").unwrap();
    let result = re.replace_all(string_to_analyze, "");
    result.graphemes(true).count() as f64
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

pub fn get_average_syllable_count(sylcount_list: Vec<i32>) -> f64 {
    let mut sum = 0;
    for w in &sylcount_list {
        sum += w;
    }
    sum as f64 / sylcount_list.len() as f64
}

// num of words with 2 syllables or less;
pub fn short_syl_count(word_list: Vec<String>) -> i32 {
    let sylcount_list = get_syllable_counts(&word_list);
    let mut short_syls = 0;
    for syl in sylcount_list {
        if syl <= 2 {
            short_syls += 1;
        }
    }
    short_syls
}

// num of words with 3 syls or more;
pub fn long_syl_count(word_list: Vec<String>) -> i32 {
    let sylcount_list = get_syllable_counts(&word_list);
    let mut long_syls = 0;
    for syl in sylcount_list {
        if syl >= 3 {
            long_syls += 1;
        }
    }
    long_syls
}

pub fn wordcount_list(word_list: Vec<String>) -> i32 {
    word_list.len() as i32
}

pub fn split_into_sentences(doc: &str) -> Vec<String> {
    let trainer: Trainer<Standard> = Trainer::new();
    let mut data = TrainingData::new();

    trainer.train(doc, &mut data);
    let mut sent_list: Vec<String> = Vec::new();

    for s in SentenceTokenizer::<Standard>::new(doc, &data) {
        sent_list.push(s.to_string());
    }
    sent_list
}

pub fn get_sentence_lengths(doc: &str) -> Vec<usize> {
    let trainer: Trainer<Standard> = Trainer::new();
    let mut data = TrainingData::new();

    trainer.train(doc, &mut data);

    let mut sent_word_list: Vec<String> = Vec::new();
    let mut sent_wordcount_list: Vec<usize> = Vec::new();

    for s in SentenceTokenizer::<Standard>::new(doc, &data) {
        sent_word_list.push(s.to_string());
    }
    for s in sent_word_list {
        let mut temp_vec: Vec<String> = Vec::new();
        for word in s.split_whitespace() {
            temp_vec.push(word.to_string());
        }
        sent_wordcount_list.push(temp_vec.len());
    }
    sent_wordcount_list
}

pub fn sentence_average_word_count(s: &[usize]) -> f64 {
    let sum: usize = s.iter().sum();
    sum as f64 / s.len() as f64
}

pub fn flesch_reading_ease(string_to_analyze: &str) -> f64 {
    let (average_sentence_length, average_word_syllables) = fk_values(string_to_analyze);
    206.835 - (1.015 * average_sentence_length) - (84.6 * average_word_syllables)
}

pub fn flesch_kincaid_grade_level(string_to_analyze: &str) -> f64 {
    let (average_sentence_length, average_word_syllables) = fk_values(string_to_analyze);
    (0.39 * average_sentence_length) + (11.8 * average_word_syllables) - 15.59
}

fn fk_values(string_to_analyze: &str) -> (f64, f64) {
    let sentence_lengths = get_sentence_lengths(string_to_analyze);
    let average_sentence_length = sentence_average_word_count(&sentence_lengths);

    // avg syls per word
    let words = get_all_words(string_to_analyze);
    let word_syllables = get_syllable_counts(&words);
    let average_word_syllables = get_average_syllable_count(word_syllables);

    (average_sentence_length, average_word_syllables)
}

pub fn lix(string_to_analyze: &str) -> f64 {
    //total_words
    let all_words = get_all_words(string_to_analyze);
    //num long words
    let num_long_words = percent_long_words(all_words);
    //avg_num_words/sentence
    let sent_wordcount_list = get_sentence_lengths(string_to_analyze);
    let avg_words_per_sentence = sentence_average_word_count(&sent_wordcount_list);
    num_long_words + avg_words_per_sentence
}

pub fn rix(string_to_analyze: &str) -> f64 {
    let all_words = get_all_words(string_to_analyze);
    let long_words_count = long_words(&all_words);
    let sentence_count = split_into_sentences(string_to_analyze).len() as f64;
    long_words_count / sentence_count
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

// FIXME Avoid allocation of normalized string.
fn get_syllable_count(word: &str) -> u32 {
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
