use color_eyre::eyre::{Ok, Result};
use std::env;

// Custom imports
use crate::view::app;

// Module config
mod filesystem;
mod generators;
mod models;
mod view;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Getting the path arg
    let app_path: Option<String> = env::args().nth(1);

    // Creating the ratatui terminal
    let terminal = ratatui::init();
    let result = app::run(terminal, &app_path.unwrap_or(String::from(".")));
    ratatui::restore();

    result
}
