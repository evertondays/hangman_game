use std::collections::HashMap;

mod visual_effects;
mod word;
mod player_input;

const LETTERS_IN_ALPHABET: usize = 26;

fn main() {
    let normalization_map = word::initialize_normalization_hashmap();

    let word = word::get_word();

    let mut all_guess: [char; LETTERS_IN_ALPHABET] = ['_'; LETTERS_IN_ALPHABET];
    let mut input = String::new();
    let mut errors: i8 = 0;

    loop {
        visual_effects::clear_screen();
        visual_effects::print_strength(errors);

        let guess = player_input::guess_letter(&mut input, &mut all_guess, &normalization_map);
        if !check_is_correct(&word, guess, &normalization_map) {
            errors = errors + 1;
        }
    }
}

fn check_is_correct(word: &String, guess: char, normalization_map: &HashMap<char, char>) -> bool {
    for c in word.chars() {
        if word::normalize_char(c, &normalization_map) == guess {
            return true;
        }
    }

    return false;
}
