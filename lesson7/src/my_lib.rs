use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use csv::ReaderBuilder;
use slug::slugify;
use std::error::Error;
use std::fs::read_to_string;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

// Enum representing possible commands
#[derive(Debug)]
pub enum Command {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    Csv,
    Exit,
}

// Implement FromStr for Command to allow parsing from strings
impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Command, Self::Err> {
        match s {
            "lowercase" => Ok(Command::Lowercase),
            "uppercase" => Ok(Command::Uppercase),
            "no-spaces" => Ok(Command::NoSpaces),
            "slugify" => Ok(Command::Slugify),
            "exit" => Ok(Command::Exit),
            "csv" => Ok(Command::Csv),
            _ => Err(()),
        }
    }
}

// Function to handle interactive mode
pub fn interactive_mode() {
    // Create a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    // Spawn a thread to read input from stdin
    let input_thread = thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(input) => {
                    // Split input into command and arguments
                    let parts: Vec<&str> = input.splitn(2, ' ').collect();
                    if parts.len() == 2 || parts.len() == 1 {
                        // Parse the command
                        let command = match Command::from_str(parts[0]) {
                            Ok(cmd) => cmd,
                            Err(_) => {
                                eprintln!("Invalid command");
                                continue;
                            }
                        };
                        let input_arg = if parts.len() == 2 {
                            parts[1].to_string()
                        } else {
                            String::new()
                        };
                        // Send command and input to processing thread
                        tx.send((command, input_arg)).unwrap();
                    } else {
                        eprintln!("Invalid input format. Use <command> <input>");
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                }
            }
        }
    });

    // Spawn a thread to process commands
    let processing_thread = thread::spawn(move || {
        while let Ok((command, input)) = rx.recv() {
            // Process the command and print the result
            match command {
                Command::Exit => {
                    println!("Exiting...");
                    process::exit(0); // Exit the entire application
                }
                _ => match process_command(&command, &input) {
                    Ok(output) => println!("{}", output),
                    Err(e) => eprintln!("Error processing command: {}", e),
                },
            }
        }
    });

    // Wait for threads to finish
    input_thread.join().unwrap();
    processing_thread.join().unwrap();
}

// Function to handle oneshot mode
pub fn oneshot_mode(modification: String) {
    let mut original_text = String::new();
    println!("Enter the text you want to modify:");
    io::stdin()
        .read_line(&mut original_text)
        .expect("Failed to read line");

    // Parse the command from the argument
    let command = match Command::from_str(&modification) {
        Ok(cmd) => cmd,
        Err(_) => {
            eprintln!("Invalid command");
            std::process::exit(1);
        }
    };

    // Process the command and print the result
    match process_command(&command, &original_text.trim()) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Error processing command: {}", e),
    }
}

// Function to process commands
fn process_command(command: &Command, input: &str) -> Result<String, Box<dyn Error>> {
    match command {
        Command::Lowercase => to_lowercase(input),
        Command::Uppercase => to_uppercase(input),
        Command::NoSpaces => to_no_spaces(input),
        Command::Slugify => to_slugify(input),
        Command::Csv => read_csv(input),
        Command::Exit => Ok(String::from("Exiting...")),
    }
}

// Function to convert text to lowercase
fn to_lowercase(original_text: &str) -> Result<String, Box<dyn Error>> {
    if original_text.is_empty() {
        Err(From::from("The string is empty"))
    } else {
        Ok(original_text.to_lowercase())
    }
}

// Function to convert text to uppercase
fn to_uppercase(original_text: &str) -> Result<String, Box<dyn Error>> {
    if original_text.is_empty() {
        Err(From::from("The string is empty"))
    } else {
        Ok(original_text.to_uppercase())
    }
}

// Function to remove spaces from text
fn to_no_spaces(original_text: &str) -> Result<String, Box<dyn Error>> {
    if original_text.is_empty() {
        Err(From::from("The string is empty"))
    } else {
        Ok(original_text.replace(" ", ""))
    }
}

// Function to slugify text
fn to_slugify(original_text: &str) -> Result<String, Box<dyn Error>> {
    if original_text.is_empty() {
        Err(From::from("The string is empty"))
    } else {
        Ok(slugify(original_text))
    }
}

// Function to read and display CSV content as a table
fn read_csv<P: AsRef<Path>>(filename: P) -> Result<String, Box<dyn Error>> {
    // Read the CSV file content into a string
    let file_content = read_to_string(filename.as_ref())?;

    // Create a CSV reader from the file content
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) // Adjust this according to your CSV file
        .from_reader(file_content.as_bytes());

    // Create a new table
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    // Add headers to the table if they exist
    if let Some(headers) = rdr.headers().ok() {
        table.set_header(headers.iter().map(|h| h.to_string()));
    }

    // Add CSV records to the table
    for result in rdr.records() {
        match result {
            Ok(record) => {
                table.add_row(record.iter().map(|field| field.to_string()));
            }
            Err(err) => {
                return Err(Box::new(err));
            }
        }
    }

    // Return the table as a formatted string
    Ok(table.to_string())
}
