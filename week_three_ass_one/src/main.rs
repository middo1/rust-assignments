fn main() {
    /*
     * Print the lyrics of "The 12 days of Christmas"
     * taking advantage of the repetition in the song
     */
    let mut day = 1;
    while day < 13 {
        println!("On the {} day of Christmas", process_date(day));
        println!("My true love sent to me");
        process_gifts(day);
        day += 1;
        println!("\n\n");
    }
}

fn process_date(day: u32) -> String {
    let new_day = match day {
        1 => "first".to_string(),
        2 => "second".to_string(),
        3 => "third".to_string(),
        4 => "fourth".to_string(),
        5 => "fifth".to_string(),
        6 => "sixth".to_string(),
        7 => "seventh".to_string(),
        8 => "eight".to_string(),
        9 => "ninth".to_string(),
        10 => "tenth".to_string(),
        11 => "eleventh".to_string(),
        12 => "twelvth".to_string(),
        _ => String::new(),
    };
    return new_day;
}

fn process_gifts(day: u32) {
    let gifts: [&str; 12] = [
        "12 drummers drumming",
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
        " partridge in a pear tree",
    ];
    let x: usize = gifts.len() - day as usize;
    if x == 11 {
        println!("A{}", gifts[x]);
    } else {
        for i in x..gifts.len() {
            if i == 11 {
                println!("And a{}", gifts[i])
            } else {
                println!("{}", gifts[i])
            }
        }
    }
    if x == 0 {
        println!("And a partridge in a pear tree");
    }
}
