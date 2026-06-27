use capitalize::Capitalize;
use std::{
    io,
    path::{Path, PathBuf},
};

// Custom imports
use crate::{
    filesystem::files::{self, find_placeholder, read_template},
    models::model::Model,
};

/// Creates the models files on the desired path.
/// ### Created File
/// ```typescript
/// import {
///     DataTypes,
///     Model,
///     type InferAttributes,
///     type InferCreationAttributes,
///     type CreationOptional,
/// } from "sequelize";
///
/// // Module imports
/// import { sequlize } from "../config/database.js";
///
/// // Defines the {{ model }} model
/// export class {{ capitalizedModel }} extends Model <
///     InferAttributes<{{ capitalizedModel }}>
///     InferCreationAttributes<{{ capitalizedModel }}>
/// > {
/// {{ for field in fields }}
///     declare {{ field.name }}: {{ field.ts_data_type }}
/// {{ endfor fields }}
/// }
///
/// {{ capitalizedModel }}.init(
///     {
///         {{ for field in fields }}
///             {{ field.name }}: {
///                 type: DataTypes.{{ field.sq_data_type }},
///                 primaryKey: {{ field.primary_key }},
///                 autoIncrement: {{ field.auto_increment }},
///                 allow_null: {{ field.allow_null }},
///                 unique: {{ field.unique }},
///                 defaultValue: {{ field.default }}
///             }
///         {{ endfor fields }}
///     },
///     {
///         {
///             sequelize,
///             tableName: "{{ model }}",
///             timestamps: false,
///         }
///     }
/// );
///
/// // {{ capitalizedModel }} model with all the required functions
/// export const {{ model }}Model = {
///     select{{ capitalizedModel }}s: async (): Promise<{{ capitalizedModel }}[]> => {
///         return await {{ modelCapitalized }}.findAll();
///     },
///
///     select{{ capitelizedModel }}ById: async ({{ model }}_id: number): Promise<{{ capitalizedModel }} | null> => {
///         return await {{ capitalizedModel }}.findByPk({{ model }}_id);
///     },
///
///     insert{{ capitalizedModel }}: async (data: any): Promise<void> => {
///         await {{ capitalizedModel }}.create(data);
///     },
///
///     update{{ capitalizedModel }}ById: async ({{ model }}_id: number, data: any): Promise<void> => {
///         await {{ capitalizedModel }}.update(data, {
///             where: {
///                 {{ model }}_id,
///             },
///         });
///     },
///
///     delete{{ capitalizedModel }}: async ({{ model }}_id: number): Promise<void> => {
///         await {{ capitalizedModel }}.destroy({
///             where: {
///                 {{ model }}_id,
///             },
///         });
///     },
/// }
/// ```
/// ### Examples
/// ```rust
/// create_models_file("./example-project", models); /* models: &[Model] */
/// ```
pub fn create_models_file(path: &str, models: &[Model]) -> Result<(), io::Error> {
    let model_template_path: &Path = Path::new("templates/express-sequelize/model.txt");
    let template_content: String = read_template(model_template_path)?;

    for model in models {
        let mut formatted_content: String =
            files::find_field_loop_placeholder(&template_content, &model.fields);

        formatted_content = find_placeholder(&formatted_content, "model", &model.name);
        formatted_content = find_placeholder(
            &formatted_content,
            "capitalizedModel",
            &model.name.capitalize(),
        );

        let file_path: PathBuf = PathBuf::from(path)
            .join("server")
            .join("models")
            .join(format!("{}Model.ts", &model.name));

        files::create_file(&formatted_content, file_path)?;
    }

    Ok(())
}
