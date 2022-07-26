use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut counter: u32 = 0;

    loop {
        print!("Input your guess: ");
        io::stdout() // Flushing to ensure the output is showed before stdin
            .flush()
            .expect("Failed to flush stdout"); // Handle errors

        let mut guess = String::new(); // "mut" denote that it's not a constant
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess: u32 = match guess.trim() // Shadowing the previous devlaration to change the type
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number!");
                    continue; // Next loop
                },
            };

        counter += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed right in {counter} tries!");
                break; // Break the loop
            },
        }
    }
}

fn main() {
    guessing_game();
}
