use std::{fs, io, path::{Path, PathBuf}};

// Custom imports
use crate::filesystem::files;

/// Creates the package.json file on the desired path.
/// ### Created File
/// ```json
/// {
///    "name": "{{ app_name }}",
///    "version": "1.0.0",
///    "description": "{{ app_name }} - Server",
///    "license": "ISC",
///    "author": "{{ author_name }} <{{ author_email }}>",
///    "type": "module",
///    "main": "app.ts",
///    "scripts": {
///        "dev": "tsx --watch server.ts",
///        "start": "node dist/server.js",
///        "build": "tsc",
///    },
///    "devDependencies": {
///        "@types/bcrypt": "^6.0.0",
///        "@types/cors": "^2.8.19",
///        "@types/express": "^5.0.6",
///        "@types/jsonwebtoken": "^9.0.10",
///        "@types/node": "^25.6.0",
///        "tsx": "^4.21.0",
///    },
///    "dependencies": {
///        "bcrypt": "^6.0.0",
///        "cors": "^2.8.6",
///        "dotenv": "^17.4.2",
///        "express": "^5.2.1",
///        "jsonwebtoken": "^9.0.3",
///        "@sequelize/postgres": "^7.0.0-alpha.48",
///        "sequelize": "^6.37.8",
///        "typescript": "^6.0.3"
///    }
/// }
/// ```
/// ### Examples
/// ```rust
/// create_package_file("app_name", "author", "author@gmail.com", "./example-proyect");
/// ```
pub fn create_package_file(app_name: &str, author_name:&str, author_email: &str, path: &str) -> Result<(), io::Error> {
    let package_template_path: &Path = Path::new("templates/express-sequelize/package.txt");
    let template_content: String = fs::read_to_string(package_template_path)?;
    let mut formatted_content: String = files::find_placeholder(&template_content, "app_name", app_name);
    formatted_content = files::find_placeholder(&formatted_content, "author_name", author_name);
    formatted_content = files::find_placeholder(&formatted_content, "author_email", author_email);

    let file_path: PathBuf = PathBuf::from(path).join("server").join("package.json");
    
    files::create_file(&formatted_content, file_path)?;

    Ok(())
}

/// Create the .env file on the desired path.
/// ### Created File
/// ```env
/// POSTGRES_DB=
/// POSTGRES_USER=
/// POSTGRES_PASSWORD=
/// POSTGRES_HOST=
/// POSTGRES_PORT=
/// ```
/// ### Examples
/// ```rust
/// create_env_file("./example-proyect")
/// ```
pub fn create_env_file(path: &str) -> Result<(), io::Error> {
    let package_template_path: &Path = Path::new("templates/express-sequelize/env.txt");
    let template_content: String = fs::read_to_string(package_template_path)?;

    let file_path: PathBuf = PathBuf::from(path).join(".env");
    
    files::create_file(&template_content, file_path)?;

    Ok(())
}