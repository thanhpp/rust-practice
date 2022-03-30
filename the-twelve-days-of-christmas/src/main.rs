fn main() {
    let lyrics = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtle-doves",
        "And a partridge in a pear tree",
    ];
    let days = [
        "first", "second", "third", "forth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for i in 0..12 {
        println!("On the {} of Christmas", days[i]);
        println!("My true love sent to me");
        for j in 0..i + 1 {
            println!("{}", lyrics[11 - j])
        }

        println!()
    }
}
