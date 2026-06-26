// Custom imports
use crate::filesystem::directories;
use crate::generators::config;
use crate::generators::controller;
use crate::generators::project;
use crate::generators::router;
use crate::generators::utils;

use crate::models::model::*;

pub fn generate_project(
    app_name: &str,
    author_name: &str,
    author_email: &str,
    path: &str,
) -> Result<(), std::io::Error> {
    let models = vec![
        Model {
            name: "user".to_string(),
            fields: vec![
                Field {
                    name: "user_id".to_string(),
                    data_type: DataType::Integer,
                    primary_key: true,
                    foreign_key: None,
                    unique: true,
                    allow_null: false,
                    default: None,
                },
                Field {
                    name: "username".to_string(),
                    data_type: DataType::String,
                    primary_key: false,
                    foreign_key: None,
                    unique: true,
                    allow_null: false,
                    default: None,
                },
                Field {
                    name: "email".to_string(),
                    data_type: DataType::String,
                    primary_key: false,
                    foreign_key: None,
                    unique: true,
                    allow_null: false,
                    default: None,
                },
                Field {
                    name: "is_admin".to_string(),
                    data_type: DataType::Boolean,
                    primary_key: false,
                    foreign_key: None,
                    unique: false,
                    allow_null: false,
                    default: Some(DefaultValue::Boolean(false)),
                },
                Field {
                    name: "created_at".to_string(),
                    data_type: DataType::DateTime,
                    primary_key: false,
                    foreign_key: None,
                    unique: false,
                    allow_null: false,
                    default: Some(DefaultValue::Now),
                },
            ],
        },
        Model {
            name: "product".to_string(),
            fields: vec![
                Field {
                    name: "product_id".to_string(),
                    data_type: DataType::Integer,
                    primary_key: true,
                    foreign_key: None,
                    unique: true,
                    allow_null: false,
                    default: None,
                },
                Field {
                    name: "name".to_string(),
                    data_type: DataType::String,
                    primary_key: false,
                    foreign_key: None,
                    unique: false,
                    allow_null: false,
                    default: None,
                },
                Field {
                    name: "price".to_string(),
                    data_type: DataType::Float,
                    primary_key: false,
                    foreign_key: None,
                    unique: false,
                    allow_null: false,
                    default: Some(DefaultValue::Float(0.0)),
                },
                Field {
                    name: "stock".to_string(),
                    data_type: DataType::Integer,
                    primary_key: false,
                    foreign_key: None,
                    unique: false,
                    allow_null: false,
                    default: Some(DefaultValue::Integer(0)),
                },
                Field {
                    name: "created_at".to_string(),
                    data_type: DataType::DateTime,
                    primary_key: false,
                    foreign_key: None,
                    unique: false,
                    allow_null: false,
                    default: Some(DefaultValue::Now),
                },
            ],
        },
    ];

    match project::create_env_file(path) {
        Ok(()) => println!("Env file successfully created!"),
        Err(error) => return Err(error),
    }

    match directories::create_folder_structure(path) {
        Ok(()) => println!("Project folder structure successfully created!"),
        Err(error) => return Err(error),
    };

    match project::create_package_file(path, app_name, author_name, author_email) {
        Ok(()) => println!("Package file successfully created!"),
        Err(error) => return Err(error),
    };

    match project::create_tsconfig_file(path) {
        Ok(()) => println!("Tsconfig file successfully created!"),
        Err(error) => return Err(error),
    }

    match utils::create_utils_file(path, app_name) {
        Ok(()) => println!("Utils file successfully created!"),
        Err(error) => return Err(error),
    };

    match config::create_database_file(path) {
        Ok(()) => println!("Database config file successfully created!"),
        Err(error) => return Err(error),
    }

    match config::create_dotenv_file(path) {
        Ok(()) => println!("Dotenv config file successfully created!"),
        Err(error) => return Err(error),
    }

    match project::create_app_file(path, app_name, &models) {
        Ok(()) => println!("App config file successfully created!"),
        Err(error) => return Err(error),
    }

    match controller::create_controllers_file(path, &models) {
        Ok(()) => println!("Controllers successfully created!"),
        Err(error) => return Err(error),
    }

    match router::create_routers_files(path, &models) {
        Ok(()) => println!("Routers successfully created!"),
        Err(error) => return Err(error),
    }

    Ok(())
}
