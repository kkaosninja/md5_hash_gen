use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

const INPUT_FILENAME: &str = "input.txt";
const OUTPUT_FILENAME: &str = "output.txt";

fn main() {
    println!("--- BEGIN PROGRAM ---");

    //Open Input File containing strings that need to be hashed
    let input_file_handle = File::open(INPUT_FILENAME).expect("Failed to open input file");
    let input_file_lines = BufReader::new(input_file_handle).lines();

    // Open output file where the hashes of the strings in the input file need to be written
    // Set Truncate=True to clear previous instance of file. This requires Write=True
    let output_file_handle = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(OUTPUT_FILENAME)
        .expect("Failed to open output file");
    let mut output_file_writer = BufWriter::new(output_file_handle);

    // For each input line, get the hash and write it to the output file
    for input_file_line in input_file_lines {
        if let Ok(unhashed_string) = input_file_line {
            println!("Currently hashing {unhashed_string}");

            let input_line_hash = get_md5_hash(unhashed_string);

            output_file_writer
                .write_all(input_line_hash.as_bytes())
                .expect("Failed to write hash to output file");
            output_file_writer
                .write_all("\n".as_bytes())
                .expect("Failed to write newline to output file!");
        }
    }

    // Flush Output file writer buffer
    output_file_writer
        .flush()
        .expect("Could not flush output writer!");

    println!("--- END PROGRAM ---");
}

/// . Return the MD5 Hash of the input string
fn get_md5_hash(input_str: String) -> String {
    let digest = md5::compute(input_str);

    return format!("{:x}", digest);
}

#[test]
fn test_get_md5_hash() {
    let test_string = String::from("Hello World!");

    const CORRECT_ANSWER: &str = "ed076287532e86365e841e92bfc50d8c";

    assert_eq!(get_md5_hash(test_string), CORRECT_ANSWER);
}
