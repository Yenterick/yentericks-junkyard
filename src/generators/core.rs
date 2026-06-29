// Custom imports
use crate::filesystem::directories;
use crate::generators::{config, controller, model, project, router, utils};

use crate::models::model::*;

pub fn generate_project(
    app_name: &str,
    author_name: &str,
    author_email: &str,
    path: &str,
) -> Result<(), std::io::Error> {
    let models: Vec<Model> = vec![
        Model {
            name: "user".to_string(),
            fields: vec![
                Field {
                    name: "user_id".to_string(),
                    data_type: DataType::PrimaryKey,
                    primary_key: true,
                    auto_increment: true,
                    foreign_key: None,
                    unique: true,
                    allow_null: false,
                    default: None,
                },
                Field {
                    name: "username".to_string(),
                    data_type: DataType::String,
                    primary_key: false,
                    auto_increment: false,
                    foreign_key: None,
                    unique: true,
                    allow_null: false,
                    default: None,
                },
                Field {
                    name: "email".to_string(),
                    data_type: DataType::String,
                    primary_key: false,
                    auto_increment: false,
                    foreign_key: None,
                    unique: true,
                    allow_null: false,
                    default: None,
                },
                Field {
                    name: "is_admin".to_string(),
                    data_type: DataType::Boolean,
                    primary_key: false,
                    auto_increment: false,
                    foreign_key: None,
                    unique: false,
                    allow_null: false,
                    default: Some(DefaultValue::Boolean(false)),
                },
                Field {
                    name: "created_at".to_string(),
                    data_type: DataType::CurrentDate,
                    primary_key: false,
                    auto_increment: false,
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
                    data_type: DataType::PrimaryKey,
                    primary_key: true,
                    auto_increment: true,
                    foreign_key: None,
                    unique: true,
                    allow_null: false,
                    default: None,
                },
                Field {
                    name: "user_id".to_string(),
                    data_type: DataType::Integer,
                    primary_key: false,
                    auto_increment: false,
                    foreign_key: Some(ForeignKey {
                        model: "user".to_string(),
                        field: "user_id".to_string(),
                    }),
                    unique: false,
                    allow_null: false,
                    default: None,
                },
                Field {
                    name: "name".to_string(),
                    data_type: DataType::String,
                    primary_key: false,
                    auto_increment: false,
                    foreign_key: None,
                    unique: false,
                    allow_null: false,
                    default: None,
                },
                Field {
                    name: "price".to_string(),
                    data_type: DataType::Float,
                    primary_key: false,
                    auto_increment: false,
                    foreign_key: None,
                    unique: false,
                    allow_null: false,
                    default: Some(DefaultValue::Float(0.0)),
                },
                Field {
                    name: "stock".to_string(),
                    data_type: DataType::Integer,
                    primary_key: false,
                    auto_increment: false,
                    foreign_key: None,
                    unique: false,
                    allow_null: false,
                    default: Some(DefaultValue::Integer(0)),
                },
                Field {
                    name: "created_at".to_string(),
                    data_type: DataType::CurrentDate,
                    primary_key: false,
                    auto_increment: false,
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

    match model::create_models_file(path, &models) {
        Ok(()) => println!("Models files successfully created!"),
        Err(error) => return Err(error),
    }

    match model::create_index_file(path, &models) {
        Ok(()) => println!("Index file successfully created!"),
        Err(error) => return Err(error),
    }

    match controller::create_controllers_file(path, &models) {
        Ok(()) => println!("Controllers files successfully created!"),
        Err(error) => return Err(error),
    }

    match router::create_routers_files(path, &models) {
        Ok(()) => println!("Routers files successfully created!"),
        Err(error) => return Err(error),
    }

    Ok(())
}
