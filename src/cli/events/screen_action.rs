use crate::{cli::events::pages::Pages, models::model::Field};

pub enum ScreenAction {
    None,
    Confirm,
    NextPage(Pages),
    PreviousPage(Pages),
    OpenError(String),
    TextInput(String),
    ReturnInput(String),
    ReturnField(Field),
    Back,
    Exit,
}
