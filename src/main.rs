use std::io::{self, Write};
use std::path::Path;

fn get_file_path() -> String {
    loop {
        print!("Please enter the path to your FASTA file: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let path = Path::new(input);

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension.to_str().unwrap_or("").to_lowercase() == "fasta" {
                    return input.to_string();
                } else {
                    println!("The file is not a FASTA file. Please provide a file with .fasta extension.");
                }
            } else {
                println!("The file has no extension. Please provide a file with .fasta extension.");
            }
        } else {
            println!("The file does not exist. Please try again.");
        }
    }
}

fn main() {
    let file_path = get_file_path();
    println!("You selected the file: {}", file_path);
    // Add your file processing logic here
}

