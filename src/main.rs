use color_eyre::eyre::Result;
use std::env;

// Custom imports
// use crate::view::app;
use crate::cli::app;

// Module config
mod cli;
mod filesystem;
mod generators;
mod models;
mod view;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Getting the path arg
    let app_path: Option<String> = env::args().nth(1);
    let app_name: Option<String> = env::args().nth(2);

    // Creating the ratatui terminal
    let terminal = ratatui::init();
    let result = app::run(terminal);
    ratatui::restore();

    result
}
