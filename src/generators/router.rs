use capitalize::Capitalize;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

// Custom imports
use crate::filesystem::files;

pub fn create_routers_file(path: &str, controllers: Vec<&str>) -> Result<(), io::Error> {
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
