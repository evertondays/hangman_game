use rand::prelude::*;
use std::fs;
use std::process;

pub fn get_word() -> String {
    let (words, saved_words_quantity) = read_file();

    match choose_word(words, saved_words_quantity) {
        Ok(choosed_word) => {
            print!("{}\n", choosed_word); // TODO remover essa linha
            return choosed_word;
        }
        Err(err) => {
            eprintln!("Erro ao ler arquivo: {}", err);
            process::exit(1);
        }
    }
}

fn read_file() -> (String, usize) {
    let content =
        fs::read_to_string("./assets/words_pt.txt").expect("Não foi possível ler o arquivo!");

    let lines_count = content.lines().count();
    return (content, lines_count);
}

fn choose_word(words: String, saved_words_quantity: usize) -> Result<String, String> {
    let randon_number = rand::rng().random_range(0..saved_words_quantity);

    let mut word_counter = 0;
    let mut word = String::from("");
    for c in words.chars() {
        let is_break_line = is_break_line(c);
        let is_choose_word = word_counter == randon_number;

        if is_break_line && is_choose_word {
            return Ok(word);
        }

        if is_break_line {
            word_counter = word_counter + 1;
        }

        if is_choose_word {
            word.push(c);
        }
    }

    Err("Palavra escolhida não encontrada!".to_string())
}

fn is_break_line(c: char) -> bool {
    return c == 0xA as char;
}
