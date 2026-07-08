#[derive(PartialEq, Eq, Hash)]
pub enum Pages {
    TemplateSelection,
    Models,
    Fields,
    ProjectConfiguration,
    Generation,
}

impl Pages {
    pub fn to_string(&self) -> String {
        match self {
            Pages::TemplateSelection => "Template Selection".to_owned(),
            Pages::Models => "Models".to_owned(),
            Pages::Fields => "Fields".to_owned(),
            Pages::ProjectConfiguration => "Project Configuration".to_owned(),
            Pages::Generation => "Generation".to_owned(),
        }
    }
}
