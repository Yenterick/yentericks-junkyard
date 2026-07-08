#[derive(PartialEq, Eq, Hash)]
pub enum Page {
    TemplateSelection,
    Models,
    Fields,
    ProjectConfiguration,
    Generation,
}

impl Page {
    pub fn to_string(&self) -> String {
        match self {
            Page::TemplateSelection => "Template Selection".to_owned(),
            Page::Models => "Models".to_owned(),
            Page::Fields => "Fields".to_owned(),
            Page::ProjectConfiguration => "Project Configuration".to_owned(),
            Page::Generation => "Generation".to_owned(),
        }
    }
}
