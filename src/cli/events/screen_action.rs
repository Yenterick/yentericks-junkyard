use crate::cli::events::pages::Pages;

pub enum ScreenAction {
    None,
    Confirm,
    NextPage(Pages),
    PreviousPage(Pages),
    OpenConfirmation(String),
    Exit,
}
