use rand::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::process;

pub fn get_word() -> String {
    let (words, saved_words_quantity) = read_file();

    match choose_word(words, saved_words_quantity) {
        Ok(chosen_word) => {
            print!("{}\n", chosen_word); // TODO remover essa linha
            return chosen_word;
        }
        Err(err) => {
            eprintln!("Erro ao ler arquivo: {}", err);
            process::exit(1);
        }
    }
}

pub fn initialize_normalization_hashmap() -> HashMap<char, char> {
    return HashMap::from([
        ('á', 'a'),
        ('â', 'a'),
        ('à', 'a'),
        ('ã', 'a'),
        ('é', 'e'),
        ('ê', 'e'),
        ('í', 'i'),
        ('ó', 'o'),
        ('ô', 'o'),
        ('õ', 'o'),
        ('ú', 'u'),
        ('ç', 'c'),
    ]);
}

pub fn normalize_char(c: char, normalization_hashmap: &HashMap<char, char>) -> char {
    match normalization_hashmap.get(&c) {
        Some(&normalized) => normalized,
        None => c,  
    }
}

fn read_file() -> (String, usize) {
    let content =
        fs::read_to_string("./assets/words_pt.txt").expect("Não foi possível ler o arquivo!");

    let lines_count = content.lines().count();
    return (content, lines_count);
}

fn choose_word(words: String, saved_words_quantity: usize) -> Result<String, String> {
    let random_number = rand::rng().random_range(0..saved_words_quantity);

    let mut word_counter = 0;
    let mut word = String::from("");
    for c in words.chars() {
        let is_break_line = is_break_line(c);
        let is_choose_word = word_counter == random_number;

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
