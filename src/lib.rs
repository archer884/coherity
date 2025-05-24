use punkt::{params::Standard, SentenceTokenizer, Trainer, TrainingData};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use wordsworth::syllable_counter;

fn word_list_from_string(s: &str) -> Vec<String> {
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

pub fn linsear_scoring(sylcount_list: Vec<i32>) -> f64 {
    let mut score_sum: f64;
    score_sum = 0.0;
    for syl in &sylcount_list {
        if (*syl as f64) < 2.5 {
            score_sum += 1.0;
        } else {
            score_sum += 3.0;
        }
    }
    score_sum
}

pub fn syllable_count_list(word_list: Vec<String>) -> Vec<i32> {
    let mut sylcount_list: Vec<i32> = Vec::new();
    for word in &word_list {
        sylcount_list.push(syllable_counter(word).try_into().unwrap());
    }
    sylcount_list
}

pub fn avg_syl_count(sylcount_list: Vec<i32>) -> f64 {
    let mut sum = 0;
    for w in &sylcount_list {
        sum += w;
    }
    sum as f64 / sylcount_list.len() as f64
}

// num of words with 2 syllables or less;
pub fn short_syl_count(word_list: Vec<String>) -> i32 {
    let sylcount_list = syllable_count_list(word_list);
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
    let sylcount_list = syllable_count_list(word_list);
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

pub fn sent_word_counts(doc: &str) -> Vec<usize> {
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

pub fn sentence_average_word_count(sent_wordcount_list: Vec<usize>) -> f64 {
    let mut sum = 0;
    for w in &sent_wordcount_list {
        sum += w;
    }
    sum as f64 / sent_wordcount_list.len() as f64
}

pub fn flesch_reading_ease(string_to_analyze: &str) -> f64 {
    let sent_counts = sent_word_counts(string_to_analyze);
    let avg_sent_length = sentence_average_word_count(sent_counts);
    //syls
    let all_words = word_list_from_string(string_to_analyze);
    let all_syls = syllable_count_list(all_words);
    let avg_syls = avg_syl_count(all_syls);
    206.835 - (1.015 * avg_sent_length) - (84.6 * avg_syls)
}

pub fn flesch_kincaid_grade_level(string_to_analyze: &str) -> f64 {
    // avg_words_per_sentence
    let sent_counts = sent_word_counts(string_to_analyze);
    let avg_sent_length = sentence_average_word_count(sent_counts);
    // avg syls per word
    let all_words = word_list_from_string(string_to_analyze);
    let all_syls = syllable_count_list(all_words);
    let avg_syls = avg_syl_count(all_syls);
    (0.39 * avg_sent_length) + (11.8 * avg_syls) - 15.59
}

pub fn lix(string_to_analyze: &str) -> f64 {
    //total_words
    let all_words = word_list_from_string(string_to_analyze);
    //num long words
    let num_long_words = percent_long_words(all_words);
    //avg_num_words/sentence
    let sent_wordcount_list = sent_word_counts(string_to_analyze);
    let avg_words_per_sentence = sentence_average_word_count(sent_wordcount_list);
    num_long_words + avg_words_per_sentence
}

pub fn rix(string_to_analyze: &str) -> f64 {
    let all_words = word_list_from_string(string_to_analyze);
    let long_words_count = long_words(&all_words);
    let sentence_count = split_into_sentences(string_to_analyze).len() as f64;
    long_words_count / sentence_count
}

pub fn linsear_write(string_to_analyze: &str) -> f64 {
    //compiling variables for linsear_write
    let all_words = word_list_from_string(string_to_analyze);
    let all_syls = syllable_count_list(all_words);
    let summed_score = linsear_scoring(all_syls);
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
    let all_words = word_list_from_string(string_to_analyze).len() as f64;
    // characters
    let chars = character_count(string_to_analyze);
    // sents
    let sent_count = split_into_sentences(string_to_analyze).len() as f64;

    0.0588 * ((chars / all_words) * 100.0) - 0.296 * ((sent_count / chars) * 100.0) - 15.8
}

pub fn automated_readability_index(string_to_analyze: &str) -> f64 {
    //total_words
    let all_words = word_list_from_string(string_to_analyze).len() as f64;
    // characters
    let chars = character_count(string_to_analyze);
    // sents
    let sent_count = split_into_sentences(string_to_analyze).len() as f64;

    (4.71 * (chars / all_words)) + (0.5 * (all_words / sent_count)) - 21.43
}
