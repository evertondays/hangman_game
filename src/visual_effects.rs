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
