use std::{
    io,
    path::{Path, PathBuf},
};

// Custom imports
use crate::filesystem::files::{self, read_template};

/// Creates the utils file on the desired path.
/// ### Created File
/// ```typescript
/// // Gets a prefix with the actual time to log info
/// export const getLogPrefix = () => {
///     const now = new Date();
///     const timeStamp = now.toLocaleString();
///     return `[{{ app_name }}-server@1.0.0 | ${timeStamp}] - `;
/// };
///
/// // Logs info in the terminal
/// export const log = (message: string) => {
///     console.log(`${getLogPrefix()}${message}`);
/// };
///
/// // Simulates time.sleep() from Python
/// export const sleep = (ms: number) => {
///     return new Promise((resolve) => setTimeout(resolve, ms));
/// };
/// ```
/// ### Examples
/// ```rust
/// create_utils_file("./example-project", "app_name");
/// ```
pub fn create_utils_file(path: &str, app_name: &str) -> Result<(), io::Error> {
    let utils_template_path: &Path = Path::new("templates/express-sequelize/utils.txt");
    let template_content: String = read_template(utils_template_path)?;
    let formatted_content: String =
        files::find_placeholder(&template_content, "app_name", app_name);

    let file_path: PathBuf = PathBuf::from(path)
        .join("server")
        .join("utils")
        .join("utils.ts");

    files::create_file(&formatted_content, file_path)?;

    Ok(())
}
