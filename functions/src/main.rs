fn main() {
    println!("Hello, world!");

    print_labeled_measurement(10, 'h');

    // let x = five();
    let x = plus_five(5);
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_five(x: i32) -> i32 {
    x + five()
}
