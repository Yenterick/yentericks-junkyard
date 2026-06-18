use std::env;

// Custom imports
use crate::filesystem::directories;

// Module config
mod filesystem;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name: &String = &args[1];
    let path: &String = &args[2];

    println!("Creating the package {name}...");
    println!("Working on directory {path}");
    
    match directories::create_folder_structure(path) {
        Ok(()) => println!("The folder structure was successfully created!"),
        Err(error) => println!("The folder structure wasn't created due to an error... {error:?}")
    };
}