use std::{
    io,
    path::{Path, PathBuf},
};

// Custom imports
use crate::filesystem::files::{self, read_template};

/// Creates the database config file on the desired path.
/// ### Created File
/// ```typescript
/// import { Sequelize } from '@sequelize/core';
/// import { PostgresDialect } from '@sequelize/postgres';
///
/// // Modules import
/// import { log, sleep } from '../utils/utils.js';
///
/// // Creating the PostgreSQL sequelize object
/// export const sequelize = new Sequelize({
///     dialect: PostgresDialect,
///     database: process.env.POSTGRES_DB || "",
///     user: process.env.POSTGRES_USER || "",
///     password: process.env.POSTGRES_PASSWORD || "",
///     host: process.env.POSTGRES_HOST || "localhost",
///     port: Number(process.env.POSTGRES_PORT) || 5432,
///     ssl: true
/// });
///
/// // Function to connect to PostgreSQL
/// export const dbConnection = async (): Promise<void> => {
///     log('Connecting to PostgreSQL...');
///     let connected = false;
///     while (!connected) {
///         try {
///             await sequelize.authenticate();
///             log('PostgreSQL successfully connected.');
///             connected = true;
///         } catch (error) {
///             // Loops till the database is successfully connected
///             log(
///                 `PostgreSQL got an error while trying to connect: ${error} - Retrying in 5 seconds...`
///             );
///             await sleep(5000);
///         }
///     }
/// };
/// ```
/// ### Examples
/// create_database_file("./example-proyect");
pub fn create_database_file(path: &str) -> Result<(), io::Error> {
    let database_template_path: &Path = Path::new("templates/express-sequelize/database.txt");
    let template_content: String = read_template(database_template_path)?;

    let file_path: PathBuf = PathBuf::from(path)
        .join("server")
        .join("config")
        .join("database.ts");

    files::create_file(&template_content, file_path)?;

    Ok(())
}

/// Creates the dotenv config file on the desired path.
/// ### Created File
/// ```typescript
/// import dotenv from "dotenv";
/// import path, { dirname } from "path";
/// import { fileURLToPath } from "url";
///
/// const __filename = fileURLToPath(import.meta.url);
/// const __dirname = dirname(__filename);
///
/// // Load environment variables from the root directory
/// dotenv.config({ path: path.join(__dirname, "..", "..", ".env") });
///
/// ```
/// ### Examples
/// create_dotenv_file("./example-proyect");
pub fn create_dotenv_file(path: &str) -> Result<(), io::Error> {
    let database_template_path: &Path = Path::new("templates/express-sequelize/dotenv.txt");
    let template_content: String = read_template(database_template_path)?;

    let file_path: PathBuf = PathBuf::from(path)
        .join("server")
        .join("config")
        .join("dotenv.ts");

    files::create_file(&template_content, file_path)?;

    Ok(())
}
