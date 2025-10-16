use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

use std::io::{stdout};

#[derive(Debug)]
struct Letter {
    c: char,
    visible: bool,
}

pub struct VisualWord {
    size: usize,
    letters: [Letter; 35],
}

pub fn get_visual_word(word: &String) -> VisualWord {
    let word_chars: Vec<char> = word.chars().collect();

    let letters = std::array::from_fn(|i| {
        if let Some(c) = word_chars.get(i) {
            Letter {
                c: *c,
                visible: false,
            }
        } else {
            Letter {
                c: '\0',
                visible: false,
            }
        }
    });

    let size = word.chars().count().min(35);

    return VisualWord { size, letters };
}

pub fn print_word(word: &VisualWord) {
    for i in 0..word.size {
        if word.letters[i].visible {
            print!("{}", word.letters[i].c);
        } else {
            print!("_");
        }
    }

    print!("\n");
}

pub fn print_strength(value: i8) {
    match value {
        0 => print_0(),
        1 => print_1(),
        2 => print_2(),
        3 => print_3(),
        4 => print_4(),
        5 => print_5(),
        6 => print_6(),
        _ => print!("erro!"),
    }
}

pub fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

fn print_0() {
    print!("+--+\n");
    print!("|  |\n");
    print!("   |\n");
    print!("   |\n");
    print!("   |\n");
    print!("   |\n");
    print!("=====\n");
}

fn print_1() {
    print!("+--+\n");
    print!("|  |\n");
    print!("O  |\n");
    print!("   |\n");
    print!("   |\n");
    print!("   |\n");
    print!("=====\n");
}

fn print_2() {
    print!(" +--+\n");
    print!(" |  |\n");
    print!(" O  |\n");
    print!(" |  |\n");
    print!("    |\n");
    print!("    |\n");
    print!("=====\n");
}

fn print_3() {
    print!(" +--+\n");
    print!(" |  |\n");
    print!(" O  |\n");
    print!("/|  |\n");
    print!("    |\n");
    print!("    |\n");
    print!("=====\n");
}

fn print_4() {
    print!(" +--+\n");
    print!(" |  |\n");
    print!(" O  |\n");
    print!("/|\\ |\n");
    print!("    |\n");
    print!("    |\n");
    print!("=====\n");
}

fn print_5() {
    print!(" +--+\n");
    print!(" |  |\n");
    print!(" O  |\n");
    print!("/|\\ |\n");
    print!("/   |\n");
    print!("    |\n");
    print!("=====\n");
}

fn print_6() {
    print!(" +--+\n");
    print!(" |  |\n");
    print!(" O  |\n");
    print!("/|\\ |\n");
    print!("/ \\ |\n");
    print!("    |\n");
    print!("=====\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_visual_word_with_simple_example() {
        // Arrange
        let word = String::from("rust");

        // Act
        let result = get_visual_word(&word);

        // Assert
        assert_eq!(result.size, 4);
        assert_eq!(result.letters[0].c, 'r');
        assert_eq!(result.letters[0].visible, false);
        assert_eq!(result.letters[3].c, 't');
        assert_eq!(result.letters[4].c, '\0'); 
    }

    #[test]
    fn test_get_visual_word_with_long_example() {
        // Arrange
        let word = String::from("paralelepípedo");

        // Act
        let result = get_visual_word(&word);

        // Assert
        assert_eq!(result.size, 14);
        assert_eq!(result.letters[9].c, 'í');
        assert_eq!(result.letters[0].visible, false);
        assert_eq!(result.letters[2].c, 'r');
        assert_eq!(result.letters[15].c, '\0'); 
    }

    #[test]
    fn test_get_visual_word_with_fake_example() {
        // Arrange
        let word = String::from("ãuç");

        // Act
        let result = get_visual_word(&word);

        // Assert
        assert_eq!(result.size, 3);
        assert_eq!(result.letters[0].c, 'ã');
        assert_eq!(result.letters[0].visible, false);
        assert_eq!(result.letters[1].c, 'u');
        assert_eq!(result.letters[1].visible, false);
        assert_eq!(result.letters[2].c, 'ç');
        assert_eq!(result.letters[2].visible, false);
    }
}