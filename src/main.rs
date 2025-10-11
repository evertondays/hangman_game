use std::fs;
use rand::prelude::*;

fn main() {
    let (words, saved_words_quantity) = read_file();
    // let word  = choose_word(words, saved_words_quantity);

    match choose_word(words, saved_words_quantity) {
        Ok(word) => print!("{}", word),
        Err(err) => eprintln!("Erro ao ler arquivo: {}", err),
    }
}

fn read_file() -> (String, usize) {
    let content = fs::read_to_string("./assets/words_pt.txt")
        .expect("Não foi possível ler o arquivo!");

    let lines_count = content.lines().count();
    return (content, lines_count);
}

fn choose_word(words: String, saved_words_quantity: usize) -> Result<String, String> {
    let randon_number = rand::rng().random_range(0..saved_words_quantity);
    
    print!("{}\n", randon_number);

    let mut counter = 0;
    let mut word = String::from("");
    for c in words.chars() {
        let is_break_line = is_break_line(c);

        if is_break_line && counter == randon_number {
            return Ok(word);
        }

        if is_break_line {
            counter = counter + 1;
        }

        if counter == randon_number {
            word.push(c);
        }
    }

    Err("Palavra escolhida não encontrada!".to_string())
}

fn is_break_line(c: char) -> bool {
    return c == 0xA as char;
}
