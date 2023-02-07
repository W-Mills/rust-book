fn main() {
    let mut verse_index: usize = 0;

    while verse_index <= 11 {
        first_lines(verse_index);
        things_sent(verse_index);
        println!(""); // janky verse separator

        verse_index += 1;
    }

    fn first_lines(verse_index: usize) {
        let english_number = [
            "first",
            "second",
            "third",
            "fourth",
            "fifth",
            "sixth",
            "seventh",
            "eighth",
            "ninth",
            "tenth",
            "eleventh",
            "twelfth",
        ];

        let day_num = english_number[verse_index];
        println!("On the {} day of Christmas", day_num);
        println!("My true love sent to me");
    }

    fn things_sent(verse_index: usize) {
        let things_to_send = [
            "A partridge in a pear tree",
            "Two turtle-doves",
            "Three French hens",
            "Four calling birds",
            "Five golden rings (five golden rings)",
            "Six geese a laying",
            "Seven swans a swimming",
            "Eight cows milking",
            "Nine people dancing",
            "Ten lords a-leaping",
            "Eleven pipers piping",
            "Twelve drummers drumming",
        ];

        // +1 to adjust for zero-based indexing
        for gift_index in (0..verse_index + 1).rev() {
            if gift_index == 0 && verse_index != 0 {
                println!("And a partridge in a pear tree"); // *And* a ...
            } else {
                println!("{}", things_to_send[gift_index]);
            }
        }
    }
}
