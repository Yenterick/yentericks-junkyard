use std::env;

use crate::filesystem::directories;

// Module imports
mod filesystem;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name: &String = &args[1];
    let path: &String = &args[2];


    println!("Creating the package {name}...");
    println!("Working on directory {path}");
    
    directories::create_folder_structure(path);
}