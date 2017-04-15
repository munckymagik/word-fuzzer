extern crate rand;

use rand::Rng;

pub fn fuzz(word: &str) -> String {
    let mut out = String::with_capacity(word.len());
    if word.len() == 0 { return out; }

    let chars = word.chars().collect::<Vec<char>>();

    if let Some((first, rest)) = chars.split_first() {
        out.push(*first);

        if let Some((last, middle)) = rest.split_last() {
            if middle.len() == 1 {
                // 3 letter words
                out.extend(middle);
            } else if middle.len() == 2 {
                // 4 letter words
                out.extend(middle.iter().rev());
            } else if middle.len() >= 3 {
                // 5 or more letter words
                let mut shuffled = middle.to_owned();
                rand::thread_rng().shuffle(shuffled.as_mut_slice());

                // Deal with 1 in 6 chance of getting the original permutation from shuffle when
                // there are 3 letters in the middle. Cost is a 1 in 3 chance of getting the
                // reverse permutation.
                if shuffled == middle { shuffled.reverse(); }

                out.extend(shuffled);
            }

            // for 2 or more letters: always append the final character
            out.push(*last);
        }
    }

    out
}
