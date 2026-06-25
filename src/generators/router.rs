use capitalize::Capitalize;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

// Custom imports
use crate::filesystem::files;

/// Creates the routers files on the desired path
/// ### Created File
/// ```typescript
/// import { Router } from "express";
///
/// Module imports
/// import {
///     select{{ controllerCapitalized }}s,
///     select{{ controllerCapitalized }}byId,
///     update{{ controllerCapitalized }},
///     delete{{ controllerCapitalized }},
/// } from "../controller/{{ controller }}.ts";
///
/// const router: Router = Router();
///
/// router.get('/', select{{ controllerCapitalized }}s);
/// router.get("/:id", select{{ controllerCapitalized }}byId);
/// router.put("/:id", update{{ controllerCapitalized }});
/// router.delete("/:id", delete{{ controllerCapitalized }});
///
/// export default router;
/// ```
/// ### Examples
/// ```rust
/// create_routers_files("./example-project", controllers) /* controllers: Vec<&str> */
/// ```
pub fn create_routers_files(path: &str, controllers: Vec<&str>) -> Result<(), io::Error> {
    let router_template_path: &Path = Path::new("templates/express-sequelize/router.txt");
    let template_content: String = fs::read_to_string(router_template_path)?;

    for controller in controllers {
        let mut formatted_content: String =
            files::find_placeholder(&template_content, "controller", controller);
        formatted_content = files::find_placeholder(
            &formatted_content,
            "controllerCapitalized",
            &controller.capitalize(),
        );

        let file_path: PathBuf = PathBuf::from(path)
            .join("server")
            .join("routes")
            .join(format!("{}.ts", controller));
        files::create_file(&formatted_content, file_path)?;
    }

    Ok(())
}
