use slug::slugify;
use std::env;
use std::io;

fn main() {
    // Reading CLI arguments and collecting them into a vector.
    // Calling std::process::exit(),
    // in the case of no arguments entered
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments received from the CLI.\nNo need to enter text.");
        std::process::exit(1);
    }

    // Reading User input into original_text variable
    let mut original_text = String::new();
    println!("Enter the text you want to modify:");
    io::stdin().read_line(&mut original_text).expect("Failed to read line");

    // Аpplying the method to the text according to the received argument
    println!("\nTrying to apply '{}'\n", args[1]);
    // there are two options:
    // - changing the original_text variable, as it is mutable
    // - using the modified_text variable, 
    //   if it is necessary to keep the original_text variable unchanged
    //original_text = match args[1].as_str() {
    let modified_text = match args[1].as_str() {
        "lowercase" => original_text.to_lowercase(),
        "uppercase" => original_text.to_uppercase(),
        "no-spaces" => original_text.replace(" ", ""),
        "slugify" => slugify(original_text),
        _ => String::from("The entered argument is not in the list of available options"),
    };
    println!("The text after modification:");
    println!("{}", modified_text);
    //println!("=>\t{}", original_text);
}
