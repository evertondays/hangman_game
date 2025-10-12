mod visual_effects;
mod word;
mod player_input;

fn main() {
    let word = word::get_word();

    let mut all_guess: [char; 26] = ['_'; 26];
    let mut input = String::new();
    let mut errors: i8 = 0;

    loop {
        visual_effects::clear_screen();
        visual_effects::print_strength(errors);

        let guess = player_input::guess_letter(&mut input, &mut all_guess);
        if !check_is_correct(&word, guess) {
            errors = errors + 1;
        }
    }
}

fn check_is_correct(word: &String, guess: char) -> bool {
    for c in word.chars() {
        if c == guess {
            return true;
        }
    }

    return false;
}
