fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree.",
        "two Turtle Doves,",
        "three French Hens,",
        "four Calling Birds,",
        "five Golden Rings,",
        "six Geese a-Laying,",
        "seven Swans a-Swimming,",
        "eight Maids a-Milking,",
        "nine Ladies Dancing,",
        "ten Lords a-Leaping,",
        "eleven Pipers Piping,",
        "twelve Drummers Drumming,",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas my true love gave to me:",
            days[day]
        );

        // Print gifts in reverse order up to the current day
        for gift in (0..=day).rev() {
            if gift == 0 && day != 0 {
                print!("and ");
            }
            println!("{}", gifts[gift]);
        }
        println!(); // Add a blank line between verses
    }
}
