use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use slug::slugify;
use std::env::args;
use std::error::Error;

pub fn run(original_text: &str, modification: &str) -> Result<String, Box<dyn Error>> {
    let result = match modification {
        "lowercase" => to_lowercase(original_text),
        "uppercase" => to_uppercase(original_text),
        "no-spaces" => to_no_spaces(original_text),
        "slugify" => to_slugify(original_text),
        "csv" => read_csv(original_text),
        _ => unreachable!("The entered argument is not in the list of available options"),
    };

    match result {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

fn to_lowercase(original_text: &str) -> Result<String, Box<dyn Error>> {
    if original_text.is_empty() {
        Err(From::from("The string is empty"))
    } else {
        Ok(original_text.to_lowercase())
    }
}

fn to_uppercase(original_text: &str) -> Result<String, Box<dyn Error>> {
    if original_text.is_empty() {
        Err(From::from("The string is empty"))
    } else {
        Ok(original_text.to_uppercase())
    }
}

fn to_no_spaces(original_text: &str) -> Result<String, Box<dyn Error>> {
    if original_text.is_empty() {
        Err(From::from("The string is empty"))
    } else {
        Ok(original_text.replace(" ", ""))
    }
}

fn to_slugify(original_text: &str) -> Result<String, Box<dyn Error>> {
    if original_text.is_empty() {
        Err(From::from("The string is empty"))
    } else {
        Ok(slugify(original_text))
    }
}

fn read_csv(original_text: &str) -> Result<(), Box<dyn Error>> {
    if original_text.trim().is_empty() {
        Err(From::from("Input text is empty".to_string()))
    } else {
        let mut reader = csv::Reader::from_reader(original_text.as_bytes());
        let header = comfy_table::Row::from(reader.headers()?.clone().iter());

        let mut table = comfy_table::Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(header);

        for record in reader.records() {
            let record = match record {
                Err(e) => {
                    return Err(From::from(
                        "Failed to read file".to_string() + &e.to_string(),
                    ));
                }

                Ok(record) => record,
            };

            let row = comfy_table::Row::from(record.iter());
            table.add_row(row);
        }
        Ok(table.to_string())
    }
}
