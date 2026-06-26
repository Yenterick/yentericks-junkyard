use capitalize::Capitalize;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

// Custom imports
use crate::{filesystem::files, models::model::Model};

/// Creates the routers files on the desired path
/// ### Created File
/// ```typescript
/// import { Router } from "express";
///
/// Module imports
/// import {
///     select{{ capitalizedController }}s,
///     select{{ capitalizedController }}byId,
///     update{{ capitalizedController }},
///     delete{{ capitalizedController }},
/// } from "../controller/{{ controller }}.ts";
///
/// const router: Router = Router();
///
/// router.get('/', select{{ capitalizedController }}s);
/// router.get("/:id", select{{ capitalizedController }}byId);
/// router.put("/:id", update{{ capitalizedController }});
/// router.delete("/:id", delete{{ capitalizedController }});
///
/// export default router;
/// ```
/// ### Examples
/// ```rust
/// create_routers_files("./example-project", models) /* models: &[Model] */
/// ```
pub fn create_routers_files(path: &str, models: &[Model]) -> Result<(), io::Error> {
    let router_template_path: &Path = Path::new("templates/express-sequelize/router.txt");
    let template_content: String = fs::read_to_string(router_template_path)?;

    for model in models {
        let mut formatted_content: String =
            files::find_placeholder(&template_content, "controller", &model.name);
        formatted_content = files::find_placeholder(
            &formatted_content,
            "capitalizedController",
            &model.name.capitalize(),
        );

        let file_path: PathBuf = PathBuf::from(path)
            .join("server")
            .join("routes")
            .join(format!("{}Router.ts", &model.name));
        files::create_file(&formatted_content, file_path)?;
    }

    Ok(())
}
