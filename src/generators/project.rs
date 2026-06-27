use std::{
    io,
    path::{Path, PathBuf},
};

// Custom imports
use crate::{
    filesystem::files::{self, read_template},
    models::model::Model,
};

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
/// create_package_file("./example-project", "app_name", "author", "author@gmail.com");
/// ```
pub fn create_package_file(
    path: &str,
    app_name: &str,
    author_name: &str,
    author_email: &str,
) -> Result<(), io::Error> {
    let package_template_path: &Path = Path::new("templates/express-sequelize/package.txt");
    let template_content: String = read_template(package_template_path)?;
    let mut formatted_content: String =
        files::find_placeholder(&template_content, "app_name", app_name);
    formatted_content = files::find_placeholder(&formatted_content, "author_name", author_name);
    formatted_content = files::find_placeholder(&formatted_content, "author_email", author_email);

    let file_path: PathBuf = PathBuf::from(path).join("server").join("package.json");

    files::create_file(&formatted_content, file_path)?;

    Ok(())
}

/// Creates the tsconfig.json file on the desired path.
/// ### Created File
/// ```json
/// {
///     // Visit https://aka.ms/tsconfig to read more about this file
///     "compilerOptions": {
///         // File Layout
///         // "rootDir": "./src",
///         // "outDir": "./dist",
///
///         // Environment Settings
///         // See also https://aka.ms/tsconfig/module
///         "module": "es2022",
///         "target": "esnext",
///         "types": [],
///         "outDir": "./dist",
///         "rootDir": "./",
///         // For nodejs:
///         // "lib": ["esnext"],
///         // "types": ["node"],
///         // and npm install -D @types/node
///
///         // Other Outputs
///         "sourceMap": true,
///         "declaration": true,
///         "declarationMap": true,
///
///         // Stricter Typechecking Options
///         "noUncheckedIndexedAccess": true,
///         "exactOptionalPropertyTypes": true,
///
///         // Style Options
///         // "noImplicitReturns": true,
///         // "noImplicitOverride": true,
///         // "noUnusedLocals": true,
///         // "noUnusedParameters": true,
///         // "noFallthroughCasesInSwitch": true,
///         // "noPropertyAccessFromIndexSignature": true,
///
///         // Recommended Options
///         "strict": true,
///         "jsx": "react-jsx",
///         "verbatimModuleSyntax": true,
///         "isolatedModules": true,
///         "noUncheckedSideEffectImports": true,
///         "moduleDetection": "force",
///         "skipLibCheck": true,
///         "moduleResolution": "bundler"
///     },
///     "include": ["**/*.ts", "utils.txt", "config.txt"],
///     "exclude": ["dist"]
/// }
/// ```
/// ### Examples
/// ```rust
/// create_tsconfig_file("./example-project");
/// ```
pub fn create_tsconfig_file(path: &str) -> Result<(), io::Error> {
    let tsconfig_template_path: &Path = Path::new("templates/express-sequelize/tsconfig.txt");
    let template_content: String = read_template(tsconfig_template_path)?;

    let file_path: PathBuf = PathBuf::from(path).join("server").join("tsconfig.json");

    files::create_file(&template_content, file_path)?;

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
/// create_env_file("./example-project");
/// ```
pub fn create_env_file(path: &str) -> Result<(), io::Error> {
    let env_template_path: &Path = Path::new("templates/express-sequelize/env.txt");
    let template_content: String = read_template(env_template_path)?;

    let file_path: PathBuf = PathBuf::from(path).join(".env");

    files::create_file(&template_content, file_path)?;

    Ok(())
}

/// Create the app file on the desired path.
/// ### Created File
/// ```typescript
/// import "./config/dotenv.js";
/// import express from "express";
/// import cors from "cors";
///
/// // Router imports
/// {{ for router in routers }}
/// import {{ router }}Router from "./routes/{{ router }}.js";
/// {{ endfor routers }}
///
/// // Types import
/// import type { Application, Request, Response } from "Express";
///
/// // Module imports
/// import { log } from "./utils/utils.js";
/// import { dbConnection } from "./config/database.js";
///
/// // Setup database associations
/// import "./models/associations.js"
///
/// const app: Application = express();
///
/// // App configuration
/// const PORT = process.env.PORT;
/// app.use(cors({ origin: '*' }));
/// app.use(express.json());
/// app.use(express.urlencoded({ extended: true }));
///
/// // Routes configuration
/// {{ for router in routers }}
/// app.use("/api/{{ router }}", {{ router }}Router);
/// {{ endfor routers }}
///
/// app.get("/api/health", (req: Request, res: Response) => {
///     res.status(200).json({
///         success: true,
///         message: "{{ app_name }} server is up and running!",
///     });
/// });
///
/// // Start application
/// const startApplication = async (): Promise<void> => {
///     await dbConnection();
///
///     app.listen(PORT, '0.0.0.0', () => {
///         log(`{{ app_name }} server is running on port ${PORT}...`);
///     });
/// };
///
/// startApplication();
/// ```
/// ### Examples
/// ```rust
/// create_app_file("./example-project", "app_name", models); /* models: &[Model] */
/// ```
pub fn create_app_file(path: &str, app_name: &str, models: &[Model]) -> Result<(), io::Error> {
    let app_template_content: &Path = Path::new("templates/express-sequelize/app.txt");
    let template_content: String = read_template(app_template_content)?;
    let mut formatted_text: String = files::find_loop_placeholder(
        &template_content,
        "routers",
        models
            .iter()
            .map(|model: &Model| model.name.as_str())
            .collect(),
    );
    formatted_text = files::find_placeholder(&formatted_text, "app_name", app_name);

    let file_path: PathBuf = PathBuf::from(path).join("server").join("app.ts");

    files::create_file(&formatted_text, file_path)?;

    Ok(())
}
