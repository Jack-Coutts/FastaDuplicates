use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

/// Prompts the user to enter a path to a FASTA file.
///
/// This function repeatedly asks the user for input until a valid FASTA file path is provided.
/// It checks if the file exists and has a .fasta extension (case-insensitive).
///
/// # Returns
///
/// Returns a `Result` containing the path to a valid FASTA file as a `String`,
/// or an `io::Error` if there's an issue with input/output operations.
fn get_file_path() -> io::Result<String> {
    loop {
        print!("Please enter the path to your FASTA file: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        let path = Path::new(input);
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension.to_str().unwrap_or("").to_lowercase() == "fasta" {
                    return Ok(input.to_string());
                }
            }
            println!("The file is not a FASTA file. Please provide a file with .fasta extension.");
        } else {
            println!("The file does not exist. Please try again.");
        }
    }
}

/// Processes a FASTA file and identifies duplicate sequences.
///
/// This function reads the FASTA file, extracts sequences and their IDs,
/// and groups them by sequence to identify duplicates.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the FASTA file
///
/// # Returns
///
/// Returns a `Result` containing a `HashMap` where keys are sequences and values are vectors of IDs,
/// or an `io::Error` if there's an issue with file operations.
fn process_fasta_file(file_path: &str) -> io::Result<HashMap<String, Vec<String>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut sequences: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_id = String::new();
    let mut current_sequence = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            // If we've been building a sequence, store it before moving to the next
            if !current_id.is_empty() {
                sequences.entry(current_sequence.clone())
                    .or_insert_with(Vec::new)
                    .push(current_id.clone());
                current_sequence.clear();
            }
            current_id = line.trim().to_string();
        } else {
            current_sequence.push_str(line.trim());
        }
    }

    // Don't forget to add the last sequence
    if !current_id.is_empty() {
        sequences.entry(current_sequence.clone())
            .or_insert_with(Vec::new)
            .push(current_id);
    }

    Ok(sequences)
}

/// Writes duplicate sequences to a file.
///
/// This function takes a HashMap of sequences and their IDs, identifies duplicates,
/// and writes them to a new file in the same directory as the input file.
///
/// # Arguments
///
/// * `sequence_hashmap` - A HashMap where keys are sequences and values are vectors of IDs
/// * `input_filename` - A string slice that holds the path to the input FASTA file
///
/// # Returns
///
/// Returns a `Result` indicating success or an `io::Error` if there's an issue with file operations.
fn return_duplicates(sequence_hashmap: HashMap<String, Vec<String>>, input_filename: &str) -> io::Result<()> {
    // Count the number of duplicate sequences
    let duplicate_count = sequence_hashmap.iter().filter(|(_, ids)| ids.len() > 1).count();

    // Create the output filename in the same directory as the input file
    let input_path = Path::new(input_filename);
    let parent_dir = input_path.parent().unwrap_or(Path::new(""));
    let stem = input_path.file_stem().unwrap_or_default().to_str().unwrap_or("");
    let output_filename = parent_dir.join(format!("{}_duplicates.txt", stem));

    // Open the output file
    let mut output_file = File::create(&output_filename)?;

    // Write the duplicate count
    writeln!(output_file, "Found {} duplicate sequences:", duplicate_count)?;
    writeln!(output_file)?;

    // Write each duplicate sequence
    for (sequence, ids) in sequence_hashmap {
        if ids.len() > 1 {
            writeln!(output_file, "Duplicate sequence found:")?;
            for id in ids {
                writeln!(output_file, "  ID: {}", id)?;
            }
            writeln!(output_file, "  Sequence: {}", sequence)?;
            writeln!(output_file)?;
        }
    }

    println!("Duplicate sequences written to {}", output_filename.display());
    Ok(())
}

fn main() -> io::Result<()> {
    let file_path = get_file_path()?;
    println!("You selected the FASTA file: {}", file_path);
    let sequences = process_fasta_file(&file_path)?;
    return_duplicates(sequences, &file_path)?;
    Ok(())
}