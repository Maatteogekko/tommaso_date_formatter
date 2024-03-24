use std::io::{stdin, stdout, Write};

use chrono::NaiveDate;
use tommaso_date_formatter::format;

fn main() {
    loop {
        print!("Insert date (YYYY-MM-DD): ");
        stdout().flush().unwrap();
        let mut input_date = String::new();
        stdin().read_line(&mut input_date).unwrap_or_else(|err| {
            eprintln!("Error reading input: {}", err);
            std::process::exit(1);
        });
        let input_date = {
            match NaiveDate::parse_from_str(input_date.trim(), "%Y-%m-%d") {
                Ok(v) => v,
                Err(e) => {
                    println!("Error: {}", e);
                    stdout().flush().unwrap();
                    continue;
                }
            }
        };

        print!("Insert format: ");
        stdout().flush().unwrap();
        let mut input_format = String::new();
        stdin().read_line(&mut input_format).unwrap_or_else(|err| {
            eprintln!("Error reading input: {}", err);
            std::process::exit(1);
        });

        let result = format(&input_date, input_format.trim());
        match result {
            Ok(v) => println!("Formatted date: {}", v),
            Err(e) => println!("Error: {}", e),
        }
        stdout().flush().unwrap();
    }
}
