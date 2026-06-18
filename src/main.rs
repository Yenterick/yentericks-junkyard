use std::env;

// Module imports
mod filesystem;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name: &String = &args[1];
    let file_path: &String = &args[2];


    println!("Creating the package {name}...");
    println!("Working on directory {file_path}");
    filesystem::directories::mod_test();
}