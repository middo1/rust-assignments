fn main() {
    /*-----------------------------------------------
     * This function :
     * - prints the Full Name
     * - prints the Matric  number
     * - prints the Favourite Emoji
     * - prints the result of the division of 
     * two floats
     * - also calls other functions
     ------------------------------------------------*/
    println!("Hamid Balogun Adeshina"); // Full Name
    println!("PSC2307139"); // Matric Number
    println!("{}", '\u{1F494}'); // Favourite Emoji
    let num_1 : f32 = 22.0;
    let num_2 : f32 = 7.0;
    println!("{}", num_1/num_2); // Divide two numbers to give a float
    println!("{}",name("Hamid Balogun Adeshina")); // Full Name function
    println!("{}",mat_num("PSC2307139")); // Matric Number function
    println!("{}",fav_emoji('\u{1F494}')); // Favourite Emoji function
    println!("{}",divide_two_floats(22.0, 7.0)); // Divide two numbers function
}

fn name(name: &str) -> &str{
    /*
     * This function returns inputted Name
     */
    name
}

fn mat_num(mat_num: &str) -> &str{
    /*
     * This function prints my Matric Number
     */
    mat_num
}

fn fav_emoji(emoji: char) -> char{
    /*
     * This function returns the inputted Emoji
     */
    emoji
}

fn divide_two_floats(num_1: f32, num_2: f32) -> f32{
    /*
     * This function:
     * - divides two floats
     * - and then returns the results
     */
    num_1 / num_2
}