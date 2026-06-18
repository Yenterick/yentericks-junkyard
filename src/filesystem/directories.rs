use std::{
    path::Path,
    ffi::OsStr,
    fs,
    io
};

/// Creates the next folder structure on the desired path.
/// 
/// ```bash
/// .
/// └── server/
///     ├── config/
///     ├── controllers/
///     ├── routes/
///     ├── middlewares/
///     ├── models/
///     └── utils/
/// ```
/// ### Examples
/// ```rust
/// filesystem::directories::create_folder_structuree(".");
/// ```
pub fn create_folder_structure(path: &str) -> Result<(), io::Error> {
    let formatted_path: String = format!("{}server/", if path == '.'.to_string() { "" } else { path });
    let path: &OsStr = Path::new(&formatted_path).as_os_str();
    fs::create_dir_all(path)?;

    let subfolders: [&str; 6] = ["config", "controllers", "routes", "middlewares", "models", "utils"];

    for folder in subfolders {
        let formatted_subfolder_path: String = format!("{formatted_path}{folder}/");
        let subfolder_path: &OsStr = Path::new(&formatted_subfolder_path).as_os_str();
        fs::create_dir(subfolder_path)?;
    }

    Ok(())
}

