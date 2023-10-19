fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let lyrics = [
        "my true love sent to me",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
        "a partridge in a pear tree!",
    ];
    let mut para = 1;
    for count in days {
        println!("On the {count} day of Christmas,");
        para += 1;
        for a in 0..para {
            if a == para - 1 && para == 2 {
                println!("{}", lyrics[12])
            } else if a == para - 1 {
                println!("and {}", lyrics[12])
            } else {
                println!("{}", lyrics[a])
            }
        }
        println!("")
    }
}
