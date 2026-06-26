#![allow(unused)]

#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: DataType,
    pub primary_key: bool,
    pub foreign_key: Option<ForeignKey>,
    pub unique: bool,
    pub allow_null: bool,
    pub default: Option<DefaultValue>,
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
    Float,
    Boolean,
    DateTime,
}

#[derive(Debug, Clone)]
pub enum DefaultValue {
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Now,
}
