use capitalize::Capitalize;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

// Custom imports
use crate::{filesystem::files, models::model::Model};

/// Creates the controllers files on the desired path
/// ### Created File
/// ```typescript
/// // Module imports
/// import { {{ model }}Model } from "../models/{{ model }}Model.js";
///
/// // Type imports
/// import type { Request, Response } from "express";
///
/// export const select{{ modelCapitalized }}s = async (
///     req: Request,
///     res: Response
/// ): Promise<Response> => {
///     try {
///         const {{ model }}s = await {{ model }}Model.select{{ modelCapitalized }}s();
///         return res.status(200).json({ success: true, data: {{ model }}s });
///     } catch (error: any) {
///         return res.status(500).json({
///             success: false,
///             message:
///                 error.message || "An error ocurred while fetching {{ modelCapitalized }}s"
///         })
///     }
/// }
///
/// export const select{{ modelCapitalized }}ById = async (
///     req: Request,
///     res: Response
/// ): Promise<Response> => {
///     try {
///         const { id } = req.params;
///         const {{ model }} = await {{ model }}Model.select{{ modelCapitalized }}ById(Number(id));
///         if (!{{ model }})
///             return res
///                 .status(404)
///                 .json({ success: false, message: '{{ modelCapitalized }} not found.' });
///         return res.status(200).json({ success: true, data: {{ model }} });
///     } catch (error: any) {
///         return res.status(500).json({
///             success: false,
///             message:
///                 error.message || 'An error occurred while fetching {{ model }}.',
///         });
///     }
/// };
///
/// export const delete{{ modelCapitalized }} = async (
///     req: Request,
///     res: Response
/// ): Promise<Response> => {
///     try {
///         const { id } = req.params;
///         await {{ model }}Model.delete{{ modelCapitalized }}(Number(id));
///         return res.status(200).json({
///             success: true,
///             message: '{{ modelCapitalized }} deleted successfully.',
///         });
///     } catch (error: any) {
///         return res.status(500).json({
///             success: false,
///             message:
///                 error.message || 'An error occurred while deleting {{ model }}.',
///         });
///     }
/// }
/// ```
/// ### Examples
/// ```rust
/// create_controllers_file("./example-project", models) /* models: &[Model] */
/// ```
pub fn create_controllers_file(path: &str, models: &[Model]) -> Result<(), io::Error> {
    let controller_template_path: &Path = Path::new("templates/express-sequelize/controller.txt");
    let template_content: String = fs::read_to_string(controller_template_path)?;

    /* TODO: Implement updateModel */
    for model in models {
        let mut formatted_content: String =
            files::find_placeholder(&template_content, "model", &model.name);
        formatted_content = files::find_placeholder(
            &formatted_content,
            "modelCapitalized",
            &model.name.capitalize(),
        );

        let file_path: PathBuf = PathBuf::from(path)
            .join("server")
            .join("controllers")
            .join(format!("{}Controller.ts", &model.name));
        files::create_file(&formatted_content, file_path)?;
    }

    Ok(())
}
