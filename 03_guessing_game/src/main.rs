// Import the standard input output library to accept user input
use std::io;

use rand::Rng;

// Main function
fn main() {
    println!("Guess the number!\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your number: ");

    
    // Create a mutable variable with type String to store the user input
    // due to : String the variable can only hold a String
    // String::new() creates a new empty string as placeholder
    let mut guess = String::new();
    
    // Receive User input
    io::stdin()
        // Calls read_line method on the standard input handle to receive the user input
        // the created variable is passed as input to tell where to store the data
        // needs to be passed with &mut to make it mutable
        .read_line(&mut guess)
        // Handles any errors returned by the read_line method
        .expect("Failed to read line");
    // Could all have been written as io::stdin().read_line(&mut guess).expect("Failed to read line");
    // but that is hard to read, it is wise to split long lines into multiple ones

    // Print the returned value
    println!("You guessed: {guess}");

}
