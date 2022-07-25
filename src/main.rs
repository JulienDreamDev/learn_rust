use std::io::{self, Write};

fn hello_world() {
    println!("Hello, world!"); // "!" denote a macro
}

fn guessing_game() {
    println!("Guess the number!");

    print!("Input your guess: ");
    io::stdout() // Flushing to ensure the output is showed before stdin
        .flush()
        .expect("Failed to flush stdout"); // Handle errors

    let mut guess = String::new(); // "mut" denote that it's not a constant
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess is {guess}");
}

fn main() {
    // hello_world();
    guessing_game();
}
