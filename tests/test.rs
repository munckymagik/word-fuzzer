extern crate word_fuzzer;

use word_fuzzer::fuzz;

#[test]
fn it_preserves_the_empty_string() {
    assert_eq!("", fuzz(""));
}

#[test]
fn it_preserves_one_letter_words() {
    assert_eq!("I", fuzz("I"));
}

#[test]
fn it_preserves_two_letter_words() {
    assert_eq!("it", fuzz("it"));
}

#[test]
fn it_preserves_three_letter_words() {
    assert_eq!("cat", fuzz("cat"));
}

#[test]
fn it_switches_the_middle_two_letters_of_four_letter_words() {
    assert_eq!("giat", fuzz("gait"));
}

#[test]
fn it_shuffles_the_middle_letters_of_five_or_more_letter_words() {
    let words = [
        "catch",
        "object",
        "suddenly",
        "appearing",
    ];

    for word in words.iter() {
        let fuzzed = fuzz(word);
        assert_fuzzed(word, &fuzzed);
    }
}

#[test]
fn it_copes_with_multi_byte_utf8() {
    let word = "忠犬ハチ公";
    let fuzzed = fuzz(word);
    assert_fuzzed(word, &fuzzed);
}

#[test]
fn it_never_returns_an_identical_string() {
    // A five letter word with 3 middle letters has a 1 in 6 chance of
    // being randomly shuffled and coming back identical. Therefore that
    // permutation must be thrown away.
    let word = "catch";
    let iterations = 100;
    let mut count_non_fuzzed = 0;

    for _ in 0..iterations {
        let fuzzed = fuzz(word);
        if word == &fuzzed { count_non_fuzzed += 1; }
    }

    let percentage = count_non_fuzzed as f64 / iterations as f64 * 100.;
    assert_eq!(percentage, 0.);
}

fn assert_fuzzed(word: &str, fuzzed: &str) {
    assert_ne!(fuzzed, word);

    let word_chars = word.chars().collect::<Vec<char>>();
    let fuzz_chars = fuzzed.chars().collect::<Vec<char>>();
    assert_eq!(word_chars.first(), fuzz_chars.first());
    assert_eq!(word_chars.last(), fuzz_chars.last());
}
