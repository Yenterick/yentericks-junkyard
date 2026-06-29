use std::{
    fs::{File, read_to_string},
    io::{self, Write},
    path::{Path, PathBuf},
    str,
};

// Custom imports
use crate::models::model::{Field, Relationship};

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

///
pub fn read_template(path: &Path) -> Result<String, io::Error> {
    Ok(read_to_string(path)?.replace("\r\n", "\n"))
}

/// Finds the patterns on an already readen file and changes it to the replacement.
/// ### Examples
/// ```rust
/// find_placeholder(&template_content, "app_name", app_name);
/// ```
pub fn find_placeholder(text: &str, placeholder: &str, replacement: &str) -> String {
    text.replace(&format!("{{{{ {} }}}}", placeholder), replacement)
}

/// Prepares the loop placeholder output so it doesn't come out with a lot of line jumps
/// ### Examples
/// ```rust
/// normalize_loop_output(rendered);
/// ```
fn normalize_loop_output(rendered: String) -> String {
    rendered
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Finds the patterns for a loop on an already readen file and changes it to the replacement.
/// ### Examples
/// ```rust
/// find_loop_placeholder(&template_content, "routers", routers) /* routers: Vec<String> */
/// ```
pub fn find_loop_placeholder(text: &str, loop_target: &str, replacement: Vec<&str>) -> String {
    let mut result: String = text.to_string();
    let mut search_start: usize = 0;

    while let Some(relative_for_start) = result[search_start..].find("{{ for ") {
        let for_start = search_start + relative_for_start;
        let relative_header_end: usize = result[for_start..].find("}}").unwrap();
        let header_end: usize = for_start + relative_header_end;
        let header: &str = &result[for_start..header_end];

        let content = header.strip_prefix("{{ for ").unwrap();
        let mut parts: str::SplitWhitespace<'_> = content.split_whitespace();

        let variable = parts.next().unwrap();
        parts.next().unwrap(); // skips "in"
        let collection = parts.next().unwrap();

        let endfor = format!("{{{{ endfor {} }}}}", collection);

        let Some(relative_loop_end) = result[header_end..].find(&endfor) else {
            return result;
        };
        let loop_end = header_end + relative_loop_end;
        let full_block_end: usize = loop_end + endfor.len();

        if collection != loop_target {
            search_start = full_block_end;
            continue;
        }

        let block: &str = &result[header_end + 2..loop_end].to_string();
        let mut expanded: String = String::new();

        for (index, item) in replacement.iter().enumerate() {
            let rendered = find_placeholder(block, variable, item);
            let rendered = normalize_loop_output(rendered);

            expanded.push_str(&rendered);

            if index < replacement.len() - 1 {
                expanded.push('\n');
            }
        }

        result.replace_range(for_start..full_block_end, &expanded);
    }

    result
}

/// Finds the patterns for a loop especifically for the list of fields on an already readen
/// file and changes it to the replacement.
/// ### Examples
/// ```rust
/// find_field_loop_placeholder(&template_content, fields) /* fields: &[Field] */
/// ```
pub fn find_field_loop_placeholder(text: &str, replacement: &[Field]) -> String {
    let mut result: String = text.to_string();
    let mut search_start: usize = 0;

    while let Some(relative_for_start) = result[search_start..].find("{{ for ") {
        let for_start = search_start + relative_for_start;
        let relative_header_end: usize = result[for_start..].find("}}").unwrap();
        let header_end: usize = for_start + relative_header_end;
        let header: &str = &result[for_start..header_end];

        let content = header.strip_prefix("{{ for ").unwrap();
        let mut parts: str::SplitWhitespace<'_> = content.split_whitespace();

        parts.next().unwrap();
        parts.next().unwrap(); // skips "in"
        let collection = parts.next().unwrap();

        let endfor = format!("{{{{ endfor {} }}}}", collection);

        let Some(relative_loop_end) = result[header_end..].find(&endfor) else {
            return result;
        };
        let loop_end = header_end + relative_loop_end;
        let full_block_end: usize = loop_end + endfor.len();

        if collection != "fields" {
            search_start = full_block_end;
            continue;
        }

        let block: &str = &result[header_end + 2..loop_end].to_string();
        let mut expanded: String = String::new();

        for (index, field) in replacement.iter().enumerate() {
            let rendered = field.render(block);
            let rendered = normalize_loop_output(rendered);

            expanded.push_str(&rendered);

            if index < replacement.len() - 1 {
                expanded.push('\n');
            }
        }

        result.replace_range(for_start..full_block_end, &expanded);
    }

    result
}

/// Finds the patterns for a loop especifically for the list of relationships on an already readen
/// file and changes it to the replacement.
/// ### Examples
/// ```rust
/// find_relationship_loop_placeholder(&template_content, relationships) /* relationships: &[Relationship] */
/// ```
pub fn find_relationship_loop_placeholder(text: &str, replacement: &[Relationship]) -> String {
    let mut result: String = text.to_string();
    let mut search_start: usize = 0;

    while let Some(relative_for_start) = result[search_start..].find("{{ for ") {
        let for_start = search_start + relative_for_start;
        let relative_header_end: usize = result[for_start..].find("}}").unwrap();
        let header_end: usize = for_start + relative_header_end;
        let header: &str = &result[for_start..header_end];

        let content = header.strip_prefix("{{ for ").unwrap();
        let mut parts: str::SplitWhitespace<'_> = content.split_whitespace();

        parts.next().unwrap();
        parts.next().unwrap(); // skips "in"
        let collection = parts.next().unwrap();

        let endfor = format!("{{{{ endfor {} }}}}", collection);

        let Some(relative_loop_end) = result[header_end..].find(&endfor) else {
            return result;
        };
        let loop_end = header_end + relative_loop_end;
        let full_block_end: usize = loop_end + endfor.len();

        if collection != "relationships" {
            search_start = full_block_end;
            continue;
        }

        let block: &str = &result[header_end + 2..loop_end].to_string();
        let mut expanded: String = String::new();

        for (index, relationship) in replacement.iter().enumerate() {
            let rendered = relationship.render(block);
            let rendered = normalize_loop_output(rendered);

            expanded.push_str(&rendered);

            if index < replacement.len() - 1 {
                expanded.push('\n');
            }
        }

        result.replace_range(for_start..full_block_end, &expanded);
    }

    result
}
