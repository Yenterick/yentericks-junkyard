use crate::cli::events::pages::Pages;

pub enum ScreenAction {
    None,
    Confirm,
    NextPage(Pages),
    PreviousPage(Pages),
    OpenError(String),
    TextInput(String),
    ReturnInput(String),
    Back,
    Exit,
}
