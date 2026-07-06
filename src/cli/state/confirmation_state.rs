use crate::cli::events::confirmation_choice::ConfirmationChoice;

pub struct ConfirmationState {
    pub choice: Option<ConfirmationChoice>,
}

impl ConfirmationState {
    pub fn new() -> ConfirmationState {
        ConfirmationState { choice: None }
    }
}
