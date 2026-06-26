use std::env;

// Custom imports
use crate::generators::core;

// Module config
mod filesystem;
mod generators;
mod models;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name: &String = &args[1];
    let path: &String = &args[2];
    let author: &str = "Yenterick";
    let author_email = "yenterick@gmail.com";

    println!("Creating the package {name}...");
    println!("Working on directory {path}");

    match core::generate_project(name, author, author_email, path) {
        Ok(()) => println!("The project was successfully created!"),
        Err(error) => println!("There was an error creating the project... {error}"),
    };
}
