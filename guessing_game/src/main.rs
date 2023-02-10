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

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("------! That's not a number !------");
                continue
            },
        };


        if guess < 1 || guess > 100 {
            println!("------! Secret number will be between 1 and 100 !------");
            continue;
        }

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

// This struct would allow robust error handling at the source, so that all following code is guaranteed to
// be a valid guess, rather than needing to handle possible invalidity (Ch 9.3)

// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, but got {}.", value);
//         }

//         Guess { value }
//     }

//     pub fn value(&self) -> i32 {
//         self.value
//     }
// }
