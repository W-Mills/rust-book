use std::io;

fn main() {
    loop {
        let mut fahrenheit = String::new();

        println!("\nPlease input a fahrenheit temperature");

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: f32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a real temperature!\n");
                continue
            },
        };

        let celcius: f32 = {
            5.0 / 9.0 * (fahrenheit - 32.0)
        };

        println!("{fahrenheit}F is {celcius:.1}C");
        break;
    };
}
