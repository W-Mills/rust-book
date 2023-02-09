fn main() {
    println!("Hello, world!");

    let mut s1 = "hello".to_string();
    let s2 = " my friend";
    s1.push_str(s2);
    println!("s2 is: {}", s2);
    println!("s1 is: {}", s1);

    s1.push('!'); // note the single quotation (char)
    println!("s1 is: {}", s1);

    let s3 = String::from(" It is a great day");
    let s4 = s1 + &s3; // ownership of s1 is taken, a copy of s3 is appended to the end, and ownership of the result it returned
    println!("s4 is {}", s4);
    // println!("s1 is: {}", s1); => This would not compile, because ownership of s1 was taken above

    let s5 = "pizza";
    let s6 = "party";
    let s7 = format!("{s5} {s6}"); // format! macro makes easier to read, and uses references
    println!("s7 = {}", s7); // "pizza party"
    println!("s5 = {}", s5); // "pizza" => still compiles b/c of reference
    println!("s4 = {}", s4); // "party"

    for char in "hi".chars() {
        println!("{char}");
    }
    for byte in "hi".bytes() {
        println!("{byte}");
    }


    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    // let team_name = String::from("Blue");
    let score = scores.get(&"Blue".to_string()).copied().unwrap_or(0);

    println!("Blue team score: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!("{:?}", scores);
    println!("{:?}", scores.entry(String::from("Blue")));


}
