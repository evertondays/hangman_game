use std::collections::HashMap;

use crate::visual_effects::VisualWord;

mod player_input;
mod visual_effects;
mod word;

const LETTERS_IN_ALPHABET: usize = 26;

fn main() {
    let mut input = String::new();
    let normalization_map = word::initialize_normalization_hashmap();

    let mut errors: i8 = 0;
    let mut all_corrects = 0;
    let word = word::get_word();
    let mut visual_word = visual_effects::get_visual_word(&word);
    let mut all_guess: [char; LETTERS_IN_ALPHABET] = ['_'; LETTERS_IN_ALPHABET];

    loop {
        visual_effects::clear_screen();
        visual_effects::print_strength(errors);
        visual_effects::print_word(&visual_word); // TODO melhorar nome dessa função
        visual_effects::print_used_words(&all_guess);

        let guess = player_input::guess_letter(&mut input, &mut all_guess, &normalization_map);
        let corrects = check_corrects(&mut visual_word, guess, &normalization_map);

        // TODO encapsular isso no end_game
        if corrects == 0 {
            errors = errors + 1;
        } else {
            all_corrects = all_corrects + corrects;
        }

        if errors == 6 {
            visual_effects::clear_screen();
            print!("Você perdeu! :(\nA palavra era: {}\n", word);
            break;
        }
    }
}

fn check_corrects(
    word: &mut VisualWord,
    guess: char,
    normalization_map: &HashMap<char, char>,
) -> i32 {
    let mut corrects = 0;

    for i in 0..word.size {
        if word::normalize_char(word.letters[i].c, &normalization_map) == guess {
            word.letters[i].visible = true;
            corrects = corrects + 1;
        }
    }

    return corrects;
}
