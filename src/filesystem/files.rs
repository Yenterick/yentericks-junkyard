use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
    str,
};

/// Creates a new file by receiving the desired content and the file path.
/// ### Examples
/// ```rust
/// create_file("Hello world", "./server");
/// ```
pub fn create_file(content: &str, path: PathBuf) -> Result<(), io::Error> {
    let mut file: File = File::create(path)?;
    let write_buffer: &[u8] = content.as_bytes();
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

/// Finds the patterns for a loop on an already readen file and changes it to the replacement.
/// ### Examples
/// ```rust
/// find_loop_placeholder(&template_content, "routers", routers) /* routers: Vec<String> */
/// ```
pub fn find_loop_placeholder(text: &str, loop_target: &str, replacement: Vec<&str>) -> String {
    let mut result: String = text.to_string();

    while let Some(for_start) = result.find("{{ for ") {
        let relative_header_end: usize = result[for_start..].find("}}").unwrap();
        let header_end: usize = for_start + relative_header_end;
        let header: &str = &result[for_start..header_end];

        let content = header.strip_prefix("{{ for ").unwrap();
        let mut parts: str::SplitWhitespace<'_> = content.split_whitespace();

        let variable = parts.next().unwrap();
        parts.next().unwrap(); // skips "in"
        let collection = parts.next().unwrap();

        if collection != loop_target {
            break;
        }

        let endfor = format!("{{{{ endfor {} }}}}", collection);

        let relative_loop_end: usize = result[header_end..].find(&endfor).unwrap();
        let loop_end = header_end + relative_loop_end;

        let block = result[header_end + 2..loop_end].trim();
        let mut expanded: String = String::new();

        for (index, item) in replacement.iter().enumerate() {
            expanded.push_str(&find_placeholder(block, variable, item));

            if index < replacement.len() - 1 {
                expanded.push('\n');
            }
        }

        let full_block_end: usize = loop_end + endfor.len();

        result.replace_range(for_start..full_block_end, &expanded);
    }

    result
}
