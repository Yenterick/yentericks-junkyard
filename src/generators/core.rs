// Custom imports
use crate::filesystem::directories;
use crate::generators::utils;
use crate::generators::project;
use crate::generators::config;

pub fn generate_project(app_name: &str, author_name: &str, author_email: &str, path: &str) -> Result<(), std::io::Error> {
    let routes: Vec<String> = vec![String::from("users"), String::from("products")];

    match project::create_env_file(path) {
        Ok(()) => println!("Env file successfully created!"),
        Err(error) => return Err(error)
    }
    
    match directories::create_folder_structure(path) {
        Ok(()) => println!("Project folder structure successfully created!"),
        Err(error) => return Err(error)
    };

    match project::create_package_file(app_name, author_name, author_email, path) {
        Ok(()) => println!("Package file successfully created!"),
        Err(error) => return Err(error)
    };

    match project::create_tsconfig_file(path) {
        Ok(()) => println!("Tsconfig file successfully created!"),
        Err(error) => return Err(error)
    }

    match utils::create_utils_file(app_name, path) {
        Ok(()) => println!("Utils file successfully created!"),
        Err(error) => return Err(error)
    };

    match config::create_database_file(path) {
        Ok(()) => println!("Database config file successfully created!"),
        Err(error) => return Err(error)
    }

    match config::create_dotenv_file(path) {
        Ok(()) => println!("Dotenv config file successfully created!"),
        Err(error) => return Err(error)
    }

    match project::create_app_file(path, routes) {
        Ok(()) => println!("App config file successfully created!"),
        Err(error) => return Err(error)
    }

    Ok(())
}
