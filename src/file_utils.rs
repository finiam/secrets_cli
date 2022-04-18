use std::{fs::File, io::Read};

pub fn read_file_to_string(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Open the file in read-only mode.
    let mut file = File::open(filename)?;
    // The file is open (no error).
    let mut content = String::new();

    // Read all the file content into a variable (ignoring the result of the operation).
    file.read_to_string(&mut content)?;

    Ok(content)
}
