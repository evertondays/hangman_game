mod word;
use std::io::{self, Write};

fn main() {
    let word = word::get_word();
    let mut input = String::new();

    loop {
        guess_letter(&mut input);
    }
}

fn get_guess(input: &mut String) -> char {
    loop {
        input.clear();
        print!("Digite uma letra: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(input)
            .expect("Erro ao ler entrada");

        let input = input.trim();

        if let Some(letter) = input.chars().next() {
            return letter;
        }

        println!("Nenhum caractere digitado! Tente novamente.");
    }
}

fn guess_letter(input: &mut String) {
    let mut all_guess: [char; 26] = ['_'; 26];
    let guess = get_guess(input);

    for i in 0..all_guess.len() {
        if all_guess[i] == '_' {
            all_guess[i] = guess;
            break;
        }

        if guess == all_guess[i] {
            println!("Você já tentou essa lentra!\n");
        }
    }
}