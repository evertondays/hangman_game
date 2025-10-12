mod visual_effects;
mod word;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write, stdout};

const ACCEPTED_LETTERS: [char; 26] = [
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'
];

fn main() {
    let word = word::get_word();

    let mut all_guess: [char; 26] = ['_'; 26];
    let mut input = String::new();
    let mut errors: i8 = 0;

    loop {
        clear_screen();
        visual_effects::print_strength(errors);

        let guess = guess_letter(&mut input, &mut all_guess);
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

fn get_player_input(input: &mut String) -> char {
    loop {
        input.clear();
        print!("Digite uma letra: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(input).expect("\nErro ao ler entrada");

        let input = input.trim().to_lowercase();

        if input.len() != 1 {
            println!("\nÉ necessário digirar um caracter!");
        }

        if let Some(letter) = input.chars().last() {
            for c in ACCEPTED_LETTERS {
                if c == letter {
                    return letter
                }
            }

            println!("Caracter inválido!")
        }
    }
}

fn guess_letter(input: &mut String, all_guess: &mut [char; 26]) -> char {
    loop {
        let guess = get_player_input(input);

        for i in 0..all_guess.len() {
            if guess == all_guess[i] {
                println!("\nVocê já tentou essa letra!");
                break;
            }

            if all_guess[i] == '_' as char {
                all_guess[i] = guess;
                return guess;
            }
        }
    }
}

fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}
