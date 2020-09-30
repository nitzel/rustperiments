use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub fn main() {
    println!("## ch2.rs - a guessing game ##");
    loop {
        let number_of_guesses = run_game().expect("Something went wrong");
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

    let mut number_of_guesses: u32 = 1;
    loop {
        print!("Guess #{}: ", number_of_guesses);
        io::stdout().flush().ok()?;
        let mut raw_guess = String::new();
        io::stdin()
            .read_line(&mut raw_guess)
            .expect("Failed to read line");
        let guess: Result<u32, ParseIntError> = raw_guess.trim().parse();
        let guess = match guess {
            Ok(x) => x,
            Err(_) => {
                println!("No, '{}' is not a number", raw_guess.trim());
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
