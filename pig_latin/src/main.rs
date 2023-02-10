use std::io;
use std::collections::HashMap;

fn main() {

    let consonants = HashMap::from([
        ('b', true),
        ('c', true),
        ('d', true),
        ('f', true),
        ('g', true),
        ('j', true),
        ('k', true),
        ('l', true),
        ('m', true),
        ('n', true),
        ('p', true),
        ('q', true),
        ('s', true),
        ('t', true),
        ('v', true),
        ('x', true),
        ('z', true),
        ('h', true),
        ('r', true),
        ('w', true),
        ('y', true),
    ]);

    loop {
        println!("\nPlease input a sentence to pig-latinify:");

        let mut sentence = String::new();

        io::stdin()
            .read_line(&mut sentence)
            .expect("Failed to read line");

        let mut sentence: String = match sentence.trim().parse() {
            Ok(sentence) => sentence,
            Err(_) => {
                println!("That's not a sentence!\n");
                continue
            },
        };

        sentence = sentence.to_lowercase();
        let mut pig_latin = String::from("");

        for word in sentence.split_whitespace() {
            let first_char = word.chars().next().unwrap();
            let is_consonant = consonants.get(&first_char).copied().unwrap_or(false);
            let mut piggied = String::from(" ");

            if is_consonant == true {
                for (i, char) in word.chars().enumerate() {
                    if i == 0 {
                        continue
                    } else {
                        piggied.push_str(&char.to_string());
                    }
                }
                let suffix = format!("-{first_char}ay");
                piggied.push_str(&suffix.to_string());
            } else { // is vowel
                piggied = format!(" {word}-hay");
            }
            pig_latin.push_str(&piggied);
        }

        println!("{}", pig_latin.trim());

        break;
    }
}

// input string
// slice input string into words, iterate over the chars for each word
//   if char is a consonant => slice off, append to end of string + "ay"
//   else => no slice, but append to end of string "hay"

// ex
// first => irst-fay
// first time at pig latin => irst-fay ime-tay at-hay ig-pay atin-lay
