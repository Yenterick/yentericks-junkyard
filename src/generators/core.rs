// Custom imports
use crate::filesystem::directories;
use crate::generators::utils;

pub fn generate_project(app_name: &str, path: &str) -> Result<(), std::io::Error> {
    match directories::create_folder_structure(path) {
        Ok(()) => println!("Project folder structure successfully created!"),
        Err(error) => return Err(error)
    };

    match utils::create_utils_file(app_name, path) {
        Ok(()) => println!("Utils file successfully created!"),
        Err(error) => return Err(error)
    };

    Ok(())
}
