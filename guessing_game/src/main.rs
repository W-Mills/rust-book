use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nGuess the secret number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("\nPlease input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number!\n");
                continue
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("\n*** Spot on!! ***\n Winner Gagnon!!\n");
                break;
            }
        }
    }
}