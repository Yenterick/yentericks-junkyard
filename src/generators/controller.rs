use capitalize::Capitalize;
use std::{
    io,
    path::{Path, PathBuf},
};

// Custom imports
use crate::{
    filesystem::files::{self, read_template},
    models::model::{Field, Model},
};

/// Creates the controllers files on the desired path.
/// ### Created File
/// ```typescript
/// // Module imports
/// import { {{ model }}Model } from "../models/{{ model }}Model.js";
///
/// // Type imports
/// import type { Request, Response } from "express";
///
/// export const select{{ capitalizedModel }}s = async (
///     req: Request,
///     res: Response
/// ): Promise<Response> => {
///     try {
///         const {{ model }}s = await {{ model }}Model.select{{ capitalizedModel }}s();
///         return res.status(200).json({ success: true, data: {{ model }}s });
///     } catch (error: any) {
///         return res.status(500).json({
///             success: false,
///             message:
///                 error.message || "An error ocurred while fetching {{ capitalizedModel }}s"
///         })
///     }
/// }
///
/// export const select{{ capitalizedModel }}ById = async (
///     req: Request,
///     res: Response
/// ): Promise<Response> => {
///     try {
///         const { id } = req.params;
///         const {{ model }} = await {{ model }}Model.select{{ capitalizedModel }}ById(Number(id));
///         if (!{{ model }})
///             return res
///                 .status(404)
///                 .json({ success: false, message: '{{ capitalizedModel }} not found.' });
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
/// export const update{{ capitalizedModel }} = async (
///     req: Request,
///     res: Response
/// ): Promise<Response> => {
///     try {
///         const { id } = req.params;
///         const {
///             {{ for field in fields }}
///             {{ field }}
///             {{ endfor fields }}
///         } = req.body;
///
///         const updateData: any = {}'
///         {{ for field in fields }}
///         if ({{ field }}) updateData.{{ field }} = {{ field }};
///         {{ endfor }}
///
///         await {{ model }}Model.update{{ capitalizedModel }}ById(Number(id), updateData);
///
///         return res.status(200).json({
///             success: true,
///             message: '{{ capitalizedModel }} updated successfully.',
///         });
///     } catch (error: any) {
///         return res.status(500).json({
///             success: false,
///             message:
///                 error.message || 'An error occurred while updating {{ model }}.',
///         });
///     }
/// };
///
/// export const delete{{ capitalizedModel }} = async (
///     req: Request,
///     res: Response
/// ): Promise<Response> => {
///     try {
///         const { id } = req.params;
///         await {{ model }}Model.delete{{ capitalizedModel }}(Number(id));
///         return res.status(200).json({
///             success: true,
///             message: '{{ capitalizedModel }} deleted successfully.',
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
    let template_content: String = read_template(controller_template_path)?;

    for model in models {
        let mut formatted_content: String =
            files::find_placeholder(&template_content, "model", &model.name);
        formatted_content = files::find_placeholder(
            &formatted_content,
            "capitalizedModel",
            &model.name.capitalize(),
        );
        formatted_content = files::find_loop_placeholder(
            &formatted_content,
            "fields",
            model
                .fields
                .iter()
                .filter(|field: &&Field| {
                    ![format!("{}_id", model.name), String::from("created_at")]
                        .contains(&field.name)
                })
                .map(|field: &Field| field.name.as_str())
                .collect(),
        );

        let file_path: PathBuf = PathBuf::from(path)
            .join("server")
            .join("controllers")
            .join(format!("{}Controller.ts", &model.name));
        files::create_file(&formatted_content, file_path)?;
    }

    Ok(())
}
