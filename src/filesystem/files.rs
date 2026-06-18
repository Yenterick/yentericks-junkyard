use std::{
    fs::File,
    io::{self, Read}
};

/// Creates a new file by receiving the desired content and the file path.
/// 
/// ### Examples
/// ```rust
/// create_file("Hello world", "./server");
/// ```
#[allow(dead_code)]
fn create_file(content: &str, path: &str) -> Result<(), io::Error> {
    let mut file: File  = File::create(path)?;
    let mut write_buffer: String = content.to_string();
    file.read_to_string(&mut write_buffer)?;
    Ok(())
}


