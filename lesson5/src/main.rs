use std::env;
use std::io;

//use crate::my_lib::read_csv;
use crate::my_lib::run;

mod my_lib;

fn main() {
    // Check quantity of args
    // If qty=1 -> catch error -> exit
    // If qty>2 else -> next fn
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    if len == 1 {
        println!("No arguments received from the CLI.\nNo need to enter text.");
        //panic!("===panic===")
        std::process::exit(1);
    } else if len > 2 {
        println!(
            "More than one arguments received from the CLI.\nFirst inputed argument will be used."
        );
    }

    // this modificator will be used modification
    let modification = &args[1];

    // Reading User input into original_text variable
    let mut original_text = String::new();
    println!("Enter the text you want to modify:");
    io::stdin()
        .read_line(&mut original_text)
        .expect("Failed to read line");

    println!("\nTrying to apply '{}'\n", &modification);
    println!("The text after modification:");

    let modified_text = match run(&original_text, &modification) {
        Ok(text) => text,
        Err(_) => "Can't modify".to_string(),
    };
    println!("{}", modified_text);
}
