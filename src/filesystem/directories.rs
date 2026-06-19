use std::{fs, io, path::PathBuf};

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
    let mut server_path = PathBuf::from(path);
    server_path.push("server");
    fs::create_dir_all(&server_path)?;

    let subfolders: [&str; 6] = [
        "config",
        "controllers",
        "routes",
        "middlewares",
        "models",
        "utils",
    ];

    for folder in subfolders {
        let mut subfolder_path = server_path.clone();
        subfolder_path.push(folder);
        fs::create_dir(&subfolder_path)?;
    }

    Ok(())
}
