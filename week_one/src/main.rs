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
    let fav_emoji: char = '🦀'; // Favourite Emoji
    println!("{}", fav_emoji);
    let num_1 : f32 = 22.0;
    let num_2 : f32 = 7.0;
    println!("{}", num_1/num_2); // Divide two numbers to give a float
    print_name(); // Full Name function
    print_mat_num(); // Matric Number function
    print_fav_emoji(); // Favourite Emoji function
    divide_two_floats(); // Divide two numbers function
}

fn print_name() {
    /*
     * This function prints my Full Name
     */
    println!("Hamid Balogun Adeshina");
}

fn print_mat_num() {
    /*
     * This function prints my Matric Number
     */
    println!("PSC2307139");
}

fn print_fav_emoji() {
    /*
     * This function prints my Favourite Emoji
     */
    let fav_emoji: char = '🦀';
    println!("{}", fav_emoji);
}

fn divide_two_floats() {
    /*
     * This function:
     * - divides two floats
     * - and then prints the results
     */
    let num_1 : f32 = 22.0;
    let num_2 : f32 = 7.0;
    println!("{}", num_1/num_2); 
}