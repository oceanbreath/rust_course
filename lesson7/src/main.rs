use crate::my_lib::{interactive_mode, oneshot_mode};
use std::env;

mod my_lib;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    if len == 1 {
        // No arguments provided, enter interactive mode
        interactive_mode();
    } else if len == 2 {
        // One argument provided, read input from stdin and process the command
        let modification = &args[1];
        oneshot_mode(modification.to_string());
    } else {
        // More than one argument provided, use only the first argument
        println!(
            "More than one argument received from the CLI. Only the first inputted argument will be used."
        );
        let modification = &args[1];
        oneshot_mode(modification.to_string());
    }
}
