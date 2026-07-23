#![allow(unused)]

use capitalize::Capitalize;

// Custom imports
use crate::filesystem::files;

#[derive(Debug, Clone, Default)]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: DataType,
    pub primary_key: bool,
    pub auto_increment: bool,
    pub foreign_key: Option<ForeignKey>,
    pub unique: bool,
    pub allow_null: bool,
    pub default: Option<DefaultValue>,
}

#[derive(Debug, Clone)]
pub struct Relationship {
    pub parent: String,
    pub child: String,
    pub foreign_key: String,
}

#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub model: String,
    pub field: String,
}

#[derive(Debug, Clone)]
pub enum DataType {
    String,
    Integer,
    PrimaryKey,
    Float,
    Boolean,
    Date,
    CurrentDate,
}

#[derive(Debug, Clone)]
pub enum DefaultValue {
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Now,
}

impl Model {
    pub fn new(name: String) -> Self {
        let mut model: Model = Model {
            name,
            fields: Vec::new(),
        };

        model.fields.push(Field {
            name: format!("{}_id", model.name),
            data_type: DataType::PrimaryKey,
            primary_key: true,
            auto_increment: true,
            foreign_key: None,
            unique: true,
            allow_null: false,
            default: None,
        });

        model.fields.push(Field {
            name: "created_at".to_string(),
            data_type: DataType::CurrentDate,
            primary_key: false,
            auto_increment: false,
            foreign_key: None,
            unique: false,
            allow_null: false,
            default: Some(DefaultValue::Now),
        });

        model
    }
}

impl DataType {
    pub fn as_typescript(&self) -> &'static str {
        match self {
            DataType::String => "string",
            DataType::Integer => "number",
            DataType::Float => "number",
            DataType::Boolean => "boolean",
            DataType::Date => "Date",
            DataType::PrimaryKey => "CreationOptional<number>",
            DataType::CurrentDate => "CreationOptional<Date>",
        }
    }

    pub fn as_sequelize(&self) -> &'static str {
        match self {
            DataType::String => "STRING",
            DataType::Integer | DataType::PrimaryKey => "INTEGER",
            DataType::Float => "FLOAT",
            DataType::Boolean => "BOOLEAN",
            DataType::Date | DataType::CurrentDate => "DATE",
        }
    }
}

impl DefaultValue {
    pub fn as_sequelize(&self) -> String {
        match self {
            DefaultValue::String(value) => format!("\"{}\"", value),
            DefaultValue::Integer(value) => value.to_string(),
            DefaultValue::Float(value) => value.to_string(),
            DefaultValue::Boolean(value) => value.to_string(),
            DefaultValue::Now => "DataTypes.NOW".to_string(),
        }
    }
}

impl Field {
    pub fn new(name: String, data_type: DataType, unique: bool, allow_null: bool) -> Field {
        Field {
            name,
            data_type,
            primary_key: false,
            auto_increment: false,
            foreign_key: None,
            unique,
            allow_null,
            default: None,
        }
    }

    pub fn foreign(source: String, destination: String) -> Field {
        Field {
            name: format!("{}_id", destination),
            data_type: DataType::Integer,
            primary_key: false,
            auto_increment: false,
            foreign_key: Some(ForeignKey {
                model: destination.to_owned(),
                field: format!("{}_id", destination),
            }),
            unique: false,
            allow_null: false,
            default: None,
        }
    }

    pub fn render(&self, template: &str) -> String {
        let template_content: String = template.to_string();

        let mut formatted_content: String =
            files::find_placeholder(&template_content, "field.name", &self.name);

        formatted_content = files::find_placeholder(
            &formatted_content,
            "field.ts_data_type",
            self.data_type.as_typescript(),
        );

        formatted_content = files::find_placeholder(
            &formatted_content,
            "field.sq_data_type",
            self.data_type.as_sequelize(),
        );

        formatted_content = files::find_placeholder(
            &formatted_content,
            "field.primary_key",
            &self.primary_key.to_string(),
        );

        formatted_content = files::find_placeholder(
            &formatted_content,
            "field.auto_increment",
            &self.auto_increment.to_string(),
        );

        formatted_content = files::find_placeholder(
            &formatted_content,
            "field.allow_null",
            &self.allow_null.to_string(),
        );

        formatted_content =
            files::find_placeholder(&formatted_content, "field.unique", &self.unique.to_string());

        let default = match &self.default {
            Some(value) => value.as_sequelize(),
            None => String::from("undefined"),
        };

        formatted_content = files::find_placeholder(&formatted_content, "field.default", &default);

        formatted_content
    }
}

impl Relationship {
    pub fn render(&self, template: &str) -> String {
        let template_content: String = template.to_string();

        let mut formatted_content: String = files::find_placeholder(
            &template_content,
            "relationship.parent",
            &self.parent.capitalize(),
        );

        formatted_content = files::find_placeholder(
            &formatted_content,
            "relationship.child",
            &self.child.capitalize(),
        );

        formatted_content = files::find_placeholder(
            &formatted_content,
            "relationship.foreign_key",
            &self.foreign_key,
        );

        formatted_content
    }
}
