use std::{
    fs::File, io::{self, Write}, path::PathBuf, str
};

/// Creates a new file by receiving the desired content and the file path.
/// ### Examples
/// ```rust
/// create_file("Hello world", "./server");
/// ```
pub fn create_file(content: &str, path: PathBuf) -> Result<(), io::Error> {
    let mut file: File = File::create(path)?;
    let write_buffer:&[u8] = content.as_bytes();
    file.write_all(write_buffer)?;
    Ok(())
}

/// Finds the patterns on an already readen file and changes it to the replacement.
/// ### Examples
/// ```rust
/// find_placeholder(&template_content, "app_name", app_name);
/// ```
pub fn find_placeholder(text: &str, placeholder: &str, replacement: &str) -> String {
    text.replace(&format!("{{{{ {} }}}}", placeholder), replacement)
}

pub fn find_loop_placeholder(text: &str, loop_target: &str, replacement: Vec<String>) {
    let replaced_text: String = String::new();
    let formatted_text: Vec<String> = text.lines().map(String::from).collect();
    for line in formatted_text {
        if let Some(loop_start_index) = text.find("{{ for ") {

        }
    }
}
