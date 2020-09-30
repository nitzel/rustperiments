use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

pub fn main() {
    println!("## ch2.rs - a guessing game ##");
    while let Some(number_of_guesses) = run_game() {
        println!(
            "Congratulations, you required only {} guesses",
            number_of_guesses
        );
    }
}

fn run_game() -> Option<u32> {
    println!();
    println!("#######################");
    println!("## Starting new game ##");
    println!("#######################");
    println!();
    let secret_number = rand::thread_rng().gen_range(0, 101);
    println!("We chose a number between 0 and 100 (inclusive). Find it!");
    let mut number_of_guesses: u32 = 1;
    loop {
        print!("Guess #{} (type 'quit' to exit): ", number_of_guesses);
        io::stdout().flush().ok()?;
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.to_lowercase().trim() == "quit" {
            return None;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("No, '{}' is not a number", guess.trim());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is too small", guess),
            Ordering::Greater => println!("{} is too big", guess),
            Ordering::Equal => {
                println!("You win! {} was correct!", guess);
                return Some(number_of_guesses);
            }
        }

        number_of_guesses += 1;
    }
}
