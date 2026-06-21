// Custom imports
use crate::filesystem::directories;
use crate::generators::utils;
use crate::generators::project;

pub fn generate_project(app_name: &str, author_name: &str, author_email: &str, path: &str) -> Result<(), std::io::Error> {
    match directories::create_folder_structure(path) {
        Ok(()) => println!("Project folder structure successfully created!"),
        Err(error) => return Err(error)
    };

    match utils::create_utils_file(app_name, path) {
        Ok(()) => println!("Utils file successfully created!"),
        Err(error) => return Err(error)
    };

    match project::create_package_file(app_name, author_name, author_email, path) {
        Ok(()) => println!("Package file successfully created!"),
        Err(error) => return Err(error)
    }

    Ok(())
}
