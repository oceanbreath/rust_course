use std::io;

fn main() {
    // Inviting the user to enter his/her name
    println!("Please enter your name:");

    // Mutable String variable intended to store the user input
    let mut name = String::new();

    // Reading the user input
    // and storing it to 'name' variable (&mut ?????)
    // by 'std::io' 'read_line'
    // in case of exception it throws the message 'Failed...'
    io::stdin().read_line(&mut name).expect("Failed to read line");

    // Trim whitespace from the input
    let name = name.trim();

    // Print a greeting message with the user's name
    println!("Hello, {}!\nAnd welcome to the first lesson.", name);
}
